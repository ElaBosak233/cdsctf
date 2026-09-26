# CdsCTF API 与领域扩展规范

状态：提案（目标架构尚未全部实现）
适用版本：当前 `/api` 与后续 `/api/v1` 迁移
最后核对：2026-09-26

这份文档同时记录两件事：代码当前已经提供的 HTTP 契约，以及平台继续扩展时必须遵守的目标契约。带有“目标”标记的路径、字段和数据库表不能被当作当前已实现功能。

## 1. 当前实现基线

### 1.1 HTTP 入口

当前服务由 `crates/web` 组装，入口如下：

| 路径 | 当前行为 |
| --- | --- |
| `GET /healthz` | 健康检查 |
| `/docs` | Scalar 渲染的 OpenAPI 文档 |
| `/api` | 当前唯一的 HTTP API 版本 |
| 其他路径 | 交给反向代理路由 |

`/api` 的 OpenAPI 文档由各个 Axum 子路由合并生成。新增 API 必须通过 `utoipa` 路由注册，否则实现与文档会分离。

### 1.2 认证与权限

- 认证使用 `tower-sessions` 的 session cookie，当前 cookie 名为 `cds.id`。
- session 中的 `user_id` 由认证 middleware 解析为 `AuthPrincipal`。
- `Group::Banned` 在 middleware 层直接拒绝，返回 `403`。
- 当前 `/api/admin/*` 路由统一挂载 `admin_only`，只有 `Group::Admin` 或更高等级可以访问；目标设计移除这个路由前缀，将权限下沉到资源操作。
- 普通路由是否需要登录由 handler 自己检查；未登录通常返回 `401`。
- 当前没有 `game_admins` 表，也没有比赛级管理员授权检查。

因此，当前系统只有全局用户组权限；“某个用户只管理某一场 Game”目前不是代码能力。

### 1.3 当前资源与存储模型

当前 PostgreSQL schema 已有以下聚合：

| 聚合 | 主要数据 |
| --- | --- |
| `users` | 账号、全局 `group`、密码摘要、软删除时间 |
| `emails` | 用户邮箱与验证状态 |
| `idps`、`user_idps` | Lua 脚本 IdP 与用户绑定 |
| `games` | 比赛设置、时间段、暂停、黑屏和计分版本 |
| `challenges` | 题目内容、公开状态、checker、writeup、实例配置、软删除时间 |
| `game_challenges` | 题目在某场比赛中的难度、分值、启用和冻结配置；复合主键为 `(game_id, challenge_id)` |
| `teams`、`team_users` | 比赛队伍及成员关系 |
| `submissions` | 用户提交、可选队伍和比赛上下文、状态与得分 |
| `game_notices` | 比赛公告 |
| `notes` | 用户对题目的笔记；`(user_id, challenge_id)` 唯一 |
| `configs` | 单行 JSON 配置 |

实例当前由 Kubernetes Pod 和 labels/annotations 表示，尚未有实例历史表。媒体对象由 S3 兼容存储提供，公开资源通常通过 hash 或预签名 URL 获取。

### 1.4 当前 API 路由

公共资源树挂载在 `/api`：

```text
/api/
/api/configs
/api/users
/api/challenges
/api/games
/api/idps
/api/instances
/api/notes
/api/media
/api/submissions
```

典型公共接口包括：

```text
POST /api/users/login
POST /api/users/register
POST /api/users/logout
GET  /api/users/me
PUT  /api/users/me
DELETE /api/users/me
PUT  /api/users/me/password

GET  /api/challenges
GET  /api/challenges/{challenge_id}
POST /api/challenges/status
GET  /api/challenges/{challenge_id}/attachments

GET  /api/games
GET  /api/games/{game_id}
GET  /api/games/{game_id}/scoreboard
GET  /api/games/{game_id}/events
GET  /api/games/{game_id}/challenges
GET  /api/games/{game_id}/teams
GET  /api/games/{game_id}/teams/us

GET  /api/submissions
POST /api/submissions
GET  /api/instances
POST /api/instances
POST /api/instances/{instance_id}/renew
POST /api/instances/{instance_id}/stop
GET  /api/instances/{instance_id}/wsrx
```

