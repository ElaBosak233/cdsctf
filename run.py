import subprocess

from cmd import cmd, gen_params, swag_init

if __name__ == "__main__":
    cmd(swag_init())
    try:
        subprocess.call(f"go run {gen_params()} github.com/elabosak233/pgshub/cmd/pgshub", shell=True)
    except KeyboardInterrupt:
        print("Run Finished.")
