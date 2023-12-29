# PegasiHub

> 此为后端核心部分

```sql
CREATE DATABASE pgshub;
CREATE USER 'pgshub'@'%' IDENTIFIED BY 'pgshub';
GRANT ALL PRIVILEGES ON pgshub.* TO 'pgshub'@'%';
```