当前管理员资源树挂载在 `/api/admin`，并由 `admin_only` middleware 保护：

```text
/api/admin/configs
/api/admin/users
/api/admin/challenges
/api/admin/games
/api/admin/idps
/api/admin/instances
/api/admin/submissions
```

管理员路由覆盖题目、题目附件/checker/writeup/instance 配置、Game、队伍、公告、用户、IdP、站点配置、调试实例和调试提交。目标设计不再为这些资源建立第二套 URL；它们应合并到同一资源路由，调试用途则挂在所属资源下。

### 1.5 当前响应契约

成功列表目前返回资源专用字段和 `total`，例如：

```json
{
  "challenges": [],
  "total": 0
}
```

列表响应继续使用各资源当前的字段结构，不额外引入统一 envelope。

错误响应已经有稳定的机器可读格式：

```json
{
  "code": "challenge_not_found",
  "details": null
}
```

`code` 必须是小写 snake_case。`details` 只承载字段校验或诊断信息，不能把内部错误文本当成稳定 API 字段。

## 2. 当前实现需要修正的地方

这些问题不影响继续维护旧 API，但新接口不能复制它们：

1. API 没有版本前缀，公共资源和管理员资源分别维护，造成相同聚合的 DTO、路由和权限逻辑重复；`/api/admin` 只是权限边界的 URL 表达，不是领域资源。
2. `POST /instances/{id}/renew`、`POST /instances/{id}/stop`、`POST /challenges/status` 等动作路径把命令直接编码进 URL；它们应在新版本中建模为子资源或资源状态转换。
3. 多个更新接口使用 `PUT`，请求体却是部分字段；新接口应使用 `PATCH`，只有完整替换才使用 `PUT`。
4. 删除和无内容成功响应目前大量返回 `200 {}`；新接口统一使用 `204 No Content`，创建使用 `201 Created` 和 `Location`。
5. `user_id`、`team_id`、`game_id` 由客户端传入的查询或请求字段较多。自我视角的资源必须从 session 和资源上下文推导主体，不能信任客户端声明的所有者。
6. `GET /api/instances` 通过多个可选过滤字段表达访问范围，容易把授权上下文和搜索过滤混在一起。新接口保留顶层资源集合，但把查询参数只作为资源筛选；访问范围始终由 session、实例归属和管理员权限决定。
7. 当前 OpenAPI tag 和响应结构虽已存在，但没有一份跨资源的状态码和命名规则。新接口必须先遵守第 4 节的规范。
8. 当前普通用户通过 `GET /api/submissions?id=...` 轮询刚创建的记录，比赛页面也复用了这个集合接口。目标 API 应把用户的单次结果查询与管理员的 submission 审阅集合拆开，普通用户不再拥有 submission 历史列表。

## 3. 目标领域模型

### 3.1 Game 是权限边界

Game 只表示一场正式比赛，包含比赛设置以及该比赛内的题目配置、队伍、提交、公告、计分和实例。练习场不是 Game，也不创建隐藏的 practice Game。

练习场是前端 `/playground` 对应的用户练习上下文：它面向所有 `public = true` 的 Challenge，题目是否加入某场 Game、Game 是否启用，不改变题目在 playground 中的公开练习资格。练习场的提交和实例以当前用户为主体，不属于队伍，也不带 `game_id`。

### 3.2 Challenge 与 GameChallenge 分离

Challenge 是可全局检索的题目内容；`game_challenges` 是题目在某个 Game 中的使用配置。两者不能合并。

目标字段：

```text
challenges.owner_game_id   nullable
challenges.created_by      nullable user id
challenges.updated_by      nullable user id
```

语义：

