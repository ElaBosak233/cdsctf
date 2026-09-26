# CdsCTF API、权限与题目归属重构方案

状态：设计稿

实施分支：`codex/api-cleanup`

## 1. 目标

这次改动同时解决四个问题：

1. API 当前同时维护公共资源树和 `/api/admin` 资源树，造成路由、DTO、service 和前端 client 重复。
2. `Group::Admin` 与 `admin_only` 只能表达粗粒度的管理员身份，无法表达比赛范围内的管理权限。
3. `Challenge` 虽然需要保留全局检索和复用能力，但比赛管理员又需要能够直接创建和维护比赛内的题目。
4. 题目的归属、可见性、创建者和最后修改者目前没有清晰分离。

目标是让使用者只需要理解三件事：

```text
system_admin 管理整个系统
game_admin 管理被分配的比赛
Challenge 可以被全局检索，并且可以是某场比赛独有的题目
```

## 2. 不变的产品能力

重构必须保留：

- 全局 Challenge 搜索。
- Challenge 被多个 Game 引用。
- Game 内创建 Challenge。
- 比赛题目的 checker、writeup、附件、实例和计分配置。
- 练习场（playground）。
- 正式比赛中的队伍、提交、计分、公告和实例。
- 现有题目、提交、笔记和媒体数据。

首期不引入 Challenge revision/version 系统。共享题目如果需要独立修改，使用 clone 创建新的 Challenge；版本系统可以作为后续扩展。

## 3. 领域模型

### 3.1 Game

Game 是比赛范围的权限边界，包含：

```text
Game settings
GameChallenge
Teams
Submissions
Notices
Instances
Scoreboard
```

为保留练习场，给 Game 增加类型：

```text
competition
practice
```

练习场是一个特殊的 Game，而不是一套独立的 Challenge 模型。

### 3.2 Challenge

Challenge 继续是全局资源，可以被全局搜索，也可以被多个 Game 引用。

新增字段：

```text
owner_game_id   nullable
created_by      nullable user id
updated_by      nullable user id
```

字段语义：

```text
owner_game_id = NULL
  全局题目，可以被多个 Game 引用

owner_game_id = 某个 game id
  Game 独有题目，只能被该 Game 引用

created_by
  首次创建题目的用户，创建后不变

updated_by
  最近修改题目内容、归属或生命周期状态的用户
```

`owner_game_id` 不等同于 `public`：

```text
public        是否可被公共检索或展示
owner_game_id 谁拥有内容修改和引用控制权
deleted_at    生命周期状态
```

一个 Game 独有的 Challenge 仍然可以是公开可搜索的，但其他 Game 不能引用它。

### 3.3 GameChallenge

保留 `game_challenges`，因为它表达题目在某个 Game 中的使用配置：

```text
game_id
challenge_id
difficulty
max_pts
min_pts
bonus_ratios
enabled
frozen_at
pts
```

它不是另一种题目，而是 Challenge 在 Game 中的配置和关联。

约束：

- 全局 Challenge 可以有多个 `game_challenges`。
- Game 独有 Challenge 只能有一个 `game_challenges`，且 `game_id == owner_game_id`。
- 该跨表约束由 application service 和事务保证，并通过集成测试覆盖。

## 4. 权限模型

### 4.1 全局角色

每个用户绑定一个全局 role：

```text
user
platform_operator
system_admin
```

`system_admin` 是最高角色，但不使用数字大小比较。它在授权策略中拥有全局 bypass 能力。

`banned`、`suspended` 等状态属于账号状态，不属于 role。账号状态检查优先于权限检查。

### 4.2 Game 角色

用户还可以在每场 Game 中拥有一个预设角色：

```text
game_admin
game_viewer
```

第一期只实现这两个角色。需要细分职责时再增加 `game_content_editor`、`game_judge` 等预设。

Game admin 的核心权限是：

```text
game:manage
```

它覆盖该 Game 下的所有内容：

```text
game:update
game:manage_staff
game:challenges:manage
game:teams:manage
game:submissions:manage
game:notices:manage
game:instances:manage
game:scoreboard:recalculate
```

Game admin 不需要额外拥有全局 `challenge:write`。

### 4.3 Challenge 权限规则

对 Game 独有 Challenge：

