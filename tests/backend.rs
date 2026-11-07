use lelwel::compile;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn generate(grammar: &str) -> String {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let out_dir = std::env::temp_dir().join(format!(
        "lelwel_backend_{}_{grammar}_{id}",
        std::process::id()
    ));
    std::fs::create_dir_all(&out_dir).unwrap();
    let input = format!("tests/backend/{grammar}.llw");
    let ok = compile(
        &input,
        out_dir.to_str().unwrap(),
        false,
        false,
        0,
        false,
        false,
        false,
    )
    .unwrap();
    assert!(ok, "failed to generate {input}");
    let code = std::fs::read_to_string(out_dir.join("generated.rs")).unwrap();
    let _ = std::fs::remove_dir_all(&out_dir);
    code
}

fn rule_body<'a>(code: &'a str, rule: &str) -> &'a str {
    let start = code.find(&format!("fn rule_{rule}(")).unwrap();
    let end = code[start + 1..]
        .find("\n    fn ")
        .map_or(code.len(), |i| start + 1 + i);
    &code[start..end]
}

fn expect_lines(body: &str) -> Vec<&str> {
    body.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("expect!("))
        .collect()
}

#[test]
fn unskip_single_skipped_follow() {
    let code = generate("unskip");
    assert_eq!(
        expect_lines(rule_body(&code, "single")),
        [
            r#"expect!(A, "invalid syntax, expected: A", self, diags, &[Token::Newline]);"#,
            r#"expect!(Newline, "invalid syntax, expected: Newline", self, diags, &[]);"#,
        ]
    );
}

#[test]
fn unskip_multiple_skipped_follow() {
    let code = generate("unskip");
    assert_eq!(
        expect_lines(rule_body(&code, "multi")),
        [
            r#"expect!(B, "invalid syntax, expected: B", self, diags, &[Token::Newline,Token::Whitespace]);"#,
            r#"expect!(Newline, "invalid syntax, expected: Newline", self, diags, &[Token::Newline]);"#,
            r#"expect!(Whitespace, "invalid syntax, expected: Whitespace", self, diags, &[Token::Newline]);"#,
        ]
    );
}

#[test]
fn unskip_in_loop() {
    let code = generate("unskip");
    assert_eq!(
        expect_lines(rule_body(&code, "loop")),
        [
            r#"expect!(C, "invalid syntax, expected: C", self, diags, &[Token::Newline]);"#,
            r#"expect!(Newline, "invalid syntax, expected: Newline", self, diags, &[]);"#,
        ]
    );
}

#[test]
fn unskip_empty_without_skipped_follow() {
    let code = generate("unskip");
    assert_eq!(
        expect_lines(rule_body(&code, "no_unskip")),
        [
            r#"expect!(Comma, "invalid syntax, expected: Comma", self, diags, &[]);"#,
            r#"expect!(A, "invalid syntax, expected: A", self, diags, &[]);"#,
        ]
    );
}

#[test]
fn no_unskip_argument_without_skip_usage() {
    let code = generate("unused_skip");
    assert_eq!(
        expect_lines(rule_body(&code, "s")),
        [r#"expect!(A, "invalid syntax, expected: A", self, diags);"#]
    );
}