```text
owner_game_id = NULL       全局题目，可被符合规则的多个 Game 引用
owner_game_id = game_id    该 Game 独有，只能由该 Game 引用
created_by                  首次创建者，创建后不变
updated_by                  最近一次内容、归属或生命周期修改者
```

`public` 表示可见性，不表示谁拥有编辑权。`deleted_at` 表示生命周期状态。Game 独有题目可以公开展示，但不能被其他 Game 引用。

### 3.3 Game admin 是关系，不是全局角色

目标新增关系表：

```text
game_admins
  game_id BIGINT NOT NULL
  user_id BIGINT NOT NULL
  PRIMARY KEY (game_id, user_id)
```

Game admin 可以管理所属 Game 的设置、题目关联、队伍、公告、提交、实例和计分；全局 `Admin` 对所有 Game 自动拥有同等能力。首期不增加可配置 permission 表，也不把 `game_admin` 写入用户的全局 `group`。

### 3.4 Playground 与 Instance

Playground 不是持久化聚合。它是对公开 Challenge 的筛选和用户操作视图：

```text
playground challenges = challenges WHERE public = true AND deleted_at IS NULL
```

普通用户只能在这个集合中查看题目、提交答案、创建自己的练习场实例和维护自己的笔记；正式比赛实例属于 Team，由全队共享。Challenge 的 `public` 字段决定它能否出现在 playground；GameChallenge 只决定它是否出现在某场正式比赛中。

Instance 是运行时资源

Instance 属于 Challenge 在具体上下文中的运行时资源，不是 Challenge 字段，也不是 `game_challenges` 配置。第一阶段可以继续使用 Kubernetes Pod 作为 backing store，但对外 id 必须是稳定的不透明标识。

建议统一状态：

```text
waiting | running | failed | stopped | expired
```

如果需要可靠历史、审计、租期查询或重试记录，再增加 `instances` 表；在此之前不得把 Kubernetes label 当作公开数据库 schema。

## 4. 新 API 的 REST 约束

新接口使用 `/api/v1`，并沿用当前代码已经建立的资源层级和命名：`games/{game_id}/challenges`、`games/{game_id}/teams`、`challenges/{challenge_id}/attachments` 等关系路径继续保留。要扁平化的是权限命名空间：管理员不使用 `/admin` URL 前缀，公共用户和管理员共用一棵资源树；权限由 session、资源关系和 HTTP 操作决定。创建请求中的 `challenge_id`、`game_id` 等是直接字段，不再包在额外的 `context` JSON 中。动词只保留登录、验证、重算、lint 等确实不是 CRUD 的命令。

### 4.1 方法与状态码

| 语义 | 方法 | 成功响应 |
| --- | --- | --- |
| 列表 | `GET /resources` | `200` |
| 获取 | `GET /resources/{id}` | `200` |
| 创建 | `POST /resources` | `201`，返回 `Location` |
| 部分更新 | `PATCH /resources/{id}` | `200` |
| 完整替换或幂等设置 | `PUT /resources/{id}` | `200` 或 `204` |
| 删除/解除关联 | `DELETE /resources/{id}` | `204` |
| 异步任务已接收 | `POST` 命令资源 | `202` |

资源已不存在时，重复 `DELETE` 应保持幂等；异步任务返回任务资源或明确的 `202` 响应，不返回伪造的同步结果。

### 4.2 身份、主体和权限

- 当前用户从 session 推导，不在 self 资源的请求体中接收 `user_id`。
- 正式比赛中的当前队伍由服务端根据 `game_id` 和 session 用户的成员关系推导，不让普通用户提交任意 `team_id`；Team 不是实例的父资源。
- Game 上下文可以作为资源字段或查询过滤出现；handler 必须先验证资源关系和调用者权限，再执行操作。路径是否嵌套不改变授权边界。
- `Admin` 是授权主体，不是资源命名空间。普通用户和管理员访问同一个资源路径；handler 根据权限返回不同 projection，或拒绝不允许的写操作。
- `404` 用于隐藏调用者无权访问的资源；必须明确区分时使用 `403`。
- 管理权限来自全局 `Admin` 或 `game_admins` 关系，不能只检查 URL 中的 id。

