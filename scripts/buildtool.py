#!/usr/bin/env python3

import subprocess
import sys
import time
import os

os.environ["TERM"] = "xterm-256color"
os.environ["CGO_ENABLED"] = "1"

package = "github.com/elabosak233/cloudsdale"


class Git:

    @staticmethod
    def get_last_tag():
        return shell("git describe --abbrev=0 --tags")

    @staticmethod
    def get_branch():
        return shell("git rev-parse --abbrev-ref HEAD")

    @staticmethod
    def get_last_commit_id():
        return shell("git rev-parse HEAD")


class Swag:
    @staticmethod
    def init():
        subprocess.call("swag init -g ./cloudsdale.go -o ./docs", shell=True)


class Go:
    @staticmethod
    def run():
        os.environ["DEBUG"] = "true"
        Swag.init()
        try:
            subprocess.call(f"go run {gen_params(build=False)} {package}", shell=True)
        except KeyboardInterrupt:
            print("Run Finished.")

    @staticmethod
    def build():
        Swag.init()
        if subprocess.call(f"go build {gen_params()} -o ./build/ {package}", shell=True) == 0:
            print("Build Finished.")


def shell(s):
    p = subprocess.Popen(s, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout = p.communicate()[0].decode("utf-8").strip()
    return stdout


def gen_params(build=True):
    build_flag = []

    version = Git.get_last_tag()
    if version:
        build_flag.append(f"-X '{package}/internal/global.Version={version}'")

    branch_name = Git.get_branch()
    if branch_name:
        build_flag.append(f"-X '{package}/internal/global.Branch={branch_name}'")

    commit_id = Git.get_last_commit_id()
    if commit_id:
        build_flag.append(f"-X '{package}/internal/global.GitCommitID={commit_id}'")

    build_flag.append(f"-X '{package}/internal/global.BuildAt={time.strftime('%Y-%m-%d %H:%M %z')}'")

    if build:
        return "-ldflags \"-linkmode external -w -s {}\"".format(" ".join(build_flag))
    else:
        return "-ldflags \"{}\"".format(" ".join(build_flag))


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "build":
        Go.build()
    elif len(sys.argv) > 1 and sys.argv[1] == "run":
        Go.run()
    else:
        print("Usage: buildtool.py build|run")
