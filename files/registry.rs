//! Windows registry read helpers for Fnm Node Manager.

pub const APP_KEY: &str = "Software\\Fnm\\Fnm Node Manager";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Fnm Node Manager";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Fnm Node Manager") },
        RegValue { name: "Version",     value: String::from("v2.1.14") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Fnm Node Manager") },
        RegValue { name: "Publisher",       value: String::from("Fnm") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Fnm Node Manager\\uninstall.exe\"") },
    ]
}
