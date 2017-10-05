use std::env;
use std::path::PathBuf;

fn main()
	{
    let mut args: Vec<String> = env::args().collect();

    let len = args.len();

    match len
    {
    	1 => 
    		{
    			// Considering that I will be using these to make
    			// paths eventually, maybe doing this is not the best
    			let path = std::env::current_dir().unwrap();
    			let path_s = path.to_str().unwrap().to_string();
    			args.push(path_s);
    		},

    	2 => println!("Perfect amount of variables"),
    	_ => panic!("Program only works with 1 or 2 arguments for now"),
    }

    let path = PathBuf::from(&args[1]);

    for entry in path.read_dir().expect("Read_dir call failed")
    {
    	if let Ok(entry) = entry
    	{
    		println!("{:?}", entry.path());
    	}
    }

    println!("{:?}", path);
	}