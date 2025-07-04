use assert_cmd::Command;

#[test]
fn test_main_valid_input() {
    let mut cmd = Command::cargo_bin("text_invert").unwrap();
    cmd.arg("--word=jcbd").assert().success().stdout("dbcj\n");
}

#[test]
fn test_main_empty_input() {
    let mut cmd = Command::cargo_bin("text_invert").unwrap();
    cmd.arg("--word")
        .assert()
        .failure()
        .stderr("error: a value is required for '--word <WORD>' but none was supplied\n\nFor more information, try '--help'.\n");
}

#[test]
fn test_main_palindrome_flag() {
    let mut cmd = Command::cargo_bin("text_invert").unwrap();
    cmd.args(["--word=racecar", "--palindrome"]).assert().success().stdout("racecar is a palindrome\n");

    let mut cmd = Command::cargo_bin("text_invert").unwrap();
    cmd.args(["--word=hello", "-p"]).assert().success().stdout("hello is not a palindrome\n");
}
