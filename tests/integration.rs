use std::borrow::Cow;

#[test]
fn test_output() {
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