### 4.3 错误

继续使用当前稳定格式，并在 OpenAPI 中为每个操作声明可能的错误状态：

```json
{
  "code": "validation_failed",
  "details": {
    "fields": {
      "title": "required"
    }
  }
}
```

至少统一以下状态：`400` 请求格式错误、`401` 未认证、`403` 无权限、`404` 不存在或不可见、`409` 状态冲突、`422` 语义校验失败、`423` Game 暂停、`429` 限流、`500` 服务错误。

### 4.4 移除 `/admin` 路由

当前代码把 `crates/web/src/router/api/admin` 作为第二棵路由树，并在 `nest("/admin", ...)` 上统一挂载 `admin_only`。目标不是把这棵树原样挂到 `/api/v1` 根路径，因为它会与现有的 `challenges`、`games`、`submissions`、`instances` 等同方法路由冲突。正确做法是把权限从 URL 前缀移动到资源操作：

1. 为每个聚合建立一个唯一的资源 router。普通用户和管理员使用同一个 path；handler 先读取 `AuthPrincipal`，再根据操作和资源关系选择授权策略。
2. 合并重复 DTO 和 handler。普通用户返回公开或脱敏 projection，管理员返回管理 projection；不能让两个 handler 通过不同 URL 暴露同一资源的不同版本。
3. 对同一路径上的不同方法分别授权。例如 `GET /api/v1/challenges` 可以向普通用户返回公开题目，`POST /api/v1/challenges` 只允许全局 Admin；`PATCH /api/v1/challenges/{challenge_id}` 再根据全局题目或 Game 独有题目的归属检查 Game admin 权限。
4. 调试操作仍然使用所属资源的集合：实例调试使用 `POST /api/v1/instances` 的 `purpose: "debug"` 表示，checker 预览使用 `POST /api/v1/challenges/{challenge_id}/checks`；重新判题使用 `POST /api/v1/submissions/{submission_id}/rechecks`。URL 表达资源类型，不把管理员用途写成顶层资源名。
5. 管理员过滤不能扩大可见范围。`GET /api/v1/submissions`、`GET /api/v1/instances` 等集合在普通用户调用时只能返回本人的结果或本人所属 Team 的资源，在 Admin 调用时才启用全局过滤字段和 review projection。
6. 所有资源操作共用 application service 和 policy 函数；旧 admin handler 不能继续保留一份独立业务逻辑。

目标路由示意：

```text
/api/v1/challenges
/api/v1/challenges/{challenge_id}
/api/v1/games
/api/v1/games/{game_id}
/api/v1/users
/api/v1/users/{user_id}
/api/v1/idps
/api/v1/configs
/api/v1/submissions
/api/v1/instances
/api/v1/challenges/{challenge_id}/checks
```

`/api/v1/users`、`/api/v1/idps`、`/api/v1/configs` 的写操作仍然是管理员操作，但不再出现 `/admin` 前缀。`/api/v1/users/me` 等 self 资源继续使用普通用户权限。Game、Team、GameChallenge、Notice、附件和 checker 等资源同样合并到已有的 Game 或 Challenge 资源树，由操作级策略区分普通读取、Game admin 和全局 Admin。

迁移必须按以下顺序执行：

1. 抽取并测试 `require_admin`、`require_game_admin`、`require_team_member` 等 policy 函数；`admin_only` 只作为过渡兼容层。
2. 先在同一个 application service 下实现合并后的资源 handler 和 OpenAPI 路径，暂时保留 `/api/admin` 适配路由调用这些 handler。
3. 将前端 `web/src/api/admin/**` 的请求改为同一资源路径，并让管理员页面继续通过权限和 projection 工作。
4. 为旧 `/api/admin/*` 返回 `Deprecation` 与 `Sunset`，监控调用量；兼容层不得拥有独立业务逻辑。
5. 调用量归零后删除 `api::admin` 模块、`admin_only` 路由层、旧 OpenAPI tags 和前端 `admin/` URL 前缀。

