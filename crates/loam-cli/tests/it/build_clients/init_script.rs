use crate::util::TestEnv;

#[test]
fn build_command_runs_init() {
    TestEnv::from("soroban-init-boilerplate", |env| {
        env.set_environments_toml(
            r#"
development.accounts = [
{ name = "alice" },
{ name = "bob" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts.soroban_token_contract]
workspace = true
init = """
initialize --symbol ABND --decimal 7 --name abundance --admin alice
mint --amount 2000000 --to alice
"""
"#,
        );

        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        // ensure the invoke commands are run with the proper source account
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains(" -- initialize --symbol ABND --decimal 7 --name abundance --admin alice"));
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains(" -- mint --amount 2000000 --to alice"));
        assert!(String::from_utf8_lossy(&output.stderr).contains(
            "✅ Initialization script for \"soroban_token_contract\" completed successfully"
        ));
        // ensure setting STELLAR_ACCOUNT works
        env.set_environments_toml(
            r#"
development.accounts = [
{ name = "alice" },
{ name = "bob" },
]

[development.network]
rpc-url = "http://localhost:8000/rpc"
network-passphrase = "Standalone Network ; February 2017"

[development.contracts.soroban_token_contract]
workspace = true
init = """
STELLAR_ACCOUNT=bob initialize --symbol ABND --decimal 7 --name abundance --admin bob 
STELLAR_ACCOUNT=bob mint --amount 2000000 --to bob 
"""
"#,
        );
        let output = env
            .loam_env("development", true)
            .output()
            .expect("Failed to execute command");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        // ensure the invoke commands are run with the proper source account
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("--source-account bob -- initialize --symbol ABND --decimal 7 --name abundance --admin bob"));
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("--source-account bob -- mint --amount 2000000 --to bob"));
        assert!(String::from_utf8_lossy(&output.stderr).contains(
            "✅ Initialization script for \"soroban_token_contract\" completed successfully"
        ));
    })
}
