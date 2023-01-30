//! `dot` is a shell command which helps installing or updating dotfiles
//! and setup symlinks automatically
#![allow(unused)]
#![feature(fs_try_exists, if_let_guard)]

use mylibrary::cli;
use mylibrary::sh;
use mylibrary::sh_cmd;
use std::env;
use std::fs;
use std::io;

const REPOSITORY: &str = "sugiura-hiromichi/.config";
const RELATIVE_CONF_PATH: &str = ".config";

/// TODO: receive boolean argument `relative` & `curdir`. if `relative==true` return relative path
/// from `curdir`
fn conf_path() -> String {
	match env::var("XDG_CONFIG_HOME",) {
		Ok(val,) => {
			if &val[val.len() - 1..val.len()] == "/" {
				val[..val.len() - 2].to_string()
			} else {
				val
			}
		},
		Err(_,) => home_path() + "/.config",
	}
}

fn home_path() -> String {
	match env::var("HOME",) {
		Ok(val,) => val,
		Err(_,) => panic!("|>set $HOME variable"),
	}
}

fn linkable(s: &str,) -> bool {
	s.contains(".config/.",) && !s.contains("git",) && !s.contains("DS_Store",)
		|| s.contains(".gitconfig",)
}

fn main() {
	// TODO:	let mut args = env::args();
	// detect dotfiles location
	let xdg_config_home = conf_path();

	match fs::try_exists(format!("{xdg_config_home}/.git/"),) {
		Ok(true,) => {
			//  no need to clone. pull it.
			sh_cmd!("cd", [xdg_config_home]).expect("can't `cd` to XDG_CONFIG_HOME",);
			sh_cmd!("git", ["pull"]);
		},
		_ => {
			match sh_cmd!("cd", [home_path()]) {
				Ok(_,) => {
					//  need to clone
					println!("Clone your dotfiles directory.");
					sh_cmd!("git", ["clone".to_string(), format!("git@github.com:{REPOSITORY}")]);
				},
				Err(e,) => {
					// exit
					eprintln!("Failed to move home directory:\n\t|error message: {e}");
					return;
				},
			}
		},
	}

	// symlinking
	sh_cmd!("cd", [home_path()]).expect("Failed to move home directory",);
	let Ok(files,) = fs::read_dir(RELATIVE_CONF_PATH) else{
      eprintln!("error happen while reading XDG_CONFIG_HOME");
      return;
   };

	for entry in files {
		let entry = entry.expect("Fail to get entry",).path();
		let path = entry.to_str().expect("Failed to get file_name",);
		if linkable(path,) {
			sh_cmd!("ln", ["-fsn", path]);
		}
	}

	sh_cmd!("cd", [home_path() + "/.local"]).expect("Failed to move ~/.local",);
	sh_cmd!("ln", ["-fsn".to_string(), conf_path() + "/bin"]);

	println!("\t<|dotfiles updated|>");
}
