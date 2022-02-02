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
    pub comp: usize,
}

impl User {
    pub fn do_work(&mut self, quantum: usize, compensatory: bool) -> usize {
        let mut job = self.jobs.pop().unwrap();

        if quantum > job.remaining {
            let remainder = quantum - job.remaining;

            if self.jobs.len() == 0 {
                // User can no longer win lotteries
                // If job was added back to queue, would need to update this value.
                self.tickets = 0;
            }

            if compensatory {
                self.comp += quantum/remainder;
            }

            return remainder;
        }

        job.remaining -= quantum;
        self.jobs.push(job);

        0
    }

    pub fn get_tickets(&mut self) -> usize {
        if self.tickets == 0 {
            // no work to do... no need to enter lotteries
            return 0;
        }

        // use up comp tickets (if not allowing comp tickets, comp is always 0)
        let tickets = self.tickets + self.comp;
        self.comp = 0;

        tickets
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
