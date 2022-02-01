#[derive(Debug)]
pub struct Job {
    pub work: usize,
    pub remaining: usize
}

pub fn create_job(work: usize) -> Job {
    Job {
        work: work,
        remaining: work
    }
} 

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub jobs: Vec<Job>,
    pub tickets: usize,
}

