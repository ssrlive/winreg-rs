// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use tempfile::tempdir;
use windows_sys::Win32::Foundation;
use winreg2::enums::*;
use winreg2::types::FromRegValue;
use winreg2::{RegKey, RegValue};

mod common;

#[test]
fn test_raw_handle() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let handle = hklm.raw_handle();
    assert_eq!(HKEY_LOCAL_MACHINE, handle);
}

#[test]
fn test_load_appkey() {
    let val_name = "LoadKeyTest";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("RustLoadAppkeyTest.dat");
    let val1 = "Test123".to_owned();
    {
        let key1 = RegKey::load_app_key(&file_path, true).unwrap();
        key1.set_value(val_name, &val1).unwrap();
        // this fails on Windows 7 with ERROR_ALREADY_EXISTS
        let key_err = RegKey::load_app_key_with_flags(&file_path, KEY_READ, 0).unwrap_err();
        assert_eq!(
            key_err.raw_os_error(),
            Some(Foundation::ERROR_SHARING_VIOLATION as i32)
        );
    }
    let val2: String = {
        // this fails on Windows 7 with ERROR_ALREADY_EXISTS
        let key2 = RegKey::load_app_key_with_flags(&file_path, KEY_READ, 1).unwrap();
        key2.get_value(val_name).unwrap()
    };
    assert_eq!(val1, val2);
}

#[test]
fn test_open_subkey_with_flags_query_info() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let win = hklm
        .open_subkey_with_flags("Software\\Microsoft\\Windows", KEY_READ)
        .unwrap();

    let info = win.query_info().unwrap();
    info.get_last_write_time_system();
    #[cfg(feature = "chrono")]
    info.get_last_write_time_chrono();

    assert!(win
        .open_subkey_with_flags("CurrentVersion\\", KEY_READ)
        .is_ok());
    assert!(hklm
        .open_subkey_with_flags("i\\just\\hope\\nobody\\created\\that\\key", KEY_READ)
        .is_err());
}

#[test]
fn test_create_subkey_disposition() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\WinRegRsTestCreateSubkey";
    let (_subkey, disp) = hkcu.create_subkey(path).unwrap();
    assert_eq!(disp, REG_CREATED_NEW_KEY);
    let (_subkey2, disp2) = hkcu.create_subkey(path).unwrap();
    assert_eq!(disp2, REG_OPENED_EXISTING_KEY);
    hkcu.delete_subkey_all(path).unwrap();
}

#[test]
fn test_delete_subkey() {
    let path = "Software\\WinRegRsTestDeleteSubkey";
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey(path)
        .unwrap();
    assert!(RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey(path)
        .is_ok());
}

#[test]
fn test_delete_subkey_with_flags() {
    let path = "Software\\Classes\\WinRegRsTestDeleteSubkeyWithFlags";
    RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey_with_flags(path, KEY_WOW64_32KEY)
        .unwrap();
    assert!(RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey_with_flags(path, KEY_WOW64_32KEY)
        .is_ok());
}

#[test]
fn test_rename_subkey() {
    with_key!(key, "RenameSubkey" => {
        let old_name = "SubkeyA";
        let new_name = "SubkeyB";
        key.create_subkey(old_name).unwrap();
        assert!(key.rename_subkey(old_name, new_name).is_ok());
        assert!(key.open_subkey(new_name).is_ok());
    });
}

#[test]
fn test_copy_tree() {
    with_key!(key, "CopyTree" => {
        let (sub_tree, _sub_tree_disp) = key.create_subkey("Src\\Sub\\Tree").unwrap();
        for v in &["one", "two", "three"] {
            sub_tree.set_value(v, v).unwrap();
        }
        let (dst, _dst_disp) = key.create_subkey("Dst").unwrap();
        assert!(key.copy_tree("Src", &dst).is_ok());
    });
}

