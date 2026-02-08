#!/usr/bin/env fish

cargo fetch
cd web
pnpm install --frozen-lockfile 2>/dev/null; or pnpm install
cd -
