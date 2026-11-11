use nu_utils::escape_quote_string;

/// Characters that change how a bare word is parsed, so an argument holding one is quoted.
///
/// `#` starts a comment when it begins a token (or after whitespace). Without quoting,
/// `nu script.nu #000000` becomes `main #000000` and the argument is stripped.
fn is_special_char(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '(' | '[' | '{' | '}' | '\'' | '`' | '"' | '\\' | ';' | '|' | '#'
        )
}

fn string_should_be_quoted(input: &str) -> bool {
    input.is_empty() || input.starts_with('$') || input.chars().any(is_special_char)
}

/// `input` as a double-quoted string if it needs quoting, otherwise unchanged.
fn quote_if_needed(input: &str) -> String {
    if string_should_be_quoted(input) {
        escape_quote_string(input)
    } else {
        input.into()
    }
}

/// Escapes a command-line argument so it survives being re-parsed as `main <arg>`.
///
/// - Not a flag, no leading `$` and no special characters: passed as is (`foo` -> `foo`).
/// - Not a flag, and starts with `$` or has a special character: quoted, with `"` and `\`
///   escaped (`two \words` -> `"two \\words"`).
/// - A flag without `=`: passed as is (`--foo` -> `--foo`).
/// - A flag with `=`: the first two rules apply to the value
///   (`--foo=bar` -> `--foo=bar`; `--foo=bar'` -> `--foo="bar'"`).
///
/// Special characters are white space, `(`, `[`, `{`, `}`, `'`, `` ` ``, `"`, `\`, `;`, `|`
/// and `#`.
pub fn escape_for_script_arg(input: &str) -> String {
    if !input.starts_with("--") {
        return quote_if_needed(input);
    }
    // A flag: only its value, if it has one, may need escaping.
    match input.split_once('=') {
        Some((arg_name, arg_val)) => format!("{arg_name}={}", quote_if_needed(arg_val)),
        None => input.into(),
    }
}

#[cfg(test)]
mod test {
    use super::escape_for_script_arg;

    #[test]
    fn test_not_extra_quote() {
        // check for input arg like this:
        // nu b.nu word 8
        assert_eq!(escape_for_script_arg("word"), "word".to_string());
        assert_eq!(escape_for_script_arg("8"), "8".to_string());
    }

    #[test]
    fn test_quote_special() {
        let cases = vec![
            ("two words", r#""two words""#),
            ("$nake", r#""$nake""#),
            ("`123", r#""`123""#),
            ("this|cat", r#""this|cat""#),
            ("this;cat", r#""this;cat""#),
            // `#` would start a comment when re-parsed as `main <arg>`
            ("#000000", r##""#000000""##),
            ("#", r##""#""##),
            ("foo#bar", r##""foo#bar""##),
        ];

        for (input, expected) in cases {
            assert_eq!(escape_for_script_arg(input).as_str(), expected);
        }
    }

    #[test]
    fn test_flag_value_with_hash() {
        assert_eq!(
            escape_for_script_arg("--color=#000000"),
            r##"--color="#000000""##.to_string()
        );
    }

    #[test]
    fn test_quote_newline() {
        assert_eq!(escape_for_script_arg("c\nd"), format!("\"c\nd\""));
    }

    #[test]
    fn test_arg_with_flag() {
        // check for input arg like this:
        // nu b.nu --linux --version=v5.2
        assert_eq!(escape_for_script_arg("--linux"), "--linux".to_string());
        assert_eq!(
            escape_for_script_arg("--version=v5.2"),
            "--version=v5.2".to_string()
        );

        // check for input arg like this:
        // nu b.nu linux --version v5.2
        assert_eq!(escape_for_script_arg("--version"), "--version".to_string());
        assert_eq!(escape_for_script_arg("v5.2"), "v5.2".to_string());
    }

    #[test]
    fn test_flag_arg_with_values_contains_special() {
        // check for input arg like this:
        // nu b.nu test_ver --version='xx yy' --separator="`"
        assert_eq!(
            escape_for_script_arg("--version='xx yy'"),
            r#"--version="'xx yy'""#.to_string()
        );
        assert_eq!(
            escape_for_script_arg("--separator=`"),
            r#"--separator="`""#.to_string()
        );
    }

    #[test]
    fn test_escape() {
        // check for input arg like this:
        // nu b.nu \ --arg='"'
        assert_eq!(escape_for_script_arg(r"\"), r#""\\""#.to_string());
        assert_eq!(
            escape_for_script_arg(r#"--arg=""#),
            r#"--arg="\"""#.to_string()
        );
    }
}
