//! `dot` is a shell command which helps installing or updating dotfiles
//! and setup symlinks automatically
#![allow(unused)]
#![feature(fs_try_exists, if_let_guard)]

use mylibrary::sh;
use mylibrary::sh_cmd;
use std::env;
use std::fs;
use std::io;

const REPOSITORY: &str = "sugirua-hiromichi/.config";

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
		Err(_,) => "/User/r".to_string(),
	}
}

/// TODO: require setting file to specify url of dotfile
fn main() {
	// detect dotfiles location
	let xdg_config_home = conf_path();

	match fs::try_exists(format!("{xdg_config_home}/.git/"),) {
		Ok(true,) => {
			//  no need to clone. pull it.
			sh_cmd!("git", ["pull"]);
		},
		_ => {
			match sh::cd(home_path(),) {
				Ok(_,) => {
					//  need to clone
					println!("Clone your dotfiles directory.");
					sh_cmd!("git", ["clone".to_string(), format!("https://github.com/{REPOSITORY}/")]);
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
	// ensured that current directory is `~/`
	let Ok(files,) = fs::read_dir(xdg_config_home,) else{
      eprintln!("error happen while reading XDG_CONFIG_HOME");
      return;
   };

	for entry in files {
		let entry = entry.expect("Fail to get entry",);
		let path = entry.path();
		if !path.is_dir() {
			let path = path.to_str().expect("Failed to get file_name",);
			if &path[0..1] == "." || path != ".gitignore" {
				//std::os::unix::fs::symlink(file_name, "~/",).expect("symlink error",);
				sh_cmd!("ln", ["-fsn", path]);
			}
		}
	}

	println!("\t------------|dotfiles updated|------------");
}
