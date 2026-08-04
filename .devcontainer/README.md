# CdsCTF Dev Container

The post-start hook launches PostgreSQL, Valkey, NATS JetStream, and RustFS
from `compose.yml`. Their default ports are listed in `devcontainer.json`.

When those ports are already occupied, override only the host-side bindings:

```sh
CDSCTF_DEV_DB_PORT=15432 \
CDSCTF_DEV_CACHE_PORT=16379 \
CDSCTF_DEV_QUEUE_PORT=14222 \
CDSCTF_DEV_QUEUE_MONITOR_PORT=18222 \
CDSCTF_DEV_MEDIA_PORT=19000 \
CDSCTF_DEV_MEDIA_CONSOLE_PORT=19001 \
docker compose -f .devcontainer/compose.yml up -d
```

To run the backend on the host against those alternate ports:

```sh
CDSCTF_SERVER__PORT=18888 \
CDSCTF_DB__HOST=127.0.0.1 \
CDSCTF_DB__PORT=15432 \
CDSCTF_CACHE__URL=redis://127.0.0.1:16379 \
CDSCTF_QUEUE__HOST=127.0.0.1 \
CDSCTF_QUEUE__PORT=14222 \
CDSCTF_MEDIA__ENDPOINT=http://127.0.0.1:19000 \
cargo run -p cds-server
```

NATS monitoring is available on port `8222` by default, including `/healthz`
and `/jsz`.
