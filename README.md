# PegasiHub

> 此为后端核心部分

## 数据库

### MySQL
```mysql
CREATE DATABASE pgshub;
CREATE USER 'pgshub'@'%' IDENTIFIED BY 'pgshub';
GRANT ALL PRIVILEGES ON pgshub.* TO 'pgshub'@'%';
```

### Postgres
```postgresql
CREATE USER pgshub WITH PASSWORD 'pgshub';
CREATE DATABASE pgshub OWNER pgshub;
GRANT ALL PRIVILEGES ON DATABASE pgshub TO pgshub;
```