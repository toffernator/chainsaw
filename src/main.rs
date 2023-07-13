use git2::{Repository, BranchType, Branch};
use clap::Parser;
use anyhow::{Context, Result, Error};

#[derive(Parser)]
struct Cli {
    // The repository to remove trim
    repository: std::path::PathBuf,
    
    // Show the branches that would be deleted, but do not delete any branches
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let repository = Repository::open(&args.repository)
        .with_context(|| format!("Failed to open repository at {:?}", args.repository))?;
   
     let orphans = repository.branches(Some(BranchType::Local))
         .unwrap()
         .map(|r| r.unwrap().0)
         .filter(|b| !has_remote(b));
     
     for mut o in orphans {
        let name = o.name()
            .with_context(|| "Failed to get name for a branch")?
            .ok_or(Error::msg("Branch name is not valid UTF-8"))?;

        println!("deleting {}", name);
        if !args.dry_run {
            o.delete()?
        };
    }

    Ok(())
}

// Assumes that branch.upstream() only fails when there is no upsteam
fn has_remote(branch: &Branch) -> bool {
    match branch.upstream() {
        Ok(_) => true,
        Err(_) => false,
    }
}
