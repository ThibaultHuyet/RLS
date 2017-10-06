use std::env;
use std::path::PathBuf;

fn main()
	{
	    let args: Vec<String> = env::args().collect();
	    let config = parse_config(&args);
	    // let path = PathBuf::from(&config[0]);

	    config.print_directory();
	}


// config data type
struct Config
{
	path: PathBuf,
	options: Vec<String>
}

impl Config
{
	pub fn print_directory(&self)
	{
		for entry in self.path.read_dir().expect("Could not read")
		{
			if let Ok(entry) = entry
			{
				let file = entry.file_name();
				println!("{}", file.to_str().unwrap());
			}
		}
	}
}

// Possible inputs should be:
// Nothing
// Just a directory
// A directory and any amount of options
fn parse_config(args: &[String]) -> Config
{
	let len = args.len();
	let mut options: Vec<String> = Vec::new();
	let mut path = PathBuf::new();

	match len
	{
		// If only 1 input is found, then
		// nothing was passed to the program
		// I need to make the default case here
		// This would be the directory program
		// was called in
		1 => {
				let cur = std::env::current_dir().unwrap();
				let cur_s = cur.to_str().unwrap().to_string();
				path.push(&cur_s);
			},

		// If multiple inputs are put in, I
		// would have to determine whether
		// It is an option or directory
		// TO DO: ^^^^^^^ That
		2 => {
				let ts = args[1].to_string();
				path.push(&ts);
			},

		// For now, I'm just testing with 2 inputs
		// Eventually I will find some way to
		// 
		_ => println!("Please never reach this"),
	}
	Config{path, options}
}