#!/usr/bin/env python3

import subprocess
import time
import sys
import os

os.environ["TERM"] = "xterm-256color"


class Git:

    @staticmethod
    def get_last_tag():
        return cmd("git describe --abbrev=0 --tags")

    @staticmethod
    def get_branch():
        return cmd("git rev-parse --abbrev-ref HEAD")

    @staticmethod
    def get_last_commit_id():
        return cmd("git rev-parse HEAD")


def cmd(s):
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
        build_flag.append("-X 'main.CommitId={}'".format(commit_id))
    build_flag.append("-X 'main.BuildAt={}'".format(time.strftime("%Y-%m-%d %H:%M %z")))
    return "-ldflags \"{}\"".format(" ".join(build_flag))


def swag_init():
    return "swag init -g ./cmd/pgshub/main.go -o ./docs"


def go_build():
    return f"go build {gen_params()} github.com/elabosak233/pgshub/cmd/pgshub"


def go_run():
    return f"go run {gen_params()} github.com/elabosak233/pgshub/cmd/pgshub"


if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "build":
            cmd(swag_init())
            if subprocess.call(go_build(), shell=True) == 0:
                print("Build Finished.")
    else:
        cmd(swag_init())
        try:
            subprocess.call(go_run(), shell=True)
        except KeyboardInterrupt:
            print("Run Finished.")