验收要求包括：同一资源只有一个目标 URL；普通用户访问管理员写操作得到 `403`；管理员访问同一路径得到管理 projection；旧 `/api/admin/*` 只在过渡期存在；OpenAPI 中不再出现重复的 admin 操作。

## 5. 目标资源树

以下是目标结构，不代表当前已实现：

```text
/api/v1/configs
/api/v1/version
/api/v1/users
/api/v1/users/me
/api/v1/challenges
/api/v1/challenges/{challenge_id}
/api/v1/challenges/{challenge_id}/attachments
/api/v1/challenges/{challenge_id}/checker
/api/v1/challenges/{challenge_id}/writeup
/api/v1/challenges/{challenge_id}/instance-config
/api/v1/challenges/{challenge_id}/checks
/api/v1/games
/api/v1/games/{game_id}
/api/v1/games/{game_id}/admins
/api/v1/games/{game_id}/challenges
/api/v1/games/{game_id}/teams
/api/v1/games/{game_id}/notices
/api/v1/games/{game_id}/scoreboard
/api/v1/submissions
/api/v1/instances/{instance_id}
/api/v1/instances
/api/v1/submissions/{submission_id}
/api/v1/submissions/{submission_id}/result
/api/v1/submissions/{submission_id}/status
/api/v1/notes
/api/v1/idps
```

### 5.1 Challenge

全局目录只由全局 `Admin` 创建和修改题目内容。前端 `/playground` 只是 `GET /challenges?public=true` 的筛选视图，不是独立 API 资源：

```http
GET   /api/v1/challenges
GET   /api/v1/challenges/{challenge_id}
POST  /api/v1/challenges
PATCH /api/v1/challenges/{challenge_id}
GET   /api/v1/challenges?public=true
```

GameChallenge 的归属由 Game 明确限定，沿用当前代码中的 Game 子资源路径：

```http
GET    /api/v1/games/{game_id}/challenges
POST   /api/v1/games/{game_id}/challenges
GET    /api/v1/games/{game_id}/challenges/{challenge_id}
PATCH  /api/v1/games/{game_id}/challenges/{challenge_id}
DELETE /api/v1/games/{game_id}/challenges/{challenge_id}
```

`POST /games/{game_id}/challenges` 只创建关系，必须引用已有 Challenge：

```json
{ "challenge_id": 123, "difficulty": 5, "max_pts": 2000 }
```

如果需要创建 Game 独有题目，先通过 `POST /api/v1/challenges` 创建带 `owner_game_id` 的 Challenge，再创建关系。全局 Admin 或被授权的 Game admin 才能执行对应创建。若要让独有题目变为全局资源，使用显式的迁移命令资源：

```http
POST /api/v1/games/{game_id}/challenges/{challenge_id}/releases
```

该命令只能单向释放；全局题目需要独立内容时使用 clone 创建新资源。

附件、checker、writeup 和实例配置属于 Challenge 的子资源。它们使用同一条路径，普通用户只能读取公开 projection，Admin 才能创建或修改：

```text
/api/v1/challenges/{challenge_id}/attachments
/api/v1/challenges/{challenge_id}/checker
/api/v1/challenges/{challenge_id}/writeup
/api/v1/challenges/{challenge_id}/instance-config
```

### 5.2 Game admin 关系

```http
GET    /api/v1/games/{game_id}/admins
PUT    /api/v1/games/{game_id}/admins/{user_id}
DELETE /api/v1/games/{game_id}/admins/{user_id}
```

`PUT` 是幂等的，重复添加不会产生重复关系。关系表只保存当前授权；历史审计写入统一 audit log，而不是混入业务关系表。

### 5.3 Submissions

