use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Core {
    pub company: Company,
    pub employees: Vec<Employee>,
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub employee_type: String,
    pub manager_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub employee_id: String,
    pub operating_system: String,
    pub version: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        let mut parts = value.trim().split('.');
        let mut next = |name: &str| -> Result<u32> {
            match parts.next() {
                None => Ok(0),
                Some(p) => p.parse().with_context(|| {
                    format!("invalid {name} component {p:?} in version {value:?}")
                }),
            }
        };

        let version = Self {
            major: next("major")?,
            minor: next("minor")?,
            patch: next("patch")?,
        };

        if parts.next().is_some() {
            bail!("version {value:?} has more than three components");
        }

        Ok(version)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
