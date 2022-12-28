use assert_cmd::Command;

#[test]
fn noperm() {
    let mut cmd = Command::cargo_bin("fchmodexec").unwrap();
    let output = cmd
        .args(["000", "0", "--", "stat", "--printf=%a", "-L", "/dev/stdin"])
        .assert();
    output.success().stdout("0");
}

#[test]
fn allperm() {
    let mut cmd = Command::cargo_bin("fchmodexec").unwrap();
    let output = cmd
        .args(["777", "0", "--", "stat", "--printf=%a", "-L", "/dev/stdin"])
        .assert();
    output.success().stdout("777");
}
