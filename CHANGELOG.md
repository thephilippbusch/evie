# Changelog

## 0.1.0 (2026-11-10)

The development setup ([milestone v0.1](https://github.com/thephilippbusch/evie/milestone/1)). Agents and people share documented conventions, skills and guardrails, and every release is versioned, tagged and changelogged automatically.

### Development setup

- **Agent entry point:** a root `AGENTS.md`, with a `CLAUDE.md` symlink, and shared skills in `.agents/skills/` for Claude Code, OpenCode and Codex
- **Guardrails:** Claude Code permissions and a hook that block protected actions, moon VCS hooks on every commit, and the moon MCP server for agents
- **Skill conventions:** the `authoring-skills` skill, and a CI check for skills, subagents and `AGENTS.md` files
- **Workflow skills:** git and delivery, moon, `verify`, `writing-style`, `plan` and `check-milestone`
- **Issues and investigation:** the `issue` skill with issue templates, `debug`, `benchmark`, and the Rust and TypeScript conventions
- **Review and subagents:** five focus reviewers that `review` runs in parallel, plus the verifier, milestone auditor and implementer subagents
- **Releases:** Conventional Commit PR titles checked in CI, squash merges only, and release-please for tags, changelog and GitHub releases

### Features

- add the plan and check-milestone skills ([#116](https://github.com/thephilippbusch/evie/issues/116)) ([eea7727](https://github.com/thephilippbusch/evie/commit/eea77278e5edaf0908d307d727f9f6849c60e427))
