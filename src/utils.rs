use std::path::Path;
use std::io::{BufRead, Write};
use base64::{encode};


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


pub fn create_entry(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(Path::new("."))?;
    
    // set working directory to the repository
    std::env::set_current_dir(repo.workdir().unwrap())?;

    let storage_path = Path::new("./.git-marks/");
    if !storage_path.exists() {
        std::fs::create_dir(storage_path)?;
    }
    
    // TODO: Convert the following commands to rust equivalent

    let config = git2::Config::open_default()?;
    let u_name = config.get_string("user.name")?;
    let email = config.get_string("user.email")?;

    let patch_name = format!("{}, {}, {}", name, u_name, email);
    let patch_name = format!("{}.patch", encode(patch_name.as_bytes()));
    println!("{}", patch_name);
    


    // git add .
    let cmd = std::process::Command::new("git")
        .args(["add", "."])
        .output()?;
    
    // git diff --staged -p > .git-marks/
    let cmd = std::process::Command::new("git")
        .args(["diff", "--staged", "-p"])
        .output()?;
    // write output to file
    let mut file = std::fs::File::create(storage_path.join(patch_name))?;
    file.write_all(cmd.stdout.as_slice())?;
    file.sync_all()?;

    



    Ok(())
}