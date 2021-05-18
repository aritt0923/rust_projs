use ::minigit;
use std::env;
use std::process;

fn main() {
    let mut new_repo = minigit::Repo::new();
    new_repo.add("test.txt").unwrap_or_else(|err| {
        eprintln!("Problem adding file to repo: {}", err);
        process::exit(1);
    });
    
    println!("Repo looks like: {:?}", new_repo);
    
    new_repo.add("test2.txt").unwrap_or_else(|err| {
        eprintln!("Problem adding file to repo: {}", err);
        process::exit(1);
    });
    
    println!("Now Repo looks like: {:?}", new_repo);
    

}
