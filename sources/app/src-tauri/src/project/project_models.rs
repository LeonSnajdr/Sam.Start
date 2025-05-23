use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct ProjectModel {
	pub id: Uuid,
	pub name: String,
	pub favorite: bool,
	pub quick_switch_keybind: Option<String>,
	pub date_created: DateTime<Utc>,
	pub date_last_opened: DateTime<Utc>,
}

pub struct ProjectUpdateModel {
	pub id: Uuid,
	pub name: String,
	pub favorite: bool,
	pub quick_switch_keybind: Option<String>,
}