#[test]
fn test_long_value() {
    with_key!(key, "LongValue" => {
        let name = "RustLongVal";
        let val1 = RegValue { vtype: REG_BINARY, bytes: (0..6000).map(|_| rand::random::<u8>()).collect() };
        key.set_raw_value(name, &val1).unwrap();
        let val2 = key.get_raw_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

macro_rules! test_value_sz {
    ($fname:ident, $kname:expr, $conv:expr => $tout:ty) => {
        #[test]
        fn $fname() {
            with_key!(key, $kname => {
                let name = "RustSzVal";
                let val1 = $conv("Test123 \n$%^&|+-*/\\()");
                key.set_value(name, &val1).unwrap();
                let val2: $tout = key.get_value(name).unwrap();
                assert_eq!(val1, val2);
            });
        }
    }
}

test_value_sz!(test_string_value, "StringValue", str::to_owned => String);
test_value_sz!(test_str_value, "StrValue", |x|x => String);
test_value_sz!(test_os_string_value, "OsStringValue", OsString::from => OsString);
test_value_sz!(test_os_str_value, "OsStrValue", OsStr::new => OsString);

#[test]
fn test_long_string_value() {
    with_key!(key, "LongStringValue" => {
        let name = "RustLongStringVal";
        let val1 : String = rand::thread_rng().sample_iter(&Alphanumeric).take(7000).map(char::from).collect();
        key.set_value(name, &val1).unwrap();
        let val2: String = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_long_os_string_value() {
    with_key!(key, "LongOsStringValue" => {
        let name = "RustLongOsStringVal";
        let val1 = rand::thread_rng().sample_iter(&Alphanumeric).take(7000).map(char::from).collect::<String>();
        let val1 = OsStr::new(&val1);
        key.set_value(name, &val1).unwrap();
        let val2: OsString = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

macro_rules! test_value_multi_sz {
    ($fname:ident, $kname:expr, $conv:expr => $tout:ty) => {
        #[test]
        fn $fname() {
            with_key!(key, $kname => {
                let name = "RustMultiSzVal";

                let val1 = vec![
                    $conv("lorem ipsum\ndolor"),
                    $conv("sit amet")
                ];
                key.set_value(name, &val1).unwrap();
                let val2: Vec<$tout> = key.get_value(name).unwrap();

                assert_eq!(val1, val2);
            });
        }
    }
}

test_value_multi_sz!(test_vec_string_value, "StringVectorValue", str::to_owned => String);
test_value_multi_sz!(test_vec_str_value, "StrVectorValue", |x|x => String);
test_value_multi_sz!(test_vec_os_string_value, "OsStringVectorValue", OsString::from => OsString);
test_value_multi_sz!(test_vec_os_str_value, "OsStrVectorValue", OsStr::new => OsString);

#[test]
fn test_u32_value() {
    with_key!(key, "U32Value" => {
        let name = "RustU32Val";
        let val1 = 1_234_567_890u32;
        key.set_value(name, &val1).unwrap();
        let val2: u32 = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_u64_value() {
    with_key!(key, "U64Value" => {
        let name = "RustU64Val";
        let val1 = 1_234_567_891_011_121_314u64;
        key.set_value(name, &val1).unwrap();
        let val2: u64 = key.get_value(name).unwrap();
        assert_eq!(val1, val2);
    });
}

#[test]
fn test_delete_value() {
    with_key!(key, "DeleteValue" => {
        let name = "WinregRsTestVal";
        key.set_value(name, &"Qwerty123").unwrap();
        assert!(key.delete_value(name).is_ok());
    });
}

#[test]
fn test_enum_keys() {
    with_key!(key, "EnumKeys" => {
        let mut keys1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
        keys1.sort_unstable();
        for i in &keys1 {
            key.create_subkey(i).unwrap();
        }
        let keys2: Vec<_> = key.enum_keys().map(|x| x.unwrap()).collect();
        assert_eq!(keys1, keys2);
    });
}

#[test]
fn test_enum_values() {
    with_key!(key, "EnumValues" => {
        let mut vals1 = vec!("qwerty", "asdf", "1", "2", "3", "5", "8", "йцукен");
        vals1.sort_unstable();
        for i in &vals1 {
            key.set_value(i,i).unwrap();
        }
        let mut vals2: Vec<String> = Vec::with_capacity(vals1.len());
        let mut vals3: Vec<String> = Vec::with_capacity(vals1.len());
        for (name, val) in key.enum_values()
            .map(|x| x.unwrap())
        {
            vals2.push(name);
            vals3.push(String::from_reg_value(&val).unwrap());
        }
        assert_eq!(vals1, vals2);
        assert_eq!(vals1, vals3);
    });
}

#[test]
fn test_enum_long_values() {
    with_key!(key, "EnumLongValues" => {
        let mut vals = HashMap::with_capacity(3);

        for i in &[5500, 9500, 15000] {
            let name: String = format!("val{}", i);
            let val = RegValue { vtype: REG_BINARY, bytes: (0..*i).map(|_| rand::random::<u8>()).collect() };
            vals.insert(name, val);
        }

        for (name, val) in key.enum_values()
                              .map(|x| x.unwrap())
        {
            assert_eq!(val.bytes, vals[&name].bytes);
        }
    });
}
