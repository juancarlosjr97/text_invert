use assert_cmd::Command;

#[test]
fn test_main_valid_input() {
    let mut cmd = Command::cargo_bin("reverse_word_cli").unwrap();
    cmd.arg("--word=jcbd").assert().success().stdout("dbcj\n");
}

#[test]
fn test_main_empty_input() {
    let mut cmd = Command::cargo_bin("reverse_word_cli").unwrap();
    cmd.arg("--word")
        .assert()
        .failure()
        .stderr("error: a value is required for '--word <WORD>' but none was supplied\n\nFor more information, try '--help'.\n");
}
