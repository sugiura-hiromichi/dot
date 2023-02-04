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

fn main() -> io::Result<(),> {
	let xdg_config_home = conf_path();

	println!("syncing...");
	match fs::try_exists(format!("{xdg_config_home}/.git/"),) {
		Ok(true,) => {
			//  no need to clone. pull it.
			sh_cmd!("cd", [xdg_config_home.clone()]);
			sh_cmd!("git", ["pull"])?;
		},
		_ => {
			sh_cmd!("cd", [home_path()]);
			sh_cmd!("git", ["clone".to_string(), format!("git@github.com:{REPOSITORY}")])?;
		},
	}

	// symlinking
	println!("symlinking...");
	sh_cmd!("cd", [home_path()]);
	let files = fs::read_dir(xdg_config_home,)?;
	for entry in files {
		let entry = entry.expect("Fail to get entry",).path();
		let path = entry.to_str().expect("Failed to get file_name",);
		if linkable(path,) {
			sh_cmd!("ln", ["-fsn", path])?;
		}
	}

	sh_cmd!("cd", [home_path() + "/.local"]);
	sh_cmd!("ln", ["-fsn".to_string(), conf_path() + "/bin"])?;

	println!("dotfiles updated|>");
	Ok((),)
}
