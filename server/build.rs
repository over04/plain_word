use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let web_dir = Path::new(&manifest_dir).parent().unwrap().join("web");
    let static_dir = Path::new(&manifest_dir).join("static");

    println!("cargo:rerun-if-changed=../web/src");
    println!("cargo:rerun-if-changed=../web/index.html");
    println!("cargo:rerun-if-changed=../web/package.json");
    println!("cargo:rerun-if-changed=../web/vite.config.ts");
    println!("cargo:rerun-if-changed=../web/tailwind.config.js");

    if env::var("SKIP_WEB_BUILD").is_ok() {
        println!("cargo:warning=Skipping web build (SKIP_WEB_BUILD is set)");
        return;
    }

    let node_modules = web_dir.join("node_modules");
    if !node_modules.exists() {
        println!("cargo:warning=Installing npm dependencies...");
        let status = Command::new("npm")
            .arg("install")
            .current_dir(&web_dir)
            .status()
            .expect("Failed to run npm install");

        if !status.success() {
            panic!("npm install failed");
        }
    }

    println!("cargo:warning=Building web frontend...");
    let status = Command::new("npm")
        .args(["run", "build"])
        .current_dir(&web_dir)
        .status()
        .expect("Failed to run npm build");

    if !status.success() {
        panic!("npm build failed");
    }

    let web_dist = web_dir.join("dist");
    if web_dist.exists() {
        if static_dir.exists() {
            std::fs::remove_dir_all(&static_dir).expect("Failed to remove old static dir");
        }
        copy_dir_all(&web_dist, &static_dir).expect("Failed to copy dist to static");
        println!("cargo:warning=Web build completed successfully");
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            std::fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}