use std::path::Path;
use std::io::{BufRead};
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
    // get git diff and save it as a .patch


    let diff = repo.diff_index_to_workdir(None, None)?;

    // create a patch file form the diff
    



    Ok(())
}