```text
owner_game_id == current_game_id
且 actor 是该 Game 的 game_admin
=> 可以修改题目内容
```

对全局 Challenge：

```text
game_admin
  可以修改本 Game 的 GameChallenge 配置

challenge_catalog_admin 或 system_admin
  才能修改 Challenge 全局内容
```

Game admin 可以修改全局题目的：

```text
difficulty
max_pts
min_pts
bonus_ratios
enabled
frozen_at
```

不能修改全局题目的：

```text
title
description
category
tags
checker
writeup
instance
attachments
```

## 5. Challenge 生命周期

### 5.1 在 Game 中创建

```http
POST /api/v1/games/{game_id}/challenges
```

请求：

```json
{
  "title": "SQL Injection",
  "description": "...",
  "category": 2,
  "tags": ["web"],
  "checker": "...",
  "writeup": "...",
  "difficulty": 5,
  "max_pts": 2000,
  "min_pts": 500
}
```

后端在一个事务中：

1. 验证 actor 是否有该 Game 的 `game:manage`。
2. 创建 Challenge，并设置 `owner_game_id = game_id`。
3. 设置 `created_by = actor.id` 和 `updated_by = actor.id`。
4. 创建对应的 `game_challenges`。
5. 初始化计分配置。

### 5.2 引用全局题目

```http
POST /api/v1/games/{game_id}/challenges
```

请求：

```json
{
  "challenge_id": 123,
  "difficulty": 5,
  "max_pts": 2000
}
```

这只创建 `game_challenges`，不改变 Challenge 的全局归属。

### 5.3 删除

```http
DELETE /api/v1/games/{game_id}/challenges/{challenge_id}
```

如果 Challenge 是该 Game 独有的：

```text
软删除 Challenge
删除 GameChallenge 关联
保留 submissions、notes 和审计历史
```

如果 Challenge 是全局的：

```text
只删除 GameChallenge 关联
不删除 Challenge
不影响其他 Game
```

### 5.4 转移到全局

```http
POST /api/v1/games/{game_id}/challenges/{challenge_id}/release
```

要求：

```text
Challenge.owner_game_id == game_id
actor 是该 Game 的 game_admin
```

操作：

```text
owner_game_id = NULL
updated_by = actor.id
保留当前 GameChallenge
记录审计日志
```

转移后，原 Game admin 仍可修改该题目在本场比赛中的计分和启用配置，但不能修改全局内容。

该操作建议单向进行。全局题目如果想成为某个 Game 的独有题目，应当 clone，而不是直接改变归属。

## 6. API 目标结构

### 6.1 版本和资源

新 API 使用 `/api/v1`：

```text
/api/v1/users
/api/v1/games
/api/v1/games/{game_id}/challenges
/api/v1/teams
/api/v1/submissions
/api/v1/instances
/api/v1/identity-providers
/api/v1/settings
```

前端管理页面继续使用 `/admin`，但 `/admin` 只属于前端路由，不代表 API 必须使用 `/api/admin`。

### 6.2 Challenge 路由

全局目录：

```http
GET   /api/v1/challenges
GET   /api/v1/challenges/{challenge_id}
POST  /api/v1/challenges
PATCH /api/v1/challenges/{challenge_id}
```

其中创建和修改全局 Challenge 需要 `challenge_catalog:manage`。

比赛上下文：

```http
GET    /api/v1/games/{game_id}/challenges
POST   /api/v1/games/{game_id}/challenges
GET    /api/v1/games/{game_id}/challenges/{challenge_id}
PATCH  /api/v1/games/{game_id}/challenges/{challenge_id}
DELETE /api/v1/games/{game_id}/challenges/{challenge_id}
POST   /api/v1/games/{game_id}/challenges/{challenge_id}/release
```

附件、checker、writeup 和 instance 都应带有 Game 上下文，避免只凭 challenge id 越权：

```http
/api/v1/games/{game_id}/challenges/{challenge_id}/attachments
/api/v1/games/{game_id}/challenges/{challenge_id}/checker
/api/v1/games/{game_id}/challenges/{challenge_id}/writeup
/api/v1/games/{game_id}/challenges/{challenge_id}/instance
```

### 6.3 HTTP 约定

```text
POST   创建资源，201 + Location
PATCH  部分更新
DELETE 删除资源，204
202    异步任务已接收
401    未认证
403    无权限
404    资源不存在
409    资源冲突
422    参数校验失败
```

