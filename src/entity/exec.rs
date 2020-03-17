use super::JobID;
use crate::world::Path;

#[derive(Debug)]
pub struct Exec {
	path: Path,
	job: JobID,
}
