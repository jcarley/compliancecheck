mod models;
mod policies;

use crate::models::Core;
use crate::policies::Policy;

use anyhow::{Context, Result};
use std::error::Error;
use std::{fs, path::Path};

fn load(path: &Path) -> Result<Core> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    serde_json::from_str(&contents).with_context(|| format!("failed to parse {}", path.display()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let core: Core = load(Path::new("data.json"))?;

    let mut policy = Policy::new();
    let mut policy_result = policy.check(core)?;

    println!("Company: {}", policy_result.company);
    println!("Policy Result: {}", policy_result.success());

    Ok(())
}
