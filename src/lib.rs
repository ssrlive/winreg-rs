// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.

//! Crate for accessing MS Windows registry
//!
//!## Usage
//!
//!### Basic usage
//!
//!```toml,ignore
//!# Cargo.toml
//![dependencies]
//!winreg2 = "0.53"
//!```
//!
//!```no_run
//!use std::io;
//!use std::path::Path;
//!use winreg2::enums::*;
//!use winreg2::RegKey;
//!
//!fn main() -> io::Result<()> {
//!    println!("Reading some system info...");
//!    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
//!    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
//!    let pf: String = cur_ver.get_value("ProgramFilesDir")?;
//!    let dp: String = cur_ver.get_value("DevicePath")?;
//!    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
//!    let info = cur_ver.query_info()?;
//!    println!("info = {:?}", info);
//!    let mt = info.get_last_write_time_system();
//!    println!(
//!        "last_write_time as windows_sys::Win32::Foundation::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
//!        mt.wYear, mt.wMonth, mt.wDay, mt.wHour, mt.wMinute, mt.wSecond
//!    );
//!
//!    // enable `chrono` feature on `winreg2` to make this work
//!    // println!(
//!    //     "last_write_time as chrono::NaiveDateTime = {}",
//!    //     info.get_last_write_time_chrono()
//!    // );
//!
//!    println!("And now lets write something...");
//!    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//!    let path = Path::new("Software").join("WinregRsExample1");
//!    let (key, disp) = hkcu.create_subkey(&path)?;
//!
//!    match disp {
//!        REG_CREATED_NEW_KEY => println!("A new key has been created"),
//!        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
//!    }
//!
//!    key.set_value("TestSZ", &"written by Rust")?;
//!    let sz_val: String = key.get_value("TestSZ")?;
//!    key.delete_value("TestSZ")?;
//!    println!("TestSZ = {}", sz_val);
//!
//!    key.set_value("TestMultiSZ", &vec!["written", "by", "Rust"])?;
//!    let multi_sz_val: Vec<String> = key.get_value("TestMultiSZ")?;
//!    key.delete_value("TestMultiSZ")?;
//!    println!("TestMultiSZ = {:?}", multi_sz_val);
//!
//!    key.set_value("TestDWORD", &1234567890u32)?;
//!    let dword_val: u32 = key.get_value("TestDWORD")?;
//!    println!("TestDWORD = {}", dword_val);
//!
//!    key.set_value("TestQWORD", &1234567891011121314u64)?;
//!    let qword_val: u64 = key.get_value("TestQWORD")?;
//!    println!("TestQWORD = {}", qword_val);
//!
//!    key.create_subkey("sub\\key")?;
//!    hkcu.delete_subkey_all(&path)?;
//!
//!    println!("Trying to open nonexistent key...");
//!    hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
//!        io::ErrorKind::NotFound => panic!("Key doesn't exist"),
//!        io::ErrorKind::PermissionDenied => panic!("Access denied"),
//!        _ => panic!("{:?}", e),
//!    });
//!    Ok(())
//!}
//!```
//!
//!### Iterators
//!
//!```no_run
//!use std::io;
//!use winreg2::RegKey;
//!use winreg2::enums::*;
//!
//!fn main() -> io::Result<()> {
//!    println!("File extensions, registered in system:");
//!    for i in RegKey::predef(HKEY_CLASSES_ROOT)
//!        .enum_keys().map(|x| x.unwrap())
//!        .filter(|x| x.starts_with("."))
//!    {
//!        println!("{}", i);
//!    }
//!
//!    let system = RegKey::predef(HKEY_LOCAL_MACHINE)
//!        .open_subkey("HARDWARE\\DESCRIPTION\\System")?;
//!    for (name, value) in system.enum_values().map(|x| x.unwrap()) {
//!        println!("{} = {:?}", name, value);
//!    }
//!
//!    Ok(())
//!}
//!```
//!
cfg_if::cfg_if! {
    if #[cfg(not(windows))] {
        compile_error!("OS not supported. if your application is multi-platform, use `[target.'cfg(windows)'.dependencies] winreg2 = \"...\"`");
    } else {
        pub use crate::reg_key::{EnumKeys, EnumValues, RegKey, HKEY};
        pub use crate::reg_key_metadata::RegKeyMetadata;
        pub use crate::reg_value::RegValue;

        mod common;
        #[cfg(feature = "serialization-serde")]
        pub mod decoder;
        #[cfg(feature = "serialization-serde")]
        pub mod encoder;
        pub mod enums;
        pub mod reg_key;
        pub mod reg_key_metadata;
        pub mod reg_value;
        #[cfg(feature = "transactions")]
        pub mod transaction;
        pub mod types;
    }
}
