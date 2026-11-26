mod checks;
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

    let policy = Policy::new(vec![
        checks::manager_has_no_less_than_three_employees_check,
        checks::every_employee_has_a_device_check,
        checks::minimum_device_version_check,
    ]);
    let mut policy_result = policy.check(core)?;

    println!("Company: {}", policy_result.company);
    println!("Policy Result: {}", policy_result.success());

    Ok(())
}
