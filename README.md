# PgsHub

> 此为后端核心部分

## Swagger

```bash
swag init -g ./cmd/pgshub/main.go -o ./docs
```

## 数据库

### Postgres
```sql
CREATE USER pgshub WITH PASSWORD 'pgshub';
CREATE DATABASE pgshub OWNER pgshub;
GRANT ALL PRIVILEGES ON DATABASE pgshub TO pgshub;
```