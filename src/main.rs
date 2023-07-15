use anyhow::{Context, Result, Error};
use clap::Parser;
use git2::{Repository, BranchType, Branch};
use log::{error, warn, info, debug, trace};
use simplelog::{SimpleLogger, LevelFilter, Config};
use std::fmt::Display;

#[derive(Parser)]
struct Cli {
    /// The repository to trim
    repository: std::path::PathBuf,
    
    /// Show the branches that would be deleted, but do not delete any branches
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Auto-accept any prompts
    #[arg(short, long, default_value_t = false)]
    yes: bool,

    /// trace-level logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{{ repository: {:?}, dry_run: {}, yes: {}, verbose: {} }}",
            self.repository, self.dry_run, self.yes, self.verbose
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let log_level = if args.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Off
    };
    SimpleLogger::init(log_level, Config::default())
        .with_context(|| "Failed to initialize a simple logger")?;

    info!("Executing with the args {}", args);

    let repository = Repository::open(&args.repository)
        .with_context(|| format!("Failed to open repository at {:?}", args.repository))?;
    trace!("Initialized repository at {:?}", args.repository);

   
     let orphans = repository
         .branches(Some(BranchType::Local))
         .with_context(|| format!("Failed to get local branches for {:?}", args.repository))?
         .map(|r| r.unwrap().0)
         .filter(|b| !has_remote(b));
    
    println!("Going to delete:");
    let mut to_delete: Vec<Branch> = Vec::new();
    for o in orphans {
        let name: &str = o.name()
            .with_context(|| "Failed to get name for a branch")?
            .ok_or(Error::msg("Branch name is not valid UTF-8"))?;
        println!(" - {}", name);
        to_delete.push(o)
    }
    println!("Ok? [Yn]");

    let mut input = std::string::String::new();
    if args.yes {
      input = "y".to_string()
    } else {
        std::io::stdin().read_line(&mut input)?;
    };
    trace!("Read input {}", input);
    
    if input.trim().eq_ignore_ascii_case("y") || input.eq("\n") {
        if !args.dry_run {
            for mut b in to_delete {
                b.delete()?
            }
        }
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
