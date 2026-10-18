---
name: bump-version
description: Prepare and publish a CdsCTF release by bumping the Rust workspace version, creating the release commit and signed tag, then pushing only after the user reviews the result. Use for CdsCTF releases, not dependency upgrades.
---

# Bump Version

Use this skill for the complete CdsCTF release workflow. The user may request a `patch`, `minor`, `major`, or explicit `X.Y.Z` target. If the release level is not clear from the request and recent changes, explain the recommended version and ask before changing files.

## Project rules

- The version source is `[workspace.package].version` in the repository root `Cargo.toml`. Crates inherit it with `version = { workspace = true }`; do not edit each crate manifest or `web/package.json`.
- Keep `Cargo.lock` consistent with the workspace version. Run Cargo to update and verify it rather than editing generated package entries by hand.
- Use stable semantic versions in the exact `X.Y.Z` form. Tags have no `v` prefix.
- Create an annotated, GPG-signed tag named exactly `X.Y.Z`; its fixed message is `Release X.Y.Z` (for example, tag `1.13.0` has message `Release 1.13.0`). Stop if signing fails; do not silently create an unsigned tag.
- The release tag must point to the version-bump commit on `main`; do not tag or publish an unmerged feature branch. Start from an up-to-date `origin/main` so the release commit can be pushed with the tag after review.
- Pushing a version tag triggers `.github/workflows/publish.yml` and publishes release Docker image tags. Treat push as the release/publish action.

## Release-level compatibility check

Before selecting or changing the version, compare the candidate release with the latest stable version tag. Review all commits since that tag and any candidate changes, focusing on `crates/migrator/src/migrations/**`, the migration registry, `crates/db/src/entity/**`, SQL/schema definitions, and externally consumed data contracts.

- A new or changed database migration, persisted data shape, or other structural data contract is a strong signal that the release must be at least `minor`. Do not accept a `patch` target in that case; explain the evidence and recommend the next minor version.
- If a change removes or incompatibly changes stored fields, tables, types, required API fields, or data semantics, classify it as breaking and require `major` unless the change is demonstrably backward-compatible through an expand-and-contract migration.
- A `patch` target is appropriate only when the reviewed changes are backward-compatible fixes with no meaningful schema or data-contract change.
- Do not infer safety from filenames alone. Inspect migration `up`/`down` behavior and relevant entity/API changes. If compatibility is unclear, do not approve a patch; report the uncertainty and recommend at least minor.

## Workflow

1. Inspect `git status`, current branch, root `Cargo.toml`, relevant recent commits, existing tags, and the latest `origin/main` state. On `main`, require the branch to be up to date with `origin/main` before editing. Do not proceed over pre-existing edits to `Cargo.toml` or `Cargo.lock`; preserve unrelated user changes and do not include them in the release commit.
2. Determine the target version from the current workspace version and the requested release level. Apply the compatibility check above before editing. If the requested target is below the required level, stop before changing files and explain the evidence and minimum acceptable version. Ensure the target tag does not already exist locally or on `origin`.
3. Update only the root workspace version, then run `cargo check --workspace` to refresh and validate `Cargo.lock`. Review `git diff --check` and the complete diff. Stop if Cargo changes unrelated dependencies or files; investigate instead of committing that churn.
4. Create a release commit containing only `Cargo.toml` and `Cargo.lock`, with message `chore(release): bump version to X.Y.Z`.
5. If the current branch is not `main`, leave the release commit ready for the normal merge process and do not tag or push it. After the user merges it to `main`, resume from the tag and verification steps. On `main`, confirm the release commit is the only new commit ahead of `origin/main`.
6. Create the local signed annotated tag with the exact fixed message. Verify that the tag points to the release commit, its message is exact, its signature verifies, the manifest and lockfile report the target version, and the commit is on local `main`.
7. Stop before any push. Report the version change, the compatibility assessment and evidence, commit, tag, checks, and the exact refs that would be pushed. Ask the user to inspect this prepared release. Do not push until the user has reviewed it and explicitly authorizes the push.
8. After authorization, re-check the remote refs and worktree. If the release commit is ahead of `origin/main`, atomically push `main` and the release tag together; if the commit is already on `origin/main`, push only the tag. Never force-push or overwrite an existing tag. Verify the remote tag object and peeled commit after the push.

Keep the release commit limited to version files. Do not create GitHub Releases, alter old tags, or change GitHub Actions settings unless the user separately asks.
