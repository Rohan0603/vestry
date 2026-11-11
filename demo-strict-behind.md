# Disposable strict-up-to-date demonstration (VES-4)

This file exists only to demonstrate GitHub's strict up-to-date branch-protection gate.
This branch's content matches `main`, but its history does not contain the latest `main`
merge commit, so the pull request is BEHIND. With `strict: true`, the merge must be
rejected until the branch is updated. This PR is closed without merging and the branch
is deleted; nothing here lands on `main`.
