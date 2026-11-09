---
name: ship
description: Takes finished work on a branch to an open PR by verifying, committing, verifying again as CI does, and opening the PR. Use when work for an issue is done and ready for review, or when asked to ship, wrap up or deliver a change.
---

# Ship

A workflow: each step uses one skill. If a step fails, stop and return that step's report; don't go on to the next step.

## Steps

1. Use the `verify` skill at the uncommitted scope.
2. Use the `commit` skill.
3. Use the `verify` skill at the CI scope.
4. Use the `pull-request` skill.

## Output

The PR URL from the `pull-request` skill, or the report of the step that failed.
