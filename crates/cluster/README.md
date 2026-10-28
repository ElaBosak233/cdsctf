# 实例状态与生命周期

运行中的实例以 Kubernetes Pod / Service 为事实来源，不增加 SQL 实例表。题目、用户、队伍和比赛信息仍由原有数据库管理。

## Metadata v1

- 所有查询限定 `cds/app=challenges`。
- Labels 保存 `cds/instance_id`、`cds/user_id`（创建者）、`cds/challenge_id`、`cds/mode`（practice / game / debug）及网络策略使用的 `cds/internet`。比赛实例同时保存 `cds/team_id`、`cds/game_id`；其他实例不写这两个字段。
- Annotations 仅保存 `cds/schema-version=1`、`cds/duration`（秒）、`cds/renew`（已续期次数）。不保存题目、用户、队伍、比赛 DTO 或端口快照。
- 创建时间使用 Kubernetes `creationTimestamp`；到期时间为创建时间加 `duration × (renew + 1)`。端口来自 Pod spec，NodePort 来自 Service spec。
- Service 通过 ownerReference 关联 Pod，并支持孤立 Service 的清理重试。

解析器兼容无版本字段的旧实例，以及旧的队伍 / 比赛 ID `0`。旧实例未记录调试模式，无法可靠恢复该信息，按练习实例处理。旧 annotation 快照不再读取，但不会主动修改已有资源；它们随实例销毁而移除。

缺失或非法的生命周期数据不会猜测默认 TTL：接口显示 `unknown / invalid_metadata`，禁止续期，清理器记录告警并跳过；有效身份仍可手动停止。非法身份不会参与用户接口投影，需运维检查资源。

## 状态协议

`state` 为 `starting`、`running`、`unhealthy`、`succeeded`、`failed`、`stopping` 或 `unknown`，并返回稳定的 `reason_code`、容器就绪数量、`ready`、`expires_at`、续期次数以及 `can_renew` / `can_stop`。

删除时间戳优先表示停止中，Pod phase 决定终态；当前容器错误统一表示可恢复的异常，不触发自动删除。只有所有业务容器运行且就绪、Pod Ready condition 为 True 时才返回 running / ready。未配置 readiness probe 时，这仅表示 Kubernetes 就绪，不承诺题目应用已经通过业务健康检查。

保留旧字段 `status`、`reason`、`renew`、`duration`、`started_at` 供客户端过渡。新客户端不依据 reason 文本作生命周期操作。终态仍可查询、停止，在 TTL 到期时清理；终态不占创建额度，其余状态（含停止中、未知、异常）占额度。列表优先返回非终态，再按创建时间倒序。

## 写入规则

- 续期最多三次，仅在到期前最后十分钟且尚未到期时允许。每次重读资源、身份和 UID，使用 resourceVersion 条件更新；遇到冲突重新校验，避免重复续期覆盖。
- 手动停止使用 UID 前置条件和正常终止宽限期。停止接口返回 `state=stopping` 表示请求已接受，客户端轮询直至实例消失。已消失的 Pod 在底层删除操作中视为成功；HTTP 层无法再次验证其身份时仍返回 404。
- 自动过期清理同时使用 UID / resourceVersion 前置条件，续期赢得并发更新后，旧清理快照不得删除该 Pod。错误状态本身不是清理条件。
- 创建 Service 失败时尝试回滚；清理器对超过一分钟且经实时查询确认没有 Pod 的孤立 Service 重试清理。

创建额度仍通过现有的查询后创建流程控制，不构成跨请求的原子配额。实例列表当前使用 Kubernetes list，不提供 SQL 历史记录或跨资源事务。
