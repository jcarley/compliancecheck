use crate::models::{Core, Version};
use crate::policies::PolicyResult;

use anyhow::Result;

pub fn manager_has_no_less_than_three_employees_check(
    core: &Core,
    result: &mut PolicyResult,
) -> Result<()> {
    for manager in core
        .employees
        .iter()
        .filter(|e| e.employee_type.eq_ignore_ascii_case("manager"))
    {
        let reports = core
            .employees
            .iter()
            .filter(|e| e.manager_id.as_deref() == Some(manager.id.as_str()))
            .count();

        if reports < 3 {
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

const MINIMUM_MACOS_VERSION: Version = Version::new(11, 5, 0);

pub fn minimum_device_version_check(core: &Core, result: &mut PolicyResult) -> Result<()> {
    for device in core.devices.iter() {
        match core.employees.iter().find(|e| e.id == device.employee_id) {
            Some(e) => {
                if !device.operating_system.eq_ignore_ascii_case("macos") {
                    continue;
                }

                match Version::parse(&device.version) {
                    Ok(version) if version < MINIMUM_MACOS_VERSION => {
                        result.failing_checks.push(format!(
                            "device {} for employee {} runs macOS {version}, minimum is {MINIMUM_MACOS_VERSION}",
                            device.id, e.name
                        ));
                    }
                    Ok(_) => {}
                    Err(error) => {
                        result.failing_checks.push(format!(
                            "device {} for employee {} has an unreadable version: {error:#}",
                            device.id, e.name
                        ));
                    }
                }
            }
            None => {
                result.failing_checks.push(format!(
                    "device {} is not assigned to an employee",
                    device.id
                ));
            }
        };
    }

    Ok(())
}
