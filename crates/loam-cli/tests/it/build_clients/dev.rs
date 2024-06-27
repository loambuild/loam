use crate::util::TestEnv;
use std::process::Stdio;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn dev_command_watches_for_changes_and_environments_toml() {
    TestEnv::from_async("soroban-init-boilerplate", |env| async move {
        env.set_environments_toml(
            r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

"#,
        );

        let mut dev_process = env
            .loam_process("dev")
            .current_dir(&env.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn dev process");

        // Give the dev process some time to start and perform initial build
        sleep(Duration::from_secs(20)).await;

        // Test 1: Modify a source file
        let file_changed = "contracts/hello_world/src/lib.rs";
        env.modify_file(
            file_changed,
            "// This is a test modification",
        );
        let file_changed_path = env.cwd.join(file_changed);

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(5)).await;

        // Test 2: Create and modify environments.toml
        env.set_environments_toml(
            r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.workspace = true
soroban_increment_contract.workspace = true
"#,
        );

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(5)).await;

        // Modify environments.toml again
        env.set_environments_toml(
            r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:9000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts]
hello_world.workspace = true
soroban_increment_contract.workspace = true
"#,
        );

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(5)).await;

        dev_process.kill().await.expect("Failed to kill dev process");

        let output = dev_process.wait_with_output().await.expect("Failed to wait for dev process");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        
        // Check for file changes
        assert!(stdout.contains(&format!("File changed: {:?}", file_changed_path)));

        // Check for network changes
        assert!(stderr.contains("🌐 using network at http://localhost:8000/rpc"));
        assert!(stderr.contains("🌐 using network at http://localhost:9000/rpc"));
    })
    .await;
}
