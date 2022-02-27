pub mod args;
pub mod pkg;
pub mod setup;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

// read from file
pub fn readfs(path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// write to file
pub fn writefs(path: String, contents: String) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

// read from stdin
pub fn scan<T: std::str::FromStr>(stopper: u8) -> Result<T, ()> {
    let mut input = Vec::<u8>::new();

    let mut data: [u8; 1] = [0];
    loop {
        match std::io::stdin().read_exact(&mut data) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }

        if data[0] != stopper && data[0] != '\n' as u8 {
            input.push(data[0]);
        } else {
            break;
        }
    }

    match std::str::from_utf8(&input).unwrap().trim().parse::<T>() {
        Ok(x) => Ok(x),
        Err(_) => Err(()),
    }
}

#[macro_export]
macro_rules! scan {
    ($str:tt, $_type:ty) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        neopkg::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
    ($str:tt) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        neopkg::scan::<String>('\n' as u8).expect("scanln failed")
    }};
}

// macro to clear console
#[macro_export]
macro_rules! clear {
    () => {{
        use std::process::Command;

        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/c").arg("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }};
}

#[macro_export]
macro_rules! exit {
    ($code: tt) => {{
        let key = if cfg!(target_os = "macos") {
            "return"
        } else {
            "enter"
        };
        let msg = format!("\nPress {} to exit.\n", key);
        neopkg::scanln!(msg);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! question {
    ($msg: tt) => {{
        loop {
            print!("{} [Y/n] ", $msg);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let answer = neopkg::scan::<String>('\n' as u8)
                .expect("question failed")
                .to_lowercase();
            if answer.trim() == "y" || answer.trim() == "yes" || answer.trim() == "" {
                break true;
            } else if answer.trim() == "n" || answer.trim() == "no" {
                break false;
            }
        }
    }};
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct NPConfig {
    #[default(NPInfo::default())]
    pub info: NPInfo,

    #[default(NP::default())]
    pub pkg: NP,

    #[default(NPDir::default())]
    pub dir: NPDir,
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct NPInfo {
    // neopkg name
    #[default(String::from("neopkg"))]
    pub name: String,

    // neopkg author
    #[default(String::from("miten"))]
    #[serde(default)]
    pub author: String,

    // neopkg version
    #[default(String::from("0.0.1"))]
    pub ver: String,

    // neopkg description
    #[default(String::from("cross platform package manager"))]
    #[serde(default)]
    pub desc: String,

    // neopkg license
    #[default(String::from("Apache-2.0"))]
    pub license: String,

    // neopkg git repository
    #[default(String::from("https://github.com/m1ten/neopkg"))]
    pub git: String,

    // neopkg repository
    #[default(vec![String::from("https://github.com/m1ten/neopkgs")])]
    pub repos: Vec<String>,

    // neopkg default flags/args
    #[default(None)]
    #[serde(alias = "args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct NP {
    // installed pkgs
    #[default(None)]
    #[serde(alias = "packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<Vec<String>>,
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct NPDir {
    // neopkg directory for posix
    #[default(dirs::home_dir().unwrap().join("neopkg"))]
    #[serde(default)]
    pub dir: PathBuf,

    #[default(dirs::home_dir().unwrap().join("neopkg/pkg"))]
    #[serde(default)]
    pub pkg_dir: PathBuf,

    #[default(dirs::home_dir().unwrap().join("neopkg/cache"))]
    #[serde(default)]
    pub cache_dir: PathBuf,
}
