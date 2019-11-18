
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct JobID(usize);

impl std::fmt::Display for JobID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job({})", self.0)
    }
}
impl From<JobID> for usize {
	fn from(id: JobID) -> usize {
		id.0
	}
}

pub struct Job {
	id: JobID,
}
