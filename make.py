#!/usr/bin/env python3

import subprocess
import sys
import time
import os

os.environ["TERM"] = "xterm-256color"
os.environ["CGO_ENABLED"] = "1"


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


def shell(s):
    p = subprocess.Popen(s, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout = p.communicate()[0].decode("utf-8").strip()
    return stdout


def gen_params():
    build_flag = []
    version = Git.get_last_tag()
    if version:
        build_flag.append("-X 'main.Version={}'".format(version))
    branch_name = Git.get_branch()
    if branch_name:
        build_flag.append("-X 'main.Branch={}'".format(branch_name))
    commit_id = Git.get_last_commit_id()
    if commit_id:
        build_flag.append("-X 'main.GitCommitID={}'".format(commit_id))
    build_flag.append("-X 'main.AppBuildAt={}'".format(time.strftime("%Y-%m-%d %H:%M %z")))
    return "-ldflags \"{}\"".format(" ".join(build_flag))


def swag_init():
    return "swag init -g ./main.go -o ./docs"


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "build":
        subprocess.call(swag_init(), shell=True)
        if subprocess.call(f"go build {gen_params()} github.com/elabosak233/pgshub", shell=True) == 0:
            print("Build Finished.")
    elif len(sys.argv) > 1 and sys.argv[1] == "run":
        os.environ["DEBUG"] = "true"
        subprocess.call(swag_init(), shell=True)
        try:
            subprocess.call(f"go run {gen_params()} github.com/elabosak233/pgshub", shell=True)
        except KeyboardInterrupt:
            print("Run Finished.")
    else:
        print("Usage: make.py build|run")
