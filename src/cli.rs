use crate::utils::{create_entry, open_database, revert_entry, set_entry, update_entry};

pub fn print_usage() {
    println!("git mark <command> [<args>]");
    println!();
    println!("Commands:");
    println!("  this <name>: mark the current uncommited changes as <name>");
    println!("  list: list all marks");
    println!("  as <name>: add the changes stored away by (git mark this) command");
    println!("  update <name>: update the changes stored by (git mark this) subcommand");
    println!("  revert <name>: revert the changes applied by (git mark this) command");
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

pub fn set_mark(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    set_entry(name)
}

pub fn update_mark(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    update_entry(name)
}

pub fn revert_mark(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    revert_entry(name)
}
