use std::path::Path;
use std::io::{BufRead, Write};
use base64::{encode};

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    file: String,
    encrypted: bool,
}


pub fn open_database() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(Path::new("."))?;
    if let Some(repo_dir) = repo.workdir() {
        let database_path = repo_dir.join(".git-marks/database.dir");
        if !database_path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Database not found")));
        }
        
        // readlines from the database file
        let mut lines = vec![];
        let file = std::fs::File::open(database_path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let line = line.split(" ");
            let data = line.collect::<Vec<&str>>();
            
            lines.push(Entry {
                name: data[0].to_string(),
                file: data[1].to_string(),
                encrypted: data[2] == "true",
            });
        }



        Ok(lines)
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "No repository found")));
    }
}

fn create_database(data: Vec<Entry>) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(Path::new("."))?;
    if let Some(repo_dir) = repo.workdir() {
        let database_path = repo_dir.join(".git-marks/database.dir");
        let mut file = std::fs::File::create(database_path)?;
        for entry in data {
            let line = format!("{} {} {}\n", entry.name, entry.file, entry.encrypted);
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}

fn add_database_entry(name: &str, filename: &str, encrypted: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(entries) = open_database() {
        let mut entries = entries;
        entries.push(Entry {
            name: name.to_string(),
            file: filename.to_string(),
            encrypted,
        });
        create_database(entries)?;
    } else {
        create_database(vec![Entry {
            name: name.to_string(),
            file: filename.to_string(),
            encrypted,
        }])?;
    }
    Ok(())
}

fn delete_database_entry(name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let entries = open_database()?;
    let length = entries.len();
    let new_entries: Vec<Entry> = entries.into_iter().filter(|entry| entry.name != name).collect();
    if new_entries.len() == length {
        return Ok(false);
    }
    create_database(new_entries)?;
    Ok(true)
}


pub fn create_entry(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(Path::new("."))?;
    
    // set working directory to the repository
    std::env::set_current_dir(repo.workdir().unwrap())?;

    let storage_path = Path::new("./.git-marks/");
    if !storage_path.exists() {
        std::fs::create_dir(storage_path)?;
    }

    let entries = open_database()?;
    for entry in entries {
        if entry.name == name {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Entry already exists")));
        }
    }

    
    // TODO: Convert the following commands to rust equivalent

    let config = git2::Config::open_default()?;
    let u_name = config.get_string("user.name")?;
    let email = config.get_string("user.email")?;

    let patch_name = format!("{}, {}, {}", name, u_name, email);
    let patch_name = format!("{}.patch", encode(patch_name.as_bytes()));
    
    if storage_path.join(patch_name.clone()).exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists")));
    }


    // git add .
    let _cmd = std::process::Command::new("git")
        .args(["add", "."])
        .output()?;
    
    // git diff --staged -p > .git-marks/
    let cmd = std::process::Command::new("git")
        .args(["diff", "--staged", "-p"])
        .output()?;
    // write output to file
    let mut file = std::fs::File::create(storage_path.join(patch_name.clone()))?;
    file.write_all(cmd.stdout.as_slice())?;
    
    // git restore --staged .
    let _cmd = std::process::Command::new("git")
        .args(["restore", "--staged", "."])
        .output()?;
    
    add_database_entry(name, patch_name.as_str(), false)?;
    Ok(())
}


pub fn set_entry(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database = open_database()?;
    for entry in database {
        if entry.name == name {
            // Decryption logic if 
            if entry.encrypted {
                todo!()
            } else {
                 let repo = git2::Repository::discover(Path::new("."))?;

                std::env::set_current_dir(repo.workdir().unwrap())?;
                let storage_path = Path::new("./.git-marks/");

                let _cmd = std::process::Command::new("git")
                    .args(["apply", storage_path.join(entry.file.as_str()).to_str().unwrap()])
                    .output()?;
                
            }
        }
    }


    Ok(())
}

pub fn update_entry(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if delete_database_entry(name)? {
        create_entry(name)?;
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Mark not found")));
    }

    Ok(())
}

pub fn revert_entry(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entries = open_database()?;
    for entry in entries {
        if entry.name == name {
            let repo = git2::Repository::discover(Path::new("."))?;
            std::env::set_current_dir(repo.workdir().unwrap())?;
            let storage_path = Path::new("./.git-marks/");
            let _cmd = std::process::Command::new("git")
                .args(["apply", "-R", storage_path.join(entry.file.as_str()).to_str().unwrap()])
                .output()?;
            return Ok(());
        }
    }

    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Mark not found")));
}