use std::process::Command;

#[test]
fn feat_8_short_version_flag_prints_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_fd"))
        .arg("-V")
        .output()
        .expect("fd runs");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.starts_with("fd "), "unexpected output: {text}");
}
