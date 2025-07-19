# How to contribute to CdsCTF

If you're reading this, chances are you're thinking about contributing to CdsCTF, right?
Thanks for your kind intention!
But let's make sure we play by the same rules.

## Issues

If you have an idea for a new feature or a bug fix, please open an issue first.
Specially, if you found a vulnerability, please check our [Security Policy](https://github.com/elabosak233/cdsctf/security/policy) first.

Please describe your requirements or the problems you encounter as detailed as possible.

## Pull Requests

If you have some code to contribute, please fork this repository and open a draft pull request, so we can discuss the changes.

However, please ensure first that your modifications do not conflict with the main repository.
If there is, please handle the conflicts by yourself through the method of sync fork before we can merge your pull request.

If you still have some ideas to CdsCTF, you can keep your CdsCTF repository under your own namespace.
When there are no uncommitted changes locally, you can synchronize the contents of the main repository through the following commands:

```bash
git fetch upstream
```

```bash
git reset --hard upstream/main
```

And then force push your local changes to your remote repository.

Of course, we can do it easier, delete your fork and fork again when you need.