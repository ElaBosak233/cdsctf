#!/usr/bin/env fish

set -q REMOTE_USER; or set REMOTE_USER vscode

sudo apt-get update
sudo apt-get install -y --no-install-recommends clang lld
sudo rm -rf /var/lib/apt/lists/*

set fish_path (command -v fish)
echo $fish_path | sudo tee -a /etc/shells
sudo chsh -s $fish_path $REMOTE_USER

cargo install cargo-upgrades 2>/dev/null; or true
pnpm add -g npm-check-updates 2>/dev/null; or npm install -g npm-check-updates 2>/dev/null; or true
