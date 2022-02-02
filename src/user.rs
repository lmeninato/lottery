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
            if self.jobs.len() == 0 {
                // User can no longer win lotteries
                // If job was added back to queue, would need to update this value.
                self.tickets = 0;
            }

            return quantum - job.remaining;
        }

        job.remaining -= quantum;
        self.jobs.push(job);

        0
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_total_work(&self) -> usize {
        let mut work = 0;
        for job in &self.jobs {
            work += job.work;
        }
        work
    }
}
