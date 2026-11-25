use serde::{Deserialize, Serialize};

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
    pub version: f32,
}
