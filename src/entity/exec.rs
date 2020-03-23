use super::JobID;
use crate::world::Path;

#[derive(Debug)]
pub struct Exec {
	pub path: Path,
	pub job: JobID,
}