Submission 是独立的、创建后不可变的资源。创建和读取都使用顶层集合；`challenge_id` 和可选的 `game_id` 直接位于请求体中，不使用嵌套路径或额外的 `context` 包装。创建成功后，规范资源地址始终是 `/api/v1/submissions/{submission_id}`。

普通用户创建提交时直接选择 Challenge 和可选的 Game，作者和队伍由服务端推导：

```http
POST /api/v1/submissions
```

请求体只包含提交内容：

```json
{
  "challenge_id": 123,
  "game_id": 456,
  "content": "flag{...}"
}
```

`game_id` 缺省表示 `/playground` 上下文；此时 Challenge 必须是 `public = true` 且未删除。带有 `game_id` 时，服务端必须验证 Challenge 已加入该 Game、Game 处于允许提交的状态，并从 session 用户解析其 Team。请求不能包含 `user_id` 或 `team_id`，也不能通过字段声明作者。创建成功返回 `201 Created` 和 `Location: /api/v1/submissions/{submission_id}`。

由于提交是不可变事件，客户端应发送 `Idempotency-Key`。同一用户在同一上下文中使用相同 key 和相同内容重试时，服务端返回同一个 submission；同一 key 对应不同内容或不同上下文时返回 `409`。

提交是不可变事件，不能用通用 `PATCH` 修改 content，也不提供普通用户的删除接口。用户没有 submission 历史列表权限；为了让前端等待异步 checker 的结果，用户只能查询本次创建响应返回的资源结果：

```http
GET /api/v1/submissions/{submission_id}/result
```

该接口只返回面向提交者的最小结果 projection，例如 `id`、`status`、`checked_at`，不返回提交内容，也不返回其他 submission。服务端必须验证当前用户是该 submission 的提交者；如果结果查询需要短期有效期，应由服务端设置过期策略，而不是把历史列表开放给用户。

只有管理员可以审阅 submission 集合；Game 和 Challenge 只通过过滤参数选择范围：

```http
GET /api/v1/submissions
GET /api/v1/submissions/{submission_id}
```

例如 `GET /api/v1/submissions?game_id=456&challenge_id=123`。全局 Admin 可以访问所有范围；Game admin 只能访问自己管理的 Game。管理员集合接口才允许使用 `status`、`user_id`、`team_id`、`game_id`、`challenge_id` 等过滤条件，并且过滤不能扩大授权范围。

管理员读取的是完整 review projection，可以包含提交内容、作者、队伍、Game、判题时间和计分字段。普通用户接口与管理员接口不能复用同一个可自由筛选的集合 handler。

公开 scoreboard 或 challenge status 如果需要展示解题时间线，只能使用单独的脱敏派生 projection；它不提供 submission 集合、提交内容或任意 submission 详情访问，不能被视为普通用户的审阅权限。

Checker worker 负责正常状态转换，普通客户端不能写入 `status`。如果管理员需要人工修正判题状态，可以使用状态子资源：

```http
PUT /api/v1/submissions/{submission_id}/status
```

该接口仅允许 Admin 或对应 Game admin，必须校验状态转换、正确提交冲突和计分重算，并保留审计记录。`PUT` 在这里表示完整替换 status 子资源，重复提交相同状态必须是幂等的。

如果未来需要重新执行 checker，应创建单独的任务资源，而不是直接伪造状态变化：

```http
POST /api/v1/submissions/{submission_id}/rechecks
```

该接口可以返回 `202 Accepted`。当前没有判题尝试历史表，因此第一阶段可以只保留 worker 内部重试，不公开 `rechecks` 资源。

管理员调试提交不创建 Submission 资源。它是 Challenge 下的一次同步 checker `Check`，应继续与正式 Submission 分离：

```http
POST /api/v1/challenges/{challenge_id}/checks
```

请求体只包含待检查的 `content`，服务端从路径推导 `challenge_id`。它返回即时检查结果，不计入 Submission、scoreboard 或用户结果历史；该 Check 不要求持久化为历史资源。

