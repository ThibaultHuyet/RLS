use std::env;
use std::path::PathBuf;

fn main()
	{
	    let args: Vec<String> = env::args().collect();
	    let config = parse_config(&args);
	    let path = PathBuf::from(&config[0]);

	    for entry in path.read_dir().expect("Read_dir call failed")
	    {
	    	if let Ok(entry) = entry
	    	{
	    		let file = entry.file_name();
	    		println!("{}", file.to_str().unwrap());
	    	}
	    }
	}

// Possible inputs should be:
// Nothing
// Just a directory
// A directory and any amount of options
fn parse_config(args: &[String]) -> Vec<String>
{
	let len = args.len();
	let mut result: Vec<String> = Vec::new();

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
				result.push(cur_s);
			},

		// If multiple inputs are put in, I
		// would have to determine whether
		// It is an option or directory
		// TO DO: ^^^^^^^ That
		2 => {
				let ts = args[1].to_string();
				result.push(ts);
			},
		_ => println!("Please never reach this"),
	}
	result
}