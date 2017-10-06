extern crate RLS;
use RLS::conf::*;
use std::env;
use std::path::PathBuf;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    config.print_directory();
}