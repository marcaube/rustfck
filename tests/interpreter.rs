use std::process::Command;


#[test]
fn test_hello_world() {
    let output = Command::new("./target/debug/rustfck")
        .arg("examples/hello_world.bf")
        .output()
        .expect("Failed to execute command");

    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello World!\n");
}
