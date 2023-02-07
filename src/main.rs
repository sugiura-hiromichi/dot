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
use PathIdentifier::*;

const REPOSITORY: &str = "sugiura-hiromichi/.config";

enum PathIdentifier {
	Conf,
	Home,
	Cargo,
}
impl PathIdentifier {
	fn raw(&self,) -> String {
		use PathIdentifier::*;
		match self {
			Conf => match env::var("XDG_CONFIG_HOME",) {
				Ok(val,) => {
					if &val[val.len() - 1..val.len()] == "/" {
						val[..val.len() - 2].to_string()
					} else {
						val
					}
				},
				Err(_,) => Home.raw() + "/.config",
			},
			Home => match env::var("HOME",) {
				Ok(val,) => val,
				Err(e,) => panic!("|>set $HOME variable\n|> error message | {e}"),
			},
			Cargo => match env::var("CARGO_HOME,",) {
				Ok(val,) => val,
				Err(_,) => Home.raw() + "/.cargo",
			},
		}
	}
}

fn linkable(s: &str,) -> bool {
	s.contains(".config/.",) && !s.contains("git",) && !s.contains("DS_Store",)
		|| s.contains(".gitconfig",)
}

fn main() -> io::Result<(),> {
	println!("syncing...");
	match fs::try_exists(format!("{}/.git/", Conf.raw()),) {
		Ok(true,) => {
			//  no need to clone. pull it.
			sh_cmd!("cd", [Conf.raw()]);
			sh_cmd!("git", ["pull"])?;
		},
		_ => {
			sh_cmd!("cd", [Home.raw()]);
			sh_cmd!("git", ["clone".to_string(), format!("git@github.com:{REPOSITORY}")])?;
		},
	}

	// symlinking
	println!("symlinking...");
	sh_cmd!("cd", [Home.raw()]);
	let files = fs::read_dir(Conf.raw(),)?;
	for entry in files {
		let entry = entry.expect("Fail to get entry",).path();
		let path = entry.to_str().expect("Failed to get file_name",);
		if linkable(path,) {
			sh_cmd!("ln", ["-fsn", path])?;
		}
	}

	sh_cmd!("cd", [Home.raw() + "/.local"]);
	sh_cmd!("ln", ["-fsn".to_string(), Conf.raw() + "/bin"])?;

	sh_cmd!("cd", [Cargo.raw()]);
	sh_cmd!("ln", ["-fsn".to_string(), Conf.raw() + "config.toml"])?;

	println!("dotfiles updated|>");
	Ok((),)
}
