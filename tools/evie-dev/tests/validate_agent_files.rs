mod common;

use common::TempRepo;
use evie_dev::agent_files::{Problem, validate};

const SKILL: &str = "---\nname: good\ndescription: Does one thing. Use when testing.\n---\n\n# Good\n\nRead [the reference](references/guide.md).\n";
const SUBAGENT: &str = "---\nname: helper\ndescription: Helps.\ntools: Read\nskills:\n  - good\n---\n\nUse the `good` skill.\n";

/// A repository whose agent files pass every rule.
fn valid_repo() -> TempRepo {
    let repo = TempRepo::new();
    repo.write(".agents/skills/good/SKILL.md", SKILL);
    repo.write(
        ".agents/skills/good/references/guide.md",
        "# Guide\n\nBack to [the skill](../SKILL.md#good), see [the root](/AGENTS.md).\n\n```md\n[in a fence](missing.md)\n```\n\nA span: `[x](missing.md)`. A URL: [site](https://example.com). An anchor: [top](#guide).\n",
    );
    repo.symlink("../.agents/skills", ".claude/skills");
    repo.write(".claude/agents/helper.md", SUBAGENT);
    repo.write(
        "AGENTS.md",
        "# Root\n\n[Project](apps/web/AGENTS.md), [a file with a space](docs/my%20notes.md).\n\n[ref]: <docs/my notes.md>\n",
    );
    repo.symlink("AGENTS.md", "CLAUDE.md");
    repo.write("apps/web/AGENTS.md", "# web\n");
    repo.symlink("AGENTS.md", "apps/web/CLAUDE.md");
    repo.write("docs/my notes.md", "notes\n");
    repo
}

fn problems(repo: &TempRepo) -> Vec<Problem> {
    validate(repo.path()).unwrap()
}

fn skill(name: &str, description: &str, extra: &str) -> String {
    format!("---\nname: {name}\ndescription: {description}\n{extra}---\n\n# Skill\n")
}

fn lines(count: usize) -> String {
    "line\n".repeat(count)
}

/// One broken rule, injected into the valid repository.
struct Fault {
    name: &'static str,
    inject: fn(&TempRepo),
    path: &'static str,
    message: &'static str,
}

