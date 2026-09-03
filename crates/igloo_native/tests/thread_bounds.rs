//! Compile checks for local widget captures and executor-bound callbacks.
use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn library(directory: &Path, name: &str) -> PathBuf {
    std::fs::read_dir(directory)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(&format!("{name}-"))
                && path
                    .extension()
                    .is_some_and(|extension| extension == "rlib")
        })
        .max_by_key(|path| path.metadata().unwrap().modified().unwrap())
        .unwrap_or_else(|| panic!("missing compiled library {name}"))
}

#[test]
fn async_captures_require_send_but_widget_captures_can_be_local() {
    let executable = std::env::current_exe().unwrap();
    let dependencies = executable.parent().unwrap();
    let output =
        std::env::temp_dir().join(format!("igloo-native-thread-bounds-{}", std::process::id()));
    std::fs::create_dir_all(&output).unwrap();
    let cases = [
        ("local_widget", true),
        ("arc_subscription", true),
        ("rc_subscription", false),
        ("rc_sleep", false),
    ];
    for (case, should_compile) in cases {
        let result = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .args(["--edition=2024", "--crate-type=lib", "--emit=metadata"])
            .arg(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/compile")
                    .join(format!("{case}.rs")),
            )
            .arg("--extern")
            .arg(format!(
                "igloo_native={}",
                library(dependencies, "libigloo_native").display()
            ))
            .arg("--extern")
            .arg(format!(
                "futures={}",
                library(dependencies, "libfutures").display()
            ))
            .arg("-L")
            .arg(format!("dependency={}", dependencies.display()))
            .arg("--out-dir")
            .arg(&output)
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&result.stderr);
        assert_eq!(result.status.success(), should_compile, "{case}: {stderr}");
        if !should_compile {
            assert!(
                stderr.contains("cannot be sent between threads safely")
                    || stderr.contains("cannot be shared between threads safely"),
                "{case}: {stderr}"
            );
        }
    }
    std::fs::remove_dir_all(output).unwrap();
}
