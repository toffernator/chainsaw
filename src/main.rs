use git2::Repository;

const REPOSITORY_PATH: &str = "../../../Repos/QualityTool/";
const REMOTE: &str = "origin";

fn main() {
    let repository = match Repository::open(REPOSITORY_PATH) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    list_branches(repository)
}

fn list_branches(repository: Repository) {
    let branches = match repository.branches(None) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to read branches: {}", e),
    };

    for b in branches {
        let branch = match b {
            Ok((branch, _)) => branch,
            Err(e) => panic!("failed to read branch: {}", e),
        };

        let name = match branch.name() {
            Ok(name) => name,
            Err(e) => panic!("failed to read name: {}", e),
        };

        match name {
            Some(n) => println!("Got: {}", n),
            None => println!("Got no name"),
        }
    }
}

fn list_remote_refs(repository: Repository) {
    let remote = match repository.find_remote(REMOTE) {
        Ok(remote) => remote,
        Err(e) => panic!("failed to find origin: {}", e),
    };
}
