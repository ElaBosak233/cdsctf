# PgsHub

## 数据库

### Postgres
```sql
CREATE USER pgshub WITH PASSWORD 'pgshub';
CREATE DATABASE pgshub OWNER pgshub;
GRANT ALL PRIVILEGES ON DATABASE pgshub TO pgshub;
```