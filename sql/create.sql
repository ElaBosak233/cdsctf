CREATE DATABASE pgshub;
CREATE USER 'pgshub'@'%' IDENTIFIED BY 'pgshub';
GRANT ALL PRIVILEGES ON pgshub.* TO 'pgshub'@'%';