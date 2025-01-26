use std::env;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn find_llvm_windows() -> Option<PathBuf> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = hklm.open_subkey("SOFTWARE\\LLVM").ok()?;
    let llvm_path: String = key.get_value("").ok()?;
    Some(PathBuf::from(llvm_path))
}

fn main() {
    let llvm_path = if cfg!(target_os = "windows") {
        find_llvm_windows().unwrap_or_else(|| PathBuf::from("C:\\Program Files\\LLVM"))
    } else {
        PathBuf::from("/usr/lib/llvm-19")
    };

    println!("cargo:rustc-link-search=native={}", llvm_path.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=LLVM-C");
    
    // Set environment variable for llvm-sys
    let prefix = llvm_path.to_str().unwrap();
    println!("cargo:rustc-env=LLVM_SYS_191_PREFIX={}", prefix);
}
