# 缓存

Cloudsdale 有两种缓存实现方式，一种是基于 `go-cache`，另一种是基于 `go-redis`，如果你有分布式需求，请使用 `go-redis` 保证缓存的正确性。