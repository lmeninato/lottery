#[derive(Debug, Clone)]
pub struct Job {
    pub work: usize,
    pub remaining: usize,
}

pub fn create_job(work: usize) -> Job {
    Job {
        work: work,
        remaining: work,
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub jobs: Vec<Job>,
    pub tickets: usize,
}

impl User {
    pub fn do_work(&mut self, quantum: usize) -> usize {
        let mut job = self.jobs.pop().unwrap();

        if quantum > job.remaining {
            return quantum - job.remaining;
        }

        job.remaining -= quantum;

        0
    }
}
