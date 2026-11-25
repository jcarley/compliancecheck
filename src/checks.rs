use crate::models::Core;
use crate::policies::PolicyResult;

use anyhow::Result;

pub fn manager_has_three_employees_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
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

pub fn every_employee_has_a_device_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
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

pub fn minimum_device_version_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
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
