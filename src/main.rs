use std::env;
mod cli;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {







    // CLI Section
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_str() {
                "list" => cli::list_marks()?,
                _ => cli::print_usage(),
            }
        }
        3 => {
            match args[1].as_str() {
                "this" => cli::add_mark(args[2].as_str())?,
                "as" => cli::set_mark(args[2].as_str()),
                "update" => cli::update_mark(args[2].as_str()),
                _ => cli::print_usage(),
            }
        }
        _ => cli::print_usage(),
    }



    Ok(())
}