列表响应统一使用：

```json
{
  "data": [],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total": 100
  }
}
```

## 7. 数据库迁移

### 7.1 RBAC 表

新增：

```text
roles
permissions
role_permissions
users.global_role_id
game_staff
```

推荐在 `users` 上使用 `global_role_id`，确保每个用户只有一个全局 role；使用 `game_staff(game_id, user_id, role_id)` 表达比赛范围的 role。

### 7.2 Challenge 字段

新增：

```sql
ALTER TABLE challenges
    ADD COLUMN owner_game_id BIGINT,
    ADD COLUMN created_by BIGINT,
    ADD COLUMN updated_by BIGINT;
```

外键均使用 `ON DELETE SET NULL`，以保护历史数据。旧题目的 `owner_game_id`、`created_by`、`updated_by` 初始保持 `NULL`，不伪造历史信息。

新增索引：

```text
challenges(owner_game_id)
challenges(created_by)
challenges(updated_by)
```

### 7.3 Game 类型

给 `games` 增加：

```text
kind = competition | practice
```

创建或迁移一个默认 Practice Game。练习场使用普通 GameChallenge，提交时设置 `game_id`，不需要独立的 Challenge 逻辑。

### 7.4 旧数据

旧 Challenge 默认迁移为全局题目：

```text
owner_game_id = NULL
```

原因是旧数据可能被多个 Game 使用，不能根据现有关联自动推断独占关系。

旧 `game_challenges` 保留，先由新 service 继续读写。所有代码完成迁移后再考虑删除或改为兼容 view。

## 8. 代码结构

当前 `crates/web/src/router/api/admin` 与公共资源树重复。目标结构：

```text
crates/web/src/
  router/api/v1/
    mod.rs
    users.rs
    games.rs
    challenges.rs
    submissions.rs
  application/
    users.rs
    games.rs
    challenges.rs
    submissions.rs
  policy.rs
  dto/
    users.rs
    games.rs
    challenges.rs
```

分层职责：

```text
router       路径、HTTP 方法、OpenAPI、middleware
handler      参数提取、DTO 校验、调用 application service
application  业务流程、事务、授权上下文
policy       全局 role、game role、资源归属检查
db           SeaORM entity 和 repository
dto          HTTP 请求和响应模型
```

handler 不应直接在每个接口中构造 ActiveModel 并执行完整业务流程。

授权 service 示例：

```rust
authorize_game(actor, game_id, GameAction::Manage)?;
let challenge = load_game_challenge(conn, game_id, challenge_id).await?;

if challenge.owner_game_id == Some(game_id) {
    authorize_game_owned_challenge(actor, game_id)?;
} else {
    authorize_catalog_challenge_update(actor)?;
}
```

## 9. 前端迁移

保留页面路由：

```text
/admin
/admin/games
/admin/challenges
```

迁移 API client：

```text
web/src/api/admin/games
  -> web/src/api/games

web/src/api/admin/challenges
  -> web/src/api/games/game_id/challenges
```

前端不再依赖数字 `Group`：

```ts
type AccessPolicy = {
  permissions?: Permission[];
};
```

Game 管理页面显示的是预设 role：

```text
game_admin
game_viewer
```

数据库和 API 保存稳定 key，界面通过 i18n 显示名称。建议新增：

```text
web/public/locales/zh-CN/roles.yaml
web/public/locales/en-US/roles.yaml
web/public/locales/ja-JP/roles.yaml
web/public/locales/zh-TW/roles.yaml
```

不允许普通用户创建自定义 role 或直接修改 permission。

## 10. 迁移阶段

### 阶段 0：基线

- 生成当前 OpenAPI 快照。
- 列出所有 `/api/admin` 调用点。
- 统计 Challenge 被哪些 Game 引用。
- 统计没有 Game 关联的 Challenge。
- 统计 playground、submission、note、instance 依赖。

验收：有完整的旧 API 路径、权限和数据关系清单。

### 阶段 1：RBAC 基础设施

- 创建 role、permission、game staff 数据结构。
- 回填 `Group::Admin` 为 `system_admin`。
- 回填普通用户为 `user`。
- 实现 `Subject` 和 `Permission`。
- 将 `admin_only` 改为具体权限检查。

