use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

use crate::placeholder::insert::placeholder_insert_contracts::PlaceholderInsertTileContract;
use crate::terminal::terminal_enums::TerminalHistoryPersistence;

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub tab_name: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
	pub history_persistence: TerminalHistoryPersistence,
	pub command_tiles: Vec<PlaceholderInsertTileContract>,
	pub working_dir_tiles: Vec<PlaceholderInsertTileContract>,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfoContract {
	pub id: Uuid,
	pub project_id: Uuid,
	pub name: String,
	pub date_created: DateTime<Utc>,
	pub date_last_updated: DateTime<Utc>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreateContract {
	pub project_id: Uuid,
	pub name: String,
	pub tab_name: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
	pub history_persistence: TerminalHistoryPersistence,
	pub command_tiles: Vec<PlaceholderInsertTileContract>,
	pub working_dir_tiles: Vec<PlaceholderInsertTileContract>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateContract {
	pub id: Uuid,
	pub name: String,
	pub tab_name: Option<String>,
	pub no_exit: bool,
	pub force_kill: bool,
	pub history_persistence: TerminalHistoryPersistence,
	pub command_tiles: Vec<PlaceholderInsertTileContract>,
	pub working_dir_tiles: Vec<PlaceholderInsertTileContract>,
}
