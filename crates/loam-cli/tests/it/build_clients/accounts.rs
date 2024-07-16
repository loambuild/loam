use crate::util::{AssertExt, TestEnv};

#[test]
fn create_two_accounts() {
    TestEnv::from("soroban-init-boilerplate", |env| {
        env.set_environments_toml(r#"
[production]
network = { rpc-url = "http://localhost:8000/rpc", network-passphrase = "Standalone Network ; February 2017"}

accounts = [
    { name = "alice" },
    { name = "bob" },
]
[production.contracts]
hello_world.workspace = false
soroban_increment_contract.workspace = false
soroban_custom_types_contract.workspace = false
soroban_auth_contract.workspace = false
"#);

        let stderr = env.loam("build").assert().success().stderr_as_str();
        assert!(stderr.contains("creating keys for \"alice\""));
        assert!(stderr.contains("creating keys for \"bob\""));
        assert!(env.cwd.join(".soroban/identity/alice.toml").exists());
        assert!(env.cwd.join(".soroban/identity/bob.toml").exists());

        // check that they're actually funded
        let stderr = env
            .soroban("keys")
            .args([
                "fund",
                "alice",
                "--network-passphrase",
                "\"Standalone Network ; February 2017\"",
                "--rpc-url",
                "http://localhost:8000/soroban/rpc",
            ])
            .assert()
            .success()
            .stderr_as_str();
        assert!(stderr.contains("Account already exists"));
    });
}