验收：匿名、普通用户、system_admin 的授权测试通过。

### 阶段 2：Challenge 归属和审计

- 添加 `owner_game_id`、`created_by`、`updated_by`。
- 修改 Game 内创建题目逻辑。
- 实现独有题目删除规则。
- 实现 release-to-catalog。
- 增加跨 Game 引用检查。

验收：独有题目、全局题目、转移、删除和审计字段测试通过。

### 阶段 3：Practice Game

- 增加 `games.kind`。
- 创建默认 practice game。
- 将 playground 查询改为 practice game 查询。
- 保留全局 Challenge 搜索。
- 统一 practice submission 的 `game_id`。

验收：练习场浏览、提交、状态查询和笔记功能不受影响。

### 阶段 4：新 API

- 新增 `/api/v1` 路由。
- 迁移 users、games、challenges、submissions 等资源。
- 将 GameChallenge 作为 Game 的嵌套资源。
- 统一状态码、分页和错误响应。

验收：OpenAPI 文档无重复 admin 资源，前端可以只调用 `/api/v1`。

### 阶段 5：前端迁移

- 合并或重写 `web/src/api/admin/*`。
- 替换硬编码 `/api/admin` 路径。
- 将 Group 路由判断替换为 permission 判断。
- 增加 Game staff 管理界面。
- 增加 Challenge 独有、全局和已释放状态展示。

验收：管理后台所有页面可以完成原有操作。

### 阶段 6：兼容和清理

- `/api/admin/*` 变成薄兼容层。
- 旧写接口直接调用新 application service，不复制业务逻辑。
- 增加 `Deprecation` 和 `Sunset` 响应头。
- 监控旧 API 使用量。
- 确认前端和外部客户端完成迁移后删除旧路由。

## 11. 测试和验证

必须增加：

### 权限测试

```text
普通用户不能管理 Game
game_viewer 不能修改 Game
game_admin 可以管理该 Game 的所有资源
game_admin 不能管理别的 Game
game_admin 可以修改自己拥有的 Challenge
game_admin 不能修改全局 Challenge 内容
game_admin 可以修改全局 Challenge 的比赛配置
system_admin 可以管理全部资源
被封禁用户始终返回 403
```

### Challenge 生命周期测试

```text
在 Game 中创建独有 Challenge
全局搜索独有 Challenge
其他 Game 不能引用独有 Challenge
删除独有 Challenge 会软删除
删除全局 Challenge 关联只解除当前 Game
release 后其他 Game 可以引用
release 后原 game_admin 不能编辑题目内容
clone 后新题目拥有新的 created_by
```

### 数据和构建检查

```text
cargo check --workspace
cargo test --workspace
pnpm --dir web tsc:build
pnpm --dir web check
OpenAPI snapshot diff
```

## 12. 风险和处理方式

### 题目归属误判

旧数据不自动推断 `owner_game_id`，默认全局。之后通过管理工具显式标记独有题目。

### 全局题目被误删

Game admin 删除全局题目时只删除关联，绝不删除 Challenge 本身。

### 跨 Game 越权

所有 GameChallenge、附件、checker、writeup 和 instance 操作都必须带 `game_id`，并验证 Challenge 归属关系。

### 审计字段不准确

`created_by` 只在创建时写入，`updated_by` 只记录题目内容、归属或生命周期改变。比赛计分配置应单独审计。

### 多语言角色名称不稳定

数据库只保存稳定 role key，所有显示名称和说明由前端 i18n 提供。

## 13. 最终验收标准

重构完成后应满足：

```text
Challenge 仍可全局搜索
Challenge 可以被多个 Game 引用
Game admin 可以在 Game 内创建题目
Game 独有题目不能被其他 Game 引用
Game admin 可以管理自己 Game 的全部内容
Game admin 不能编辑已经释放到全局的题目内容
删除行为根据题目归属正确区分
练习场继续使用普通 Game 模型
用户只有预设 role，没有直接 permission 配置
API 不再依赖 /api/admin 资源复制
旧 API 可以在迁移期间兼容
```

这套方案保留 Challenge 的全局资源属性，同时使用 `owner_game_id` 将“谁能修改”和“谁能引用”表达清楚；`game_challenges` 继续负责比赛配置，Game admin 只需要理解比赛边界，不需要理解底层权限组合。