const FAULTS: &[Fault] = &[
    Fault {
        name: "skill without SKILL.md",
        inject: |r| r.write(".agents/skills/empty/notes.txt", "x\n"),
        path: ".agents/skills/empty/SKILL.md",
        message: "Missing required file: SKILL.md",
    },
    Fault {
        name: "skill without frontmatter",
        inject: |r| r.write(".agents/skills/good/SKILL.md", "# Good\n"),
        path: ".agents/skills/good/SKILL.md",
        message: "SKILL.md must start with YAML frontmatter (---)",
    },
    Fault {
        name: "unclosed frontmatter",
        inject: |r| r.write(".agents/skills/good/SKILL.md", "---\nname: good\n"),
        path: ".agents/skills/good/SKILL.md",
        message: "SKILL.md frontmatter not properly closed with ---",
    },
    Fault {
        name: "flow style in the frontmatter",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &skill("good", "x", "allowed-tools: [Read]\n"),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "Invalid YAML in frontmatter: ",
    },
    Fault {
        name: "frontmatter that isn't a mapping",
        inject: |r| r.write(".agents/skills/good/SKILL.md", "---\n- good\n---\n"),
        path: ".agents/skills/good/SKILL.md",
        message: "SKILL.md frontmatter must be a YAML mapping",
    },
    Fault {
        name: "a key the spec doesn't allow",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &skill("good", "x", "tools: Read\n"),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "Unexpected fields in frontmatter: tools. Only ['allowed-tools', 'compatibility', 'description', 'license', 'metadata', 'name'] are allowed.",
    },
    Fault {
        name: "missing name",
        inject: |r| r.write(".agents/skills/good/SKILL.md", "---\ndescription: x\n---\n"),
        path: ".agents/skills/good/SKILL.md",
        message: "Missing required field in frontmatter: name",
    },
    Fault {
        name: "empty name",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("''", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Field 'name' must be a non-empty string",
    },
    Fault {
        name: "name over 64 characters",
        inject: |r| {
            let name = "a".repeat(65);
            r.write(
                &format!(".agents/skills/{name}/SKILL.md"),
                &skill(&name, "x", ""),
            );
        },
        path: ".agents/skills/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/SKILL.md",
        message: "exceeds 64 character limit (65 chars)",
    },
    Fault {
        name: "uppercase name",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("Good", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Skill name 'Good' must be lowercase",
    },
    Fault {
        name: "name with a leading hyphen",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("-good", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Skill name cannot start or end with a hyphen",
    },
    Fault {
        name: "name with a double hyphen",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("go--od", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Skill name cannot contain consecutive hyphens",
    },
    Fault {
        name: "name with an underscore",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("go_od", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Skill name 'go_od' contains invalid characters. Only letters, digits, and hyphens are allowed.",
    },
    Fault {
        name: "name that differs from the directory",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("other", "x", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Directory name 'good' must match skill name 'other'",
    },
    Fault {
        name: "non-ASCII name the spec allows",
        inject: |r| r.write(".agents/skills/café/SKILL.md", &skill("café", "x", "")),
        path: ".agents/skills/café/SKILL.md",
        message: "skill name 'café' may only use a-z, 0-9 and single hyphens",
    },
    Fault {
        name: "missing description",
        inject: |r| r.write(".agents/skills/good/SKILL.md", "---\nname: good\n---\n"),
        path: ".agents/skills/good/SKILL.md",
        message: "Missing required field in frontmatter: description",
    },
    Fault {
        name: "empty description",
        inject: |r| r.write(".agents/skills/good/SKILL.md", &skill("good", "", "")),
        path: ".agents/skills/good/SKILL.md",
        message: "Field 'description' must be a non-empty string",
    },
    Fault {
        name: "description over 1024 characters",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &skill("good", &"d".repeat(1025), ""),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "Description exceeds 1024 character limit (1025 chars)",
    },
    Fault {
        name: "compatibility over 500 characters",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &skill(
                    "good",
                    "x",
                    &format!("compatibility: {}\n", "c".repeat(501)),
                ),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "Compatibility exceeds 500 character limit (501 chars)",
    },
    Fault {
        name: "compatibility that isn't a string",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &skill("good", "x", "compatibility:\n  os: linux\n"),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "Field 'compatibility' must be a string",
    },
    Fault {
        name: "SKILL.md of 500 lines",
        inject: |r| {
            r.write(
                ".agents/skills/good/SKILL.md",
                &format!("{SKILL}{}", lines(500 - 8)),
            )
        },
        path: ".agents/skills/good/SKILL.md",
        message: "has 500 lines; the budget is under 500",
    },
    Fault {
        name: "broken link in a skill reference",
        inject: |r| {
            r.write(
                ".agents/skills/good/references/guide.md",
                "\n\n[gone](gone.md)\n",
            )
        },
        path: ".agents/skills/good/references/guide.md:3",
        message: "broken link to gone.md",
    },
    Fault {
        name: "broken root-relative link",
        inject: |r| r.write("apps/web/AGENTS.md", "[gone](/nowhere/AGENTS.md)\n"),
        path: "apps/web/AGENTS.md:1",
        message: "broken link to /nowhere/AGENTS.md",
    },
    Fault {
        name: "broken reference definition",
        inject: |r| r.write("apps/web/AGENTS.md", "[gone]: ./gone.md\n"),
        path: "apps/web/AGENTS.md:1",
        message: "broken link to ./gone.md",
    },
    Fault {
        name: "subagent of 100 lines",
        inject: |r| {
            r.write(
                ".claude/agents/helper.md",
                &format!("{SUBAGENT}{}", lines(100 - 9)),
            )
        },
        path: ".claude/agents/helper.md",
        message: "has 100 lines; the budget is under 100",
    },
    Fault {
        name: "subagent without frontmatter",
        inject: |r| r.write(".claude/agents/helper.md", "name: helper\n"),
        path: ".claude/agents/helper.md",
        message: "has no YAML frontmatter between --- lines",
    },
    Fault {
        name: "subagent without a name",
        inject: |r| r.write(".claude/agents/helper.md", "---\ndescription: x\n---\n"),
        path: ".claude/agents/helper.md",
        message: "frontmatter is missing 'name'",
    },
    Fault {
        name: "subagent with an invalid name",
        inject: |r| {
            r.write(
                ".claude/agents/helper.md",
                "---\nname: Helper_1\ndescription: x\n---\n",
            )
        },
        path: ".claude/agents/helper.md",
        message: "subagent name 'Helper_1' must be 1-64 chars of a-z, 0-9 and single hyphens",
    },
    Fault {
        name: "subagent name that differs from the file name",
        inject: |r| {
            r.write(
                ".claude/agents/helper.md",
                "---\nname: other\ndescription: x\n---\n",
            )
        },
        path: ".claude/agents/helper.md",
        message: "subagent name 'other' must equal the file name 'helper'",
    },
    Fault {
        name: "subagent without a description",
        inject: |r| {
            r.write(
                ".claude/agents/helper.md",
                "---\nname: helper\ndescription: ''\n---\n",
            )
        },
        path: ".claude/agents/helper.md",
        message: "frontmatter is missing a non-empty 'description'",
    },
    Fault {
        name: "broken link in a subagent",
        inject: |r| {
            r.write(
                ".claude/agents/helper.md",
                &format!("{SUBAGENT}[gone](gone.md)\n"),
            )
        },
        path: ".claude/agents/helper.md:10",
        message: "broken link to gone.md",
    },
    Fault {
        name: "AGENTS.md of 200 lines",
        inject: |r| r.write("apps/web/AGENTS.md", &lines(200)),
        path: "apps/web/AGENTS.md",
        message: "has 200 lines; the budget is under 200",
    },
    Fault {
        name: "AGENTS.md without CLAUDE.md",
        inject: |r| r.write("libs/x/AGENTS.md", "# x\n"),
        path: "libs/x/AGENTS.md",
        message: "has no CLAUDE.md beside it; add one with: ln -s AGENTS.md libs/x/CLAUDE.md",
    },
    Fault {
        name: "CLAUDE.md as a regular file",
        inject: |r| {
            r.write("libs/x/AGENTS.md", "# x\n");
            r.write("libs/x/CLAUDE.md", "# x\n");
        },
        path: "libs/x/CLAUDE.md",
        message: "must be a symlink to AGENTS.md, not a regular file",
    },
    Fault {
        name: "CLAUDE.md pointing elsewhere",
        inject: |r| {
            r.write("libs/x/AGENTS.md", "# x\n");
            r.symlink("../../AGENTS.md", "libs/x/CLAUDE.md");
        },
        path: "libs/x/CLAUDE.md",
        message: "must point to AGENTS.md in the same directory, not ../../AGENTS.md",
    },
    Fault {
        name: "CLAUDE.md without an AGENTS.md",
        inject: |r| r.symlink("AGENTS.md", "libs/x/CLAUDE.md"),
        path: "libs/x/CLAUDE.md",
        message: "points to AGENTS.md, which doesn't exist",
    },
    Fault {
        name: "broken symlink",
        inject: |r| r.symlink("nowhere", "docs/link"),
        path: "docs/link",
        message: "broken symlink to nowhere",
    },
    Fault {
        name: ".claude/skills as a directory",
        inject: |r| {
            std::fs::remove_file(r.join(".claude/skills")).unwrap();
            r.write(".claude/skills/README.md", "x\n");
        },
        path: ".claude/skills",
        message: "must be a symlink to ../.agents/skills",
    },
    Fault {
        name: ".claude/skills pointing elsewhere",
        inject: |r| {
            std::fs::remove_file(r.join(".claude/skills")).unwrap();
            r.symlink("../docs", ".claude/skills");
        },
        path: ".claude/skills",
        message: "must point to .agents/skills, not ../docs",
    },
];

#[test]
fn accepts_valid_agent_files() {
    let repo = valid_repo();

    assert_eq!(problems(&repo), []);
}

#[test]
fn reports_each_injected_fault() {
    for fault in FAULTS {
        let repo = valid_repo();
        (fault.inject)(&repo);

        let problems = problems(&repo);

        assert!(
            problems
                .iter()
                .any(|p| p.path == fault.path && p.message.contains(fault.message)),
            "{}: expected {}: {}, got {problems:#?}",
            fault.name,
            fault.path,
            fault.message
        );
    }
}

#[test]
fn accepts_names_and_lengths_at_their_limits() {
    let repo = valid_repo();
    let name = "a".repeat(64);
    repo.write(
        &format!(".agents/skills/{name}/SKILL.md"),
        &skill(
            &name,
            &"d".repeat(1024),
            &format!(
                "compatibility: {}\nlicense: MIT\nmetadata:\n  a: b\nallowed-tools: Read Bash\n",
                "c".repeat(500)
            ),
        ),
    );
    repo.write(
        ".agents/skills/good/SKILL.md",
        &format!("{SKILL}{}", lines(499 - 8)),
    );
    repo.write("apps/web/AGENTS.md", &lines(199));

    assert_eq!(problems(&repo), []);
}

#[test]
fn skips_files_that_git_ignores() {
    let repo = valid_repo();
    repo.write(".gitignore", "ignored/\n");
    repo.write("ignored/AGENTS.md", "[gone](gone.md)\n");

    assert_eq!(problems(&repo), []);
}

#[test]
fn binary_exits_one_and_counts_the_problems() {
    let repo = valid_repo();
    repo.write(".claude/agents/helper.md", "name: helper\n");

    let output = repo.evie_dev(&["validate-agent-files"]);
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr.starts_with(".claude/agents/helper.md: has no YAML frontmatter between --- lines\n")
    );
    assert!(stderr.ends_with(
        "\n1 problem(s) found. See the budgets in .agents/skills/authoring-skills/SKILL.md.\n"
    ));
}

#[test]
fn binary_exits_zero_for_valid_files() {
    let repo = valid_repo();

    let output = repo.evie_dev(&["validate-agent-files"]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Agent files are valid.\n"
    );
}
