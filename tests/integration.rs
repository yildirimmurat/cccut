use std::borrow::Cow;

#[test]
fn test_field_option() {
    let expected = "\
f1\n\
1\n\
6\n\
11\n\
16\n\
21";

    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-f2")
        .arg("tests/sample.tsv")
        .output()
        .expect("failed to execute command");

    let binding: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let actual: &str = binding.trim();

    assert_eq!(expected, actual);
}

#[test]
fn test_field_with_delimiter_option() {
    let expected = "\
Song title\n\
\"10000 Reasons (Bless the Lord)\"\n\
\"20 Good Reasons\"\n\
\"Adore You\"\n\
\"Africa\"";

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg("cargo run -- -f1 -d, tests/fourchords.csv | head -n5")
        .output()
        .expect("failed to execute command");

    let binding: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let actual: &str = binding.trim_start_matches("\u{feff}").trim(); // Remove BOM character at the start

    assert_eq!(expected, actual);
}
#[test]
fn test_field_with_multiple_columns() {
    let expected = "\
Song title,Artist\n\
\"10000 Reasons (Bless the Lord)\",Matt Redman and Jonas Myrin\n\
\"20 Good Reasons\",Thirsty Merc\n\
\"Adore You\",Harry Styles\n\
\"Africa\",Toto";

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg("cargo run -- -f \"1 2\" -d, tests/fourchords.csv | head -n5")
        .output()
        .expect("failed to execute command");

    let binding: Cow<str> = String::from_utf8_lossy(&output.stdout);
    let actual: &str = binding.trim_start_matches("\u{feff}").trim(); // Remove BOM character at the start

    assert_eq!(expected, actual);
}
