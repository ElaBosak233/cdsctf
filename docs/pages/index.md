# 简介

!!! warning "警告"
    Cloudsdale 仍然处于未发布阶段，快照构建成果可能十分不稳定，功能也可能未完善，快照的品质不能代表正式版本的品质。

**Cloudsdale** 是一个基于 GO 构建、使用解题模式（Jeopardy）的 CTF 平台。她非常地轻量，并且可以使用 _非常简单（可能）_ 的配置文件快速部署。

本项目灵感来源于 [CTFd](https://github.com/CTFd/CTFd)、[Cardinal](https://github.com/05sec/Cardinal) 和 [GZ::CTF](https://github.com/GZTimeWalker/GZCTF)，博采众长之下诞生了本项目。但最初的想法仅仅以尽可能处处简单的方式，给学校的 CTF 战队提供训练平台。

## 功能

- 题目
    - 静态题目：无靶机，判题依赖于一个/多个已知的 flag 字符串，通常依赖于附件系统
    - 动态题目：动态靶机，判题可依赖于静态 flag 字符串，也可以使用动态生成的 flag（通常为 `UUID`）
- 靶机
    - 多端口支持
    - 可自定义镜像的基本环境变量
    - 可自定义的容器计算资源索取量（内存与 CPU）
    - 可自定义的 flag 注入变量名
    - 可选的端口映射模式
    - 通过平台代理实现的流量捕获
- 比赛
    - 可自定义的比赛题目分值
    - 可自定义的一二三血奖励比率
    - 过程中可随时禁用/启用题目，实现多次放题
    - 基于 Websocket 实现的比赛内消息广播
- 数据库
    - 基于 GORM 的多种关系型数据库支持（PostgreSQL, SQLite3, MySQL）
- 容器支持
    - Docker
    - Kubernetes（以 k3s 为例）

## 开源协议

Cloudsdale 基于 [GPLv3](https://github.com/ElaBosak233/PgsHub/blob/main/LICENSE) 协议开源，使用和二次开发需严格遵守此协议。

<div style="background-image: url('/assets/img/GPLv3_Logo.svg'); width: 10rem; height: 5rem; background-repeat: no-repeat; background-position: center; background-size: cover;"></div>