### 5.4 Instances

Instance 是独立的运行时资源，所有上下文都作为资源字段保存。练习场实例的 owner 是 session 用户；正式比赛实例的 owner 是 `(game_id, team_id, challenge_id)`，同一队的所有成员读取和操作同一个 `instance_id`。`user_id` 不能被用来把正式比赛实例拆成用户实例。

实例 API 使用顶层集合，查询参数只负责筛选，不负责授予权限：

```http
GET    /api/v1/instances?challenge_id=123&game_id=456
POST   /api/v1/instances
GET    /api/v1/instances/{instance_id}
POST   /api/v1/instances/{instance_id}/renewals
DELETE /api/v1/instances/{instance_id}
GET    /api/v1/instances/{instance_id}/connections/{port}
```

创建请求直接包含资源选择字段：

```http
POST /api/v1/instances
```

```json
{
  "challenge_id": 123,
  "game_id": 456
}
```

`game_id` 缺省表示 Playground，此时服务端从 session 推导 `user_id`，并拒绝非公开或已删除的 Challenge。带有 `game_id` 时，服务端从 session 用户解析 `team_id`，请求体不得包含 `user_id` 或 `team_id`。正式比赛必须在 application service 和持久化/锁机制中保证 `(game_id, team_id, challenge_id, active)` 唯一；Playground 必须保证 `(user_id, challenge_id, active)` 唯一。重复创建已有活跃实例时返回 `409` 并带有现有 `instance_id`，客户端随后使用顶层资源读取或操作它，不设置 `instances/current` 别名。

普通用户的 `GET /instances` 只能返回自己在 Playground 中的实例和自己所属 Team 的正式比赛实例；管理员可以使用 `game_id`、`team_id`、`user_id`、`challenge_id` 和状态过滤审阅实例集合，但过滤不能扩大授权范围。创建成功后，客户端使用响应中的 instance id 访问规范资源；停止实例使用 `DELETE`，续期使用 `POST /renewals`。WebSocket upgrade 仍然是特殊传输，但资源授权必须先验证实例 owner 或同队成员关系。

管理员调试实例仍然是 Instance 资源，和玩家实例共用集合：

```http
POST /api/v1/instances
```

管理员请求可以额外指定：

```json
{
  "challenge_id": 123,
  "game_id": 456,
  "purpose": "debug"
}
```

`purpose` 缺省表示玩家实例；`purpose: "debug"` 只能由全局 Admin 或有权管理该 Game 的 Game admin 使用。服务端不从 session 推导 debug instance 的玩家或 Team owner；它仍返回标准的 `/api/v1/instances/{instance_id}` 资源，但不会出现在普通用户的实例集合中。

debug instance 不属于玩家或队伍，不参与计分，也不出现在普通用户的实例集合中。

## 6. 旧 API 兼容与迁移

迁移期间保留旧路由，但旧 handler 只能做参数转换并调用同一套 application service，不能复制业务逻辑。新响应应增加 `Deprecation`，并在确定下线日期后增加 `Sunset`。

Submission 旧接口的兼容层必须按调用者分流：旧的 `POST /api/submissions` 先解析公开 playground 或 Game 上下文，再调用新的创建 service；旧的 `GET /api/submissions?id=...` 只允许 submission owner 轮询自己的结果，并转换为 result projection；旧的任意历史过滤组合不再对普通用户开放。比赛 scoreboard 的旧查询迁移到 scoreboard 专用的脱敏 projection，不能继续依赖公共 submission 集合。

