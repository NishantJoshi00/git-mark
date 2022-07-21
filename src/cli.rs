use crate::utils::{open_database, create_entry};



pub fn print_usage() {
    println!("git mark <command> [<args>]");
    println!("");
    println!("Commands:");
    println!("  this <name>: mark the current uncommited changes as <name>");
    println!("  list: list all marks");
    println!("  as <name>: add the changes stored away by (git mark this) command");
    println!("  update <name>: update the changes stored by (git mark this) subcommand");
    println!("  help: print this help");
}

pub fn list_marks() -> Result<(), Box<dyn std::error::Error>> {
    let database = open_database()?;
    for entry in database {
        println!("{}", entry.name);
    }
    Ok(())
}

pub fn add_mark(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_entry(name)
}

pub fn set_mark(name: &str) {
    todo!()
}

pub fn update_mark(name: &str) {
    todo!()
}