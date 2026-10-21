# Checker subject numbers

Lua uses `check(subject_id, content)` and `generate(subject_id)` with the same
positional arguments as before. Treat `subject_id` as an opaque signed integer:
pass it unchanged to `checker.suid`, `checker.leet`, and comparisons with their
decoded values.
Do not take its absolute value or use it as a user database id.

Rust callers pass `Subject::User(id)` or `Subject::Team(id)` to both checker
entry points. The checker maps positive database ids to disjoint namespaces:

| Owner | Lua `subject_id` |
| --- | --- |
| User | `-user_id` |
| Team | `team_id` |

Zero and `i64::MIN` are invalid subject numbers. Static and regex scripts, and
dynamic scripts that already round-trip `subject_id`, need no changes.

`checker.audit.cheat(decoded_id)` also keeps its Lua signature. Rust resolves
the source number before applying policy: only a different team submitting a
team-owned flag produces `Status::Cheat(peer_team_id)`. User sources, personal
submissions, invalid source numbers, and self-reports become `Incorrect`.
The worker still verifies the peer team's game before changing team state.

The audit helper now returns `{ kind = "cheat", subject_id = decoded_id }`.
Rust also accepts the legacy `operator_id` field in hand-written return tables.
Old function parameter names such as `operator_id` continue to work because
Lua binds arguments by position. When inspecting the helper's returned table,
use `subject_id`. Return either field name, not both.

## Upgrading existing deployments

Newly generated personal and team flags round-trip through the same scripts.
Existing team flags retain their positive subject numbers and remain valid
when their keys and scripts are unchanged. Static and regex flags are unchanged.
Existing personal dynamic flags contain positive numbers and must be replaced;
there is no safe fallback that accepts them while separating same-numbered
users and teams.

Before resuming a deployment that has issued personal dynamic flags, rotate
the affected challenges' keys and recreate their active challenge instances.
This also invalidates old team flags for those challenges. Without rotation,
an old personal flag can still be mistaken for a team-owned flag with the same
positive id. For default keys, replace `challenges/{challenge_id}/.key` and
restart all checker processes to clear their key/Lua caches. For script-owned
keys, update the keys in the scripts as well. Keep submissions paused until
all generation and checking processes use the new scheme and keys.

Custom scripts that assume a positive user id, hard-code database ids, or use
`subject_id` in an unsigned external API require adaptation.

## Validation

Run `cargo test -p cds-checker --lib`. The bundled Lua templates are executed
unchanged for users and teams, including equal database ids and `i64::MAX`.
Tests cover valid flags, malformed flags, foreign sources, custom keys,
source-to-policy conversion, and signed codec round trips.
