use anyhow::{Context, Error, Result};
use clap::Parser;
use git2::{Branch, BranchType, Repository};
use log::{debug, error, info, trace, warn};
use regex::Regex;
use simplelog::{Config, LevelFilter, SimpleLogger};
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
    verbose: bool,
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ repository: {:?}, dry_run: {}, yes: {}, verbose: {} }}",
            self.repository, self.dry_run, self.yes, self.verbose
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let safe_prefixes = vec!["vault/"];

    let log_level = if args.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Off
    };
    SimpleLogger::init(log_level, Config::default())
        .with_context(|| "Failed to initialize a simple logger")?;

    info!("Executing with the args {}", args);
    info!("Ignore branches with prefixes {:?}", safe_prefixes);

    let repository = Repository::open(&args.repository)
        .with_context(|| format!("Failed to open repository at {:?}", args.repository))?;
    trace!("Initialized repository at {:?}", args.repository);

    let orphans = repository
        .branches(Some(BranchType::Local))
        .with_context(|| format!("Failed to get local branches for {:?}", args.repository))?
        .map(|r| r.unwrap().0)
        .filter(|b| !has_remote(b));

    let mut to_delete: Vec<Branch> = Vec::new();
    for o in orphans {
        let name: &str = o
            .name()
            .with_context(|| "Failed to get name for a branch")?
            .ok_or(Error::msg("Branch name is not valid UTF-8"))?;

        let is_protected = is_protected_branch(name, &safe_prefixes)
            .with_context(|| format!("failed to check if {} is a protected branch", name))?;
        if !is_protected {
            println!("{}", name);
            to_delete.push(o)
        }
    }

    let mut input = std::string::String::new();
    if args.yes {
        input = "y".to_string()
    } else {
        println!("Ok? [Yn]");
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

fn is_protected_branch(branch_name: &str, safety_prefixes: &Vec<&str>) -> Result<bool> {
    // Do not use RegexSet because it cannot be used to find the position of the
    // match to guarantee a prefix match
    for safety_prefix in safety_prefixes.into_iter() {
        let regex = Regex::new(safety_prefix).with_context(|| {
            format!(
                "{} cannot be used as a safety prefix because it is not valid regex",
                safety_prefix
            )
        })?;

        match regex.find(branch_name) {
            Some(mat) => {
                info!("Protecting {}", branch_name);
                return Ok(mat.start() == 0);
            }
            None => {
                warn!("Not protecting {}", branch_name);
                continue;
            },
        }
    }

    Ok(false)
}
