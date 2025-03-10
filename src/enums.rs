// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.

//! `use winreg2::enums::*;` to import all needed enumerations and constants
pub use windows_sys::Win32::System::Registry::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_CURRENT_USER_LOCAL_SETTINGS,
    HKEY_DYN_DATA, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_PERFORMANCE_NLSTEXT,
    HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ALL_ACCESS, KEY_CREATE_LINK, KEY_CREATE_SUB_KEY,
    KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE, KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ, KEY_SET_VALUE,
    KEY_WOW64_32KEY, KEY_WOW64_64KEY, KEY_WOW64_RES, KEY_WRITE, REG_PROCESS_APPKEY,
};

macro_rules! winapi_enum{
    ($t:ident, $doc:expr => [$($v:ident),*]) => (
        #[doc=$doc]
        #[allow(non_camel_case_types)]
        #[derive(Debug,Clone,PartialEq)]
        pub enum $t {
            $( $v = windows_sys::Win32::System::Registry::$v as isize ),*
        }
    )
}

winapi_enum!(NotifyFilter, "Enumeration of possible changes that should be reported in RegNotifyChangeKeyValue" => [
    REG_NOTIFY_CHANGE_NAME ,
    REG_NOTIFY_CHANGE_ATTRIBUTES,
    REG_NOTIFY_CHANGE_LAST_SET,
    REG_NOTIFY_CHANGE_SECURITY,
    REG_NOTIFY_THREAD_AGNOSTIC
]);

winapi_enum!(RegType, "Enumeration of possible registry value types" => [
REG_NONE,
REG_SZ,
REG_EXPAND_SZ,
REG_BINARY,
REG_DWORD,
REG_DWORD_BIG_ENDIAN,
REG_LINK,
REG_MULTI_SZ,
REG_RESOURCE_LIST,
REG_FULL_RESOURCE_DESCRIPTOR,
REG_RESOURCE_REQUIREMENTS_LIST,
REG_QWORD
]);
pub use self::RegType::*;

winapi_enum!(RegDisposition, "Enumeration of possible disposition values" => [
REG_CREATED_NEW_KEY,
REG_OPENED_EXISTING_KEY
]);
pub use self::RegDisposition::*;
