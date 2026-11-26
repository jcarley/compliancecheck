use crate::models::Core;

use anyhow::Result;

type Check = fn(&Core, &mut PolicyResult) -> Result<()>;

#[derive(Debug)]
pub struct Policy {
    checks: Vec<Check>,
}

impl Policy {
    pub fn new(checks: Vec<Check>) -> Self {
        Self { checks }
    }

    pub fn check(&self, core: Core) -> Result<PolicyResult> {
        let mut result = PolicyResult::new(core.company.name.clone());

        for check in &self.checks {
            check(&core, &mut result)?;
        }

        Ok(result)
    }
}

#[derive(Debug, Default)]
pub struct PolicyResult {
    pub company: String,
    pub failing_checks: Vec<String>,
}

impl PolicyResult {
    pub fn new(c: String) -> Self {
        PolicyResult {
            company: c,
            ..Default::default()
        }
    }

    pub fn success(&mut self) -> bool {
        self.failing_checks.is_empty()
    }
}
