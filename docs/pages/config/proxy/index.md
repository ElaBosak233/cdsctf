# 代理与流量捕获

Cloudsdale 的代理功能主要是为了流量捕获功能服务的。

当然，如果你不希望选手直接通过 IP 地址进入容器，你可以使用平台代理。你只需要保证 Cloudsdale 能够访问容器即可。

Clousdale 通过 **TCP over Websocket** 进行代理，简单来说，TCP over Websocket 就是通过一个 Websocket 链接（在 Cloudsdale 中是 `/api/proxies/[UUID]`）作为桥梁，连接靶机。

那又产生了一个问题，我是不是需要以 `ws://` 这样的形式来访问题目呢？这时候你就需要连接器。

所谓无感交互，就是让连接器在本地开启一个 TCP 端口，做题的时候你只需要向连接器提供 `ws://` 链接，然后访问被分配到的 TCP 端口即可，做题的体验没什么太大的区别。

Cloudsdale 推荐使用 [WebsocketReflectorX（简称 WSRX）](https://github.com/XDSEC/WebsocketReflectorX) 作为你的连接器。
