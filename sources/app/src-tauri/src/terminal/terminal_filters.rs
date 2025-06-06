use crate::terminal::terminal_enums::TerminalShellStatus;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalFilter {
	pub id: Option<Uuid>,
	pub project_id: Option<Uuid>,
	pub task_ids: Option<Vec<Uuid>>,
	pub shell_status: Option<Vec<TerminalShellStatus>>,
}
impl Default for TerminalFilter {
	fn default() -> Self {
		Self {
			id: None,
			project_id: None,
			task_ids: None,
			shell_status: None,
		}
	}
}
