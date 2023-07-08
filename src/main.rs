use git2::{Repository, BranchType, Branch};

const REPOSITORY_PATH: &str = ".";
const REMOTE: &str = "origin";
fn main() {
    let repository = match Repository::open(REPOSITORY_PATH) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let orphans = repository.branches(Some(BranchType::Local))
        .unwrap()
        .map(|r| r.unwrap().0)
        .filter(|b| !has_remote(b));
    
    for mut o in orphans {
        println!("deleting {}", o.name().unwrap().unwrap());
        let _ = o.delete();
    }
}

// Assumes that branch.upstream() only fails when there is no upsteam
fn has_remote(branch: &Branch) -> bool {
    match branch.upstream() {
        Ok(_) => true,
        Err(_) => false,
    }
}
