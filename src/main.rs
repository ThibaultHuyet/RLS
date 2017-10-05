use std::env;

fn main()
	{
    let mut args: Vec<String> = env::args().collect();

    let len = args.len();

    match len
    {
    	1 => 
    		{
    			let path = std::env::current_dir().unwrap();
    			let path_s = path.to_str().unwrap().to_string();
    			args.push(path_s);
    		},
    	2 => println!("Perfect amount of variables"),
    	_ => panic!("Program only works with 1 or 2 arguments"),
    }

    println!("{:?}", args);
	}