#!/usr/bin/env python3

import subprocess
import time
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
