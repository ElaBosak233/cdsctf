import subprocess

from cmd import cmd, gen_params, swag_init

if __name__ == "__main__":
    cmd(swag_init())
    if subprocess.call(f"go build {gen_params()} github.com/elabosak233/pgshub/cmd/pgshub", shell=True) == 0:
        print("Build Finished.")