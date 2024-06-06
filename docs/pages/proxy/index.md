# 代理与流量捕获

Cloudsdale 的代理功能主要是为了流量捕获功能服务的。

当然，如果你不希望选手直接通过 IP 地址进入容器，你可以使用平台代理。你只需要保证 Cloudsdale 能够访问容器即可。

Clousdale 通过 **TCP over Websocket** 进行代理，简单来说，TCP over Websocket 就是通过一个 Websocket 链接（在 Cloudsdale 中是 `/api/proxies/[UUID]`）作为桥梁，连接靶机。

那又产生了一个问题，我是不是需要以 `ws://` 这样的形式来访问题目呢？

虽然 Cloudsdale 返回的是一个这样的 `ws://` 链接，但是你只需要用一个很简单的 Python 脚本就能实现无感交互了。

所谓无感交互，就是让 Python 在本地开启一个 TCP 端口，做题的时候你只需要给脚本提供 `WS_URL` 常量，然后连接这个被脚本分配到的地址即可，做题的体验没什么太大的区别。

下面是一个比较简单的实现这个功能的 Python 脚本，仅作为参考。

```python
import asyncio
import websockets
import socket

WS_URL = "ws://0.0.0.0:8888/api/proxies/[UUID]"


async def handle_tcp_client(tcp, address):
    print(f"Accepted connection from {address}")

    async with websockets.connect(WS_URL) as ws:
        while True:
            data = tcp.recv(1024)
            if not data:
                break
            await ws.send(data)

            response = await ws.recv()
            if isinstance(response, str):
                response = response.encode("utf-8")
            tcp.send(response)


tcp_server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
tcp_server.bind(("localhost", 0))
tcp_server.listen(1)

port = tcp_server.getsockname()[1]
print(f"TCP server is listening on localhost:{port}")

while True:
    tcp_client, addr = tcp_server.accept()
    asyncio.get_event_loop().run_until_complete(handle_tcp_client(tcp_client, addr))

```

当然，我们还有另一种方法，就是使用一个连接器，比如 [Netweaver](https://github.com/elabosak233/Netweaver)，这样就能实现更好地交互