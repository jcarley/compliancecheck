use crate::models::Core;

use anyhow::Result;

#[derive(Debug, Default)]
pub struct Policy {}

impl Policy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check(&mut self, core: Core) -> Result<PolicyResult> {
        let mut result = PolicyResult::new(core.company.name.clone());

        manager_has_three_employees_check(&core, &mut result)?;
        every_employee_has_a_device_check(&core, &mut result)?;
        minimum_device_version_check(&core, &mut result)?;

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

fn manager_has_three_employees_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
    for manager in core
        .employees
        .iter()
        .filter(|e| e.employee_type == "manager")
    {
        let reports = core
            .employees
            .iter()
            .filter(|e| e.manager_id.as_deref() == Some(manager.id.as_str()))
            .count();

        if reports > 3 {
            result.failing_checks.push(format!(
                "manager {} has {} employees, expected at least 3",
                manager.name, reports
            ));
        }
    }

    Ok(())
}

fn every_employee_has_a_device_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
    for employee in core.employees.iter() {
        let devices = core
            .devices
            .iter()
            .filter(|d| d.employee_id == employee.id)
            .count();

        if devices == 0 {
            result.failing_checks.push(format!(
                "employee {} is not assigned a device",
                employee.name
            ));
        }
    }

    Ok(())
}

fn minimum_device_version_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
    for device in core.devices.iter() {
        if device.operating_system.to_lowercase() == "macos" && device.version < 11.5 {
            let employee = core
                .employees
                .iter()
                .find(|e| e.id == device.employee_id)
                .unwrap();

            result.failing_checks.push(format!(
                "device for employee {} has operating system {} with a version {}",
                employee.name, device.operating_system, device.version
            ));
        }
    }

    Ok(())
}
