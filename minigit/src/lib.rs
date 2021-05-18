use std::path::Path;
use std::process;
use std::env;
use std::fs;
use std::error::Error;
use std::io::ErrorKind;


#[derive(Debug)]
pub struct Repo {
    matrix: Vec<Vec<FileConfig>>,
    current_commit: i32,
}

#[derive(Debug)]
pub struct FileConfig {
    filename: String,
    version: i32,
    file_version: String,
}



impl FileConfig {
    pub fn new(filename: &str) -> FileConfig {
        //search directory to see if file exists
        let mut file_version = String::from(filename);
        let filename = String::from(filename);
        file_version.push_str("_0");
        let version = 0;

        FileConfig{filename, file_version, version}
    }
    pub fn increment_version(&mut self) {
        self.version = self.version +1;
        self.file_version = self.filename.clone();
        self.file_version.push_str(&self.version.to_string());
    }
}

impl Repo {
    
    pub fn new() -> Repo {
        let mut matrix: Vec<Vec<FileConfig>> = Vec::new();
        let commit0: Vec<FileConfig> = Vec::new();
        fs::create_dir("./minigit").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::AlreadyExists {

            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
        matrix.push(commit0);
        let current_commit = 0;
        Repo {matrix, current_commit}
    }
    
    pub fn add(&mut self, filename: &str) -> Result<(), &'static str>{
        let vec = &self.matrix[self.current_commit as usize];
        let exists = Path::new(&filename).exists();
        if !exists {
            return Err("Cannot find that file.");
        }
        for file in vec {
            if file.filename == filename {
                return Err("File has already been added");
            }
        }
        let file_config = FileConfig::new(filename);
        self.matrix[self.current_commit as usize].push(file_config);
        Ok(())
    }
    
    pub fn commit(&mut self) -> Result<(), &'static str> {
        //for file in current commit
            //check if file.file_version exists in minigit 
                //if so, check if file_version == file
                    //if no changes, do nothing
                    //else, increment version, copy file over to minigit
                //else, version should be 0, copy file over to minigit 
        Ok(())
    }
    
    
    //pub fn current_commit() -> 
    
}