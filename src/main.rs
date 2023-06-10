use git2::Repository;

const REPOSITORY_PATH: &str = "../../../Repos/QualityTool/";

fn main() {
    let repository = match Repository::open(REPOSITORY_PATH) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

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
