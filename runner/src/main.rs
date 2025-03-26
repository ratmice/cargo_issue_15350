fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;
    use std::process::Command;
    use std::{env, path::Path};

    let dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    assert!(Path::new(&dir).ends_with("runner"));

    // We forward the CARGO_MANIFEST_DIR env var through to wasmtime.
    let mut args: Vec<OsString> = vec!["run".into()];
    let mut env_var = std::ffi::OsString::new();
    env_var.push("CARGO_MANIFEST_DIR");
    env_var.push("=");
    env_var.push(dir);
    args.extend(["--env".into(), env_var]);
    args.extend(env::args_os().skip(1).collect::<Vec<_>>());
    eprintln!(
        "executing: wasmtime {}",
        args.clone()
            .iter()
            .map(|s| s.to_string_lossy().into())
            .collect::<Vec<String>>()
            .join(" ")
    );
    Command::new("wasmtime").args(args).spawn()?.wait()?;
    Ok(())
}
