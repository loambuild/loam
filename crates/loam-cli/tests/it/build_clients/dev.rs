use crate::util::TestEnv;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::sleep;

#[tokio::test]
async fn dev_command_watches_for_changes() {
    println!("Test function started");

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
            .spawn()
            .expect("Failed to spawn dev process");

        println!("Dev process spawned");

        // Give the dev process some time to start
        sleep(Duration::from_secs(20)).await;

        // Modify a source file
        env.modify_file(
            "contracts/hello_world/src/lib.rs",
            "// This is a test modification",
        );

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(15)).await;
        dev_process.kill().await.expect("Failed to kill dev process");
        let output = dev_process.wait_with_output().await.expect("Failed to wait for dev process");
        let stderr = String::from_utf8(output.stderr).unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        println!("Dev process output err: {}", stderr);
        println!("Dev process output : {}", stdout);
        assert!(stdout.contains("Watching for changes. Press Ctrl+C to stop."));
        assert!(stdout.contains("Changes detected. Rebuilding..."));
    })
    .await;
}

#[tokio::test]
async fn dev_command_watches_environments_toml() {
    TestEnv::from_async("soroban-init-boilerplate", |env| async move {
        let mut dev_process = env
            .loam_process("dev")
            .current_dir(&env.cwd)
            .spawn()
            .expect("Failed to spawn dev process");
        // Give the dev process some time to start
        sleep(Duration::from_secs(2)).await;

        // Create environments.toml
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

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(5)).await;

        // Modify environments.toml
        env.set_environments_toml(
            r#"
development.accounts = [
    { name = "alice" },
]

[development.network]
rpc-url = "http://localhost:9000/rpc"
network-passphrase = "Standalone Network ; February 2017"
"#,
        );

        // Wait for the dev process to detect changes and rebuild
        sleep(Duration::from_secs(5)).await;

        dev_process.kill().await.expect("Failed to kill dev process");

        let stderr = String::from_utf8(dev_process.wait_with_output().await.expect("Failed to wait for dev process").stderr).unwrap();
        println!("{}",stderr);
        assert!(stderr.contains("Changes detected. Rebuilding..."));
        assert!(stderr.contains("🌐 using network at http://localhost:9000/rpc"));
    })
    .await;
}

#[tokio::test]
async fn dev_command_stops_on_ctrl_c() {
    TestEnv::from_async("soroban-init-boilerplate", |env| async move {
        let mut dev_process = env
            .loam_process("dev")
            .current_dir(&env.cwd)
            .spawn()
            .expect("Failed to spawn dev process");

        // Give the dev process some time to start
        sleep(Duration::from_secs(15)).await;

        // Simulate Ctrl+C
        dev_process.kill().await.expect("Failed to send Ctrl+C to dev process");

        let output = dev_process.wait_with_output().await.expect("Failed to wait for dev process");
        let stderr = String::from_utf8(output.stderr).unwrap();
        
        assert!(stderr.contains("Stopping dev mode."));
    })
    .await;
}
