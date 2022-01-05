pub mod args;
pub mod pkg;
pub mod py;
pub mod setup;

use std::{fs::File, io::{self, Read, Write}};

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
        wix::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
	($str:tt) => {{
		print!("{}", $str);
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
		wix::scan::<String>('\n' as u8).expect("scanln failed")
	}};
}

// macro to clear console
#[macro_export]
macro_rules! clear {
	() => {{
		// if cfg!(target_os = "windows") {
		// 	print!("\x1B[2J");
		// } else {
		// 	print!("\x1B[2J\x1B[1;1H");
		// }
		// std::io::Write::flush(&mut std::io::stdout()).unwrap();

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
		let key = if cfg!(target_os = "macos") { "return" } else { "enter" };
		let msg = format!("\nPress {} to exit.\n", key);
		wix::scanln!(msg);
    	std::process::exit($code);
	}};
}

#[macro_export]
macro_rules! question {
	($msg: tt) => {{
		loop {
			print!("{} [Y/n] ", $msg);
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			let answer = wix::scan::<String>('\n' as u8).expect("question failed").to_lowercase();
			if answer.trim() == "y" || answer.trim() == "yes" || answer.trim() == "" {
				break true;
			} else if answer.trim() == "n" || answer.trim() == "no" {
				break false;
			}
		}
	}}
}

#[derive(Debug, Clone)]
pub struct WixPy {
    // wix name
    pub name: String,

    // wix author
    pub author: String,

    // wix version
    pub ver: String,

    // wix description
    pub desc: String,

    // wix license
    pub license: String,

    // wix git repository
    pub git: String,

    // wix repository
    pub repo: String,

    // wix mirrors
    pub mirrors: Vec<String>,

    // installed pkgs
    pub pkgs: Vec<String>,
}

// set default values for config
impl Default for WixPy {
    fn default() -> Self {
        WixPy {
            name: "wix".to_string(),
            author: "miten".to_string(),
            ver: "0.1.0".to_string(),
            desc: "cross platform package manager".to_string(),
            license: "zlib".to_string(),
            git: "https://github.com/m1ten/wix".to_string(),
            repo: "https://github.com/m1ten/wix-pkgs/".to_string(),
            mirrors: vec![] as Vec<String>,
            pkgs: vec![] as Vec<String>,
        }
    }
}


// impl Config {
//     pub fn get_field_type(config: Option<Config>) -> IndexMap<String, String> {
//         let config = match config {
//             Some(i) => i,
//             None => {
//                 let mut map = IndexMap::new();
//                 map.insert("name".to_string(), "String".to_string());
//                 map.insert("author".to_string(), "String".to_string());
//                 map.insert("ver".to_string(), "String".to_string());
//                 map.insert("desc".to_string(), "String".to_string());
//                 map.insert("license".to_string(), "String".to_string());
//                 map.insert("git".to_string(), "String".to_string());
//                 map.insert("repo".to_string(), "String".to_string());
//                 map.insert("mirrors".to_string(), "Vec<String>".to_string());
//                 map.insert("pkgs".to_string(), "Vec<String>".to_string());

//                 return map;
//             }
//         };
//         let mut map = IndexMap::new();
//         map.insert("name".to_string(), config.name.clone());
//         map.insert("author".to_string(), config.author.clone());
//         map.insert("ver".to_string(), config.ver.clone());
//         map.insert("desc".to_string(), config.desc.clone());
//         map.insert("license".to_string(), config.license.clone());
//         map.insert("git".to_string(), config.git.clone());
//         map.insert("repo".to_string(), config.repo.clone());
//         map.insert("mirrors".to_string(), config.mirrors.clone().to_string());
//         map
//     }
// }