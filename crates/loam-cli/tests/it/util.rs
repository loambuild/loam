use assert_cmd::{assert::Assert, Command};
use assert_fs::TempDir;
use fs_extra::dir::{copy, CopyOptions};
use std::future::Future;
use std::path::PathBuf;
use std::fs;
use rand::{thread_rng, Rng};
use toml::Value;
use std::error::Error;
use tokio::process::Command as ProcessCommand;

pub struct TestEnv {
    pub temp_dir: TempDir,
    pub cwd: PathBuf,
}

pub trait AssertExt {
    fn stdout_as_str(&self) -> String;
    fn stderr_as_str(&self) -> String;
}

impl AssertExt for Assert {
    fn stdout_as_str(&self) -> String {
        String::from_utf8(self.get_output().stdout.clone())
            .expect("failed to make str")
            .trim()
            .to_owned()
    }
    fn stderr_as_str(&self) -> String {
        String::from_utf8(self.get_output().stderr.clone())
            .expect("failed to make str")
            .trim()
            .to_owned()
    }
}

impl TestEnv {
    pub fn new(template: &str) -> Self {
        let temp_dir = TempDir::new().unwrap();
        let template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");

        copy(template_dir.join(template), &temp_dir, &CopyOptions::new()).unwrap();

        Self {
            cwd: temp_dir.path().join(template),
            temp_dir,
        }
    }

    pub fn from<F: FnOnce(&TestEnv)>(template: &str, f: F) {
        let test_env = TestEnv::new(template);
        f(&test_env);
    }

    pub async fn from_async<F, Fut>(template: &str, f: F)
    where
        F: FnOnce(TestEnv) -> Fut,
        Fut: Future<Output = ()>,
    {
        let test_env = TestEnv::new(template);
        f(test_env).await;
    }

    pub fn modify_file(&self, path: &str, content: &str) {
        let file_path = self.cwd.join(path);
        std::fs::write(file_path, content).expect("Failed to modify file");
    }

    pub fn delete_file(&self, path: &str) {
        let file_path = self.cwd.join(path);
        std::fs::remove_file(file_path).expect("Failed to delete file");
    }

    pub async fn from_async<F, Fut>(template: &str, f: F)
    where
        F: FnOnce(TestEnv) -> Fut,
        Fut: Future<Output = ()>,
    {
        let test_env = TestEnv::new(template);
        f(test_env).await;
    }

    pub fn modify_file(&self, path: &str, content: &str) {
        let file_path = self.cwd.join(path);
        std::fs::write(file_path, content).expect("Failed to modify file");
    }

    pub fn delete_file(&self, path: &str) {
        let file_path = self.cwd.join(path);
        std::fs::remove_file(file_path).expect("Failed to delete file");
    }

    pub fn modify_wasm(&self, contract_name: &str) -> Result<(), Box<dyn Error>> {
        // Read Cargo.toml to get the actual name
        let cargo_toml_path = self.cwd.join("contracts").join(contract_name).join("Cargo.toml");
        let cargo_toml_content = fs::read_to_string(cargo_toml_path)?;
        let cargo_toml: Value = toml::from_str(&cargo_toml_content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        let package_name = cargo_toml["package"]["name"]
            .as_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Cargo.toml"))?;

        // Convert package name to proper filename format
        let filename = package_name.replace('-', "_");

        // Construct the path to the .wasm file
        let wasm_path = self.cwd
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(format!("{}.wasm", filename));

        let mut wasm_bytes = fs::read(&wasm_path)?;
        let mut rng = thread_rng();
        let random_bytes: Vec<u8> = (0..10).map(|_| rng.gen()).collect();
        wasm_gen::write_custom_section(&mut wasm_bytes, "random_data", &random_bytes);
        fs::write(&wasm_path, wasm_bytes)?;

        Ok(())
    }

    pub fn loam_build(&self, env: &str, randomize_wasm: bool) -> Command {
        if(randomize_wasm){
            // Run initial build
            let mut initial_build = Command::cargo_bin("loam").unwrap();
            initial_build.current_dir(&self.cwd);
            initial_build.arg("build");
            initial_build.arg(env);
            initial_build.output().expect("Failed to execute initial build");

            // Modify WASM files
            let contracts_dir = self.cwd.join("contracts");
            if let Ok(entries) = fs::read_dir(contracts_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(contract_name) = entry.file_name().to_str() {
                                self.modify_wasm(contract_name).expect("Failed to modify WASM");
                            }
                        }
                    }
                }
            }
        }
        // Run final build with --build-clients
        let mut loam = Command::cargo_bin("loam").unwrap();
        loam.current_dir(&self.cwd);
        loam.arg("build");
        loam.arg(env);
        loam.arg("--build-clients");
        loam
    }

    fn cargo_bin_loam(&self) -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_loam"))
    }

    pub fn loam_process(&self, cmd: &str) -> ProcessCommand {
        println!("{}", self.cargo_bin_loam().display());
        let mut loam = ProcessCommand::new(self.cargo_bin_loam());
        loam.current_dir(&self.cwd);
        loam.arg(cmd);
        loam
    }

    fn cargo_bin_loam(&self) -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_loam"))
    }

    pub fn loam_process(&self, cmd: &str) -> ProcessCommand {
        println!("{}", self.cargo_bin_loam().display());
        let mut loam = ProcessCommand::new(self.cargo_bin_loam());
        loam.current_dir(&self.cwd);
        loam.arg(cmd);
        loam
    }

    pub fn loam(&self, cmd: &str) -> Command {
        if cmd == "build" {
            self.loam_build("production", true)
        }
        else{
            let mut loam = Command::cargo_bin("loam").unwrap();
            loam.current_dir(&self.cwd);
            loam.arg(cmd);
            loam
        }
    }

    pub fn loam_env(&self, env: &str, randomize_wasm: bool) -> Command {
        self.loam_build(env, randomize_wasm)
    }

    pub fn soroban(&self, cmd: &str) -> Command {
        let mut soroban = Command::new("soroban");
        soroban.env(
            "PATH",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/bin"),
        );
        soroban.current_dir(&self.cwd);
        soroban.arg(cmd);
        soroban
    }

    pub fn replace_file(&self, path1: &str, path2: &str) {
        let file_path1 = self.cwd.join(path1);
        let file_path2 = self.cwd.join(path2);
        let content = std::fs::read_to_string(&file_path1).expect("Failed to read file1");
        std::fs::write(&file_path2, content).expect("Failed to modify file");
    }

    pub fn set_environments_toml(&self, contents: impl AsRef<[u8]>) {
        std::fs::write(self.cwd.join("environments.toml"), contents).unwrap();
    }
}
