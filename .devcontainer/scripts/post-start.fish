#!/usr/bin/env fish

set -q containerWorkspaceFolder; or set containerWorkspaceFolder (pwd)
git config --global --add safe.directory $containerWorkspaceFolder
docker compose -f $containerWorkspaceFolder/.devcontainer/compose.yml up -d