| 当前接口 | 目标接口 |
| --- | --- |
| `GET /api/challenges` | `GET /api/v1/challenges` |
| `GET /api/games/{game_id}/challenges` | `GET /api/v1/games/{game_id}/challenges` |
| `GET /api/submissions` | 管理员使用 `GET /api/v1/submissions` 加过滤参数；普通用户只使用已知 id 的 result |
| `POST /api/submissions` | `POST /api/v1/submissions`，请求体直接包含 `challenge_id` 和可选 `game_id` |
| `GET /api/instances?challenge_id=...` | `GET /api/v1/instances?challenge_id=...&game_id=...`；普通用户只返回有权访问的实例 |
| `POST /api/admin/instances` | `POST /api/v1/instances`，管理员请求使用 `purpose: "debug"` |
| `POST /api/admin/submissions/debug` | `POST /api/v1/challenges/{challenge_id}/checks` |
| `POST /api/instances/{id}/renew` | `POST /api/v1/instances/{id}/renewals` |
| `POST /api/instances/{id}/stop` | `DELETE /api/v1/instances/{id}` |
| `GET /api/instances/{id}/wsrx?port=...` | `GET /api/v1/instances/{id}/connections/{port}` |
| `/api/admin/*` | `/api/v1` 同一资源路径上的操作级权限 |

迁移顺序：

1. 抽取共享 application service 和授权策略，旧路由先继续工作。
2. 增加数据库迁移：`game_admins`、Challenge owner/audit 字段；不为 playground 创建 Game 或 `games.kind`。
3. 实现 `/api/v1` 资源树和 OpenAPI，固定现有资源专用列表字段和错误状态码。
4. 前端迁移到 `/api/v1`，按 Game admin 显示管理能力。
5. 监控旧路径使用量，发布弃用通知，最后删除兼容路由。

## 7. 数据迁移规则

- 旧 Challenge 的 `owner_game_id`、`created_by`、`updated_by` 初始为 `NULL`，不根据历史关联猜测独占关系。
- 旧题目默认视为全局题目；只有显式操作才能标记为 Game 独有。
- `game_admins` 使用 `(game_id, user_id)` 复合主键，外键删除时级联。
- 新增字段的外键建议使用 `ON DELETE SET NULL`，保留历史资源的可读性。
- 计分配置属于 `game_challenges`，不能写入 Challenge 的全局内容字段。
- Playground 不新增比赛或 GameChallenge 数据；它只查询 `public = true` 且未删除的 Challenge。
- Playground 实例和提交的 `game_id`、`team_id` 保持为空；实例 owner 和提交 user 从 session 推导。
- 正式比赛实例的 Team 从 Game 和当前用户解析，不通过 `teams/me` 把 Team 误作实例 owner；同队成员共享同一个实例资源。
- 正式比赛和 Playground 都不使用 `instances/current`；唯一活跃实例由数据库唯一约束、application service 和锁机制共同保证。
- 普通用户没有 submission 历史列表；只能轮询自己刚创建的 submission result。
- scoreboard 和 challenge status 的公开 submission 信息必须是脱敏派生 projection，不授予 submission 审阅权限。
- 实例在没有持久化表之前不能承诺历史查询；API 只承诺当前 backing store 能提供的状态。

## 8. 验收标准

实现目标架构前，至少应通过以下检查：

```text
cargo check --workspace
cargo test --workspace
OpenAPI 文档包含每个新路由且没有重复 admin 操作
所有新创建接口返回 201 和 Location
所有新删除接口返回 204
所有部分更新接口使用 PATCH
所有 self/team 资源不接受客户端声明的 owner
普通 User 不能访问其他 Game 的资源
game_admin 只能管理被授权的 Game
全局 Admin 可以管理所有 Game
全局 Challenge 删除不会误删其他 Game 的关联
Game 独有 Challenge 不能被其他 Game 引用
playground 只展示 `public = true` 且未删除的 Challenge
playground 操作不创建隐藏 Game，不写入 `game_challenges`
playground 提交和实例的 Game/Team 字段为空，正式比赛实例按 Team 共享
旧路由和新路由调用同一 application service
```

任何新增资源都必须同时提交：数据库约束、application service、授权测试、handler 测试和 OpenAPI 元数据。这样扩展平台能力时，资源边界、权限边界和 HTTP 语义会保持一致。
