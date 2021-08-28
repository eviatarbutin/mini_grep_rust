use std::env;
use std::fs;
use std::process;
use std::io::{self, Read};

struct Flags {
	c: bool,
	i: bool,
}
struct Config {
	flags: Flags,
	word: Option<String>,
	filename: Option<String>,
}
fn main() {
	let mut args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		eprintln!("not enough arguments");
		process::exit(1);
	}
	args.remove(0);
	
	let mut config = Config{word: None, filename: None, flags: Flags {c:false, i:false}};
	
	for argument in args {
		if argument.starts_with('-') {
			if argument == "-c" {
				config.flags.c = true;
			} else if argument == "-i" {
				config.flags.i = true;
			} else {
				eprintln!("The flag: {} doesn't exist", argument);
				process::exit(1);
			}
		} else if config.word == None {
			config.word = Some(argument);
		} else if config.filename == None {
			config.filename = Some(argument);
		} else {
			eprintln!("unexpected argument: {}", argument);
			process::exit(1);
		}
	}
	
	let mut file_contents = String::new();
	
	if config.filename != None {
		file_contents = fs::read_to_string(config.filename.unwrap()).expect("error in file reading");
	} else {
		io::stdin().read_to_string(&mut file_contents).expect("error in stdin reading");
	}
	if config.flags.i {
		file_contents = file_contents.to_lowercase();
		config.word = Some(config.word.unwrap().to_lowercase());
	}
	
	let mut counter: u32 = 0;
	let lines = file_contents.lines();
	let temp: String = config.word.unwrap();
	for line in lines {
		if line.find(&temp) != None {
			if config.flags.c {
				counter += 1;
			} else {
				println!("{}", line);
			}
		}
	}
	
	if config.flags.c {
		println!("{}", counter);
	}
}
