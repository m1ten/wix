use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::{
    kdbg,
    scripts::{self, KraitScript},
};

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct KraitConfig {
    // krait name
    #[default(String::from("krait"))]
    pub name: String,

    // krait author
    #[default(String::from("miten <57693631+m1ten@users.noreply.github.com>"))]
    #[serde(default)]
    #[serde(alias = "maintainer")]
    pub author: String,

    // krait version
    #[default(String::from("0.0.1"))]
    #[serde(alias = "version")]
    pub ver: String,

    // krait description
    #[default(String::from("cross platform package manager"))]
    #[serde(default)]
    #[serde(alias = "description")]
    pub desc: String,

    // krait license
    #[default(String::from("Apache-2.0"))]
    pub license: String,

    // krait git repository
    #[default(String::from("https://github.com/m1ten/krait"))]
    pub git: String,

    #[default(None)]
    #[serde(alias = "packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<Vec<String>>,

    #[default(dirs::home_dir().unwrap().join("krait"))]
    #[serde(default)]
    #[serde(alias = "directory")]
    pub dir: PathBuf,

    // krait package repository
    #[default(vec![String::from("https://github.com/m1ten/krait-pkgs")])]
    #[serde(alias = "repositories")]
    pub repos: Vec<String>,

    // krait default flags/args
    #[default(None)]
    #[serde(alias = "flags")]
    #[serde(alias = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl KraitScript for KraitConfig {}
impl mlua::UserData for KraitConfig {}

impl KraitConfig {
    pub fn new() -> KraitConfig {
        KraitConfig::default()
    }

    #[deprecated = "an alternative solution will be provided in the future"]
    pub fn gen_lua(&self) -> Vec<String> {
        let lua = mlua::Lua::new();
        let globals = lua.globals();

        let krait_t = lua.create_table().unwrap();
        let config_t = lua.create_table().unwrap();

        let mut dir = self.dir.clone().to_string_lossy().to_string();
        // check if running on windows
        if cfg!(target_os = "windows") {
            dir = dir.replace('\\', "\\\\");
        } else {
            dir = dir.replace('/', "\\/");
        }

        // assign values to config table
        config_t.set("name", self.name.clone()).unwrap();
        config_t.set("author", self.author.clone()).unwrap();
        config_t.set("ver", self.ver.clone()).unwrap();
        config_t.set("desc", self.desc.clone()).unwrap();
        config_t.set("license", self.license.clone()).unwrap();
        config_t.set("git", self.git.clone()).unwrap();
        config_t.set("pkgs", self.pkgs.clone()).unwrap();
        config_t.set("dir", dir).unwrap();
        config_t.set("args", self.args.clone()).unwrap();
        config_t.set("repos", self.repos.clone()).unwrap();

        // add config to krait table
        krait_t.set("config", config_t).unwrap();

        // add krait table to globals
        globals.set("krait", krait_t).unwrap();

        // get the krait table
        let krait_t = globals.get::<_, mlua::Table>("krait").unwrap();

        let mut result = scripts::LuaState::gen_lua("krait".to_string(), krait_t);

        // add the comments at the beginning of the file
        let mut comments = vec![
            "--           Krait Config           ".to_string(),
            "\n-- Automatically generated by Krait ".to_string(),
            "\n--      READ THE DOCUMENTATION      ".to_string(),
            "\n".to_string(),
        ];

        // add the generated lua code
        comments.append(&mut result);

        kdbg!(result.iter().map(|x| x.to_string()).collect::<String>());

        comments
    }
}