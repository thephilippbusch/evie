//! Shell glob patterns, as bash matches them in `case` and `[[ == ]]`:
//! `*` matches any text, `/` included, `?` one character, `[...]` a set
//! (`[!...]` or `[^...]` negated, with ranges and `[:class:]`), and `\`
//! escapes the next character.

/// Whether `text` matches `pattern` as a whole.
pub fn matches(pattern: &str, text: &str, ignore_case: bool) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();
    matches_at(&pattern, &text, ignore_case)
}

fn matches_at(pattern: &[char], text: &[char], ignore_case: bool) -> bool {
    let Some((&first, rest)) = pattern.split_first() else {
        return text.is_empty();
    };
    match first {
        '*' => (0..=text.len()).any(|skip| matches_at(rest, &text[skip..], ignore_case)),
        '?' => !text.is_empty() && matches_at(rest, &text[1..], ignore_case),
        '[' => match bracket(rest) {
            Some((set, after)) => text.split_first().is_some_and(|(&c, text_rest)| {
                set.contains(c, ignore_case) && matches_at(after, text_rest, ignore_case)
            }),
            None => literal('[', rest, text, ignore_case),
        },
        '\\' => match rest.split_first() {
            Some((&escaped, after)) => literal(escaped, after, text, ignore_case),
            None => literal('\\', rest, text, ignore_case),
        },
        c => literal(c, rest, text, ignore_case),
    }
}

fn literal(c: char, pattern: &[char], text: &[char], ignore_case: bool) -> bool {
    text.split_first().is_some_and(|(&t, text_rest)| {
        same(c, t, ignore_case) && matches_at(pattern, text_rest, ignore_case)
    })
}

fn same(a: char, b: char, ignore_case: bool) -> bool {
    a == b || (ignore_case && a.to_lowercase().eq(b.to_lowercase()))
}

struct Set {
    negated: bool,
    items: Vec<Item>,
}

enum Item {
    Char(char),
    Range(char, char),
    Class(String),
}

impl Set {
    fn contains(&self, c: char, ignore_case: bool) -> bool {
        let candidates: Vec<char> = if ignore_case {
            c.to_lowercase()
                .chain(c.to_uppercase())
                .chain([c])
                .collect()
        } else {
            vec![c]
        };
        let hit = candidates.iter().any(|&c| {
            self.items.iter().any(|item| match item {
                Item::Char(x) => *x == c,
                Item::Range(low, high) => (*low..=*high).contains(&c),
                Item::Class(name) => in_class(name, c),
            })
        });
        hit != self.negated
    }
}

fn in_class(name: &str, c: char) -> bool {
    match name {
        "alnum" => c.is_alphanumeric(),
        "alpha" => c.is_alphabetic(),
        "digit" => c.is_ascii_digit(),
        "lower" => c.is_lowercase(),
        "upper" => c.is_uppercase(),
        "space" => c.is_whitespace(),
        "blank" => c == ' ' || c == '\t',
        "punct" => c.is_ascii_punctuation(),
        "xdigit" => c.is_ascii_hexdigit(),
        _ => false,
    }
}

/// Parses the set after a `[` and returns it with the rest of the pattern,
/// or `None` when the `[` has no closing `]` and so is a plain character.
fn bracket(pattern: &[char]) -> Option<(Set, &[char])> {
    let mut i = 0;
    let negated = matches!(pattern.first(), Some('!' | '^'));
    if negated {
        i += 1;
    }
    let mut items = Vec::new();
    let mut first = true;
    loop {
        let c = *pattern.get(i)?;
        if c == ']' && !first {
            return Some((Set { negated, items }, &pattern[i + 1..]));
        }
        first = false;
        if c == '[' && pattern.get(i + 1) == Some(&':') {
            let name_start = i + 2;
            if let Some(len) = pattern[name_start..]
                .windows(2)
                .position(|w| w == [':', ']'])
            {
                items.push(Item::Class(
                    pattern[name_start..name_start + len].iter().collect(),
                ));
                i = name_start + len + 2;
                continue;
            }
        }
        let c = if c == '\\' {
            i += 1;
            *pattern.get(i)?
        } else {
            c
        };
        if pattern.get(i + 1) == Some(&'-') && pattern.get(i + 2).is_some_and(|&h| h != ']') {
            items.push(Item::Range(c, pattern[i + 2]));
            i += 3;
        } else {
            items.push(Item::Char(c));
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_matches_any_text_including_slashes() {
        assert!(matches(
            "libs/proto/gen/*",
            "libs/proto/gen/rust/a.rs",
            false
        ));
        assert!(matches("*.env", "config/prod.env", false));
        assert!(matches("*", "", false));
        assert!(!matches("*.env", "prod.envx", false));
    }

    #[test]
    fn question_mark_and_sets_match_one_character() {
        assert!(matches("a?c", "abc", false));
        assert!(!matches("a?c", "ac", false));
        assert!(matches("[a-c]x", "bx", false));
        assert!(matches("[!a-c]x", "dx", false));
        assert!(!matches("[^a-c]x", "ax", false));
        assert!(matches("[[:digit:]]", "7", false));
        assert!(matches("[]]", "]", false));
        assert!(matches("[", "[", false));
    }

    #[test]
    fn backslash_escapes_the_next_character() {
        assert!(matches("\\*", "*", false));
        assert!(!matches("\\*", "x", false));
    }

    #[test]
    fn ignore_case_applies_to_letters_and_ranges() {
        assert!(matches("changelog.md", "CHANGELOG.md", true));
        assert!(!matches("changelog.md", "CHANGELOG.md", false));
        assert!(matches("[a-z]", "Q", true));
    }
}
