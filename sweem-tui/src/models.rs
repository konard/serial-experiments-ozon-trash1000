//! Domain models for the SWEeM API.
//!
//! These structs match the OpenAPI schema and use serde for JSON deserialization.
//! DateOnly from C# is mapped to NaiveDate in Rust.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "i32", into = "i32")]
pub enum Role {
    User = 0,
    Admin = 1,
}

impl From<i32> for Role {
    fn from(value: i32) -> Self {
        match value {
            1 => Role::Admin,
            _ => Role::User,
        }
    }
}

impl From<Role> for i32 {
    fn from(role: Role) -> Self {
        role as i32
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

/// Client data transfer object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub address: Option<String>,
    pub projects_total: i32,
    pub projects_completed: i32,
}

impl ClientDto {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unnamed Client")
    }
}

/// Project data transfer object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDto {
    pub id: Uuid,
    pub client_id: Uuid,
    pub name: Option<String>,
    pub start_date: NaiveDate,
    pub planned_end_date: NaiveDate,
    pub actual_end_date: Option<NaiveDate>,
    pub manager_id: Uuid,
}

impl ProjectDto {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unnamed Project")
    }

    /// Calculate project duration in days
    pub fn duration_days(&self) -> i64 {
        (self.planned_end_date - self.start_date).num_days()
    }

    /// Check if project is completed
    pub fn is_completed(&self) -> bool {
        self.actual_end_date.is_some()
    }

    /// Check if project is overdue (past planned end date but not completed)
    pub fn is_overdue(&self) -> bool {
        if self.is_completed() {
            return false;
        }
        let today = chrono::Local::now().date_naive();
        today > self.planned_end_date
    }
}

/// User data transfer object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub login: Option<String>,
    pub role: Role,
}

impl UserDto {
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unnamed User")
    }
}

/// Generic paginated result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResult<T> {
    pub items: Option<Vec<T>>,
    pub page: i32,
    pub page_size: i32,
    pub total_count: i32,
    pub total_pages: i32,
    pub has_previous: bool,
    pub has_next: bool,
}

impl<T> PaginatedResult<T> {
    pub fn items(&self) -> &[T] {
        self.items.as_deref().unwrap_or(&[])
    }
}

/// Problem details for API error responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: Option<String>,
    pub title: Option<String>,
    pub status: Option<i32>,
    pub detail: Option<String>,
    pub instance: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_serialization() {
        assert_eq!(Role::from(0), Role::User);
        assert_eq!(Role::from(1), Role::Admin);
        assert_eq!(i32::from(Role::User), 0);
        assert_eq!(i32::from(Role::Admin), 1);
    }

    #[test]
    fn test_project_duration() {
        let project = ProjectDto {
            id: Uuid::new_v4(),
            client_id: Uuid::new_v4(),
            name: Some("Test".to_string()),
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            planned_end_date: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            actual_end_date: None,
            manager_id: Uuid::new_v4(),
        };
        assert_eq!(project.duration_days(), 30);
    }
}
