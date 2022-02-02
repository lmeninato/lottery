use rand::distributions::WeightedIndex;
use rand::prelude::*;

mod parse;
mod user;

pub use user::User;

fn inspect(user: User) {
    println!("User: {} ({} tickets) has jobs:", user.name, user.tickets);

    for (i, job) in user.jobs.iter().enumerate() {
        println!("Job {}", i);
        println!("  Total work to do: {}", job.work);
        println!("  Work remaining: {}", job.remaining);
    }
    println!("");
}

fn inspect_users(users: Vec<User>) {
    for user in users {
        inspect(user);
    }
}

fn work_queue_is_not_empty(users: &Vec<User>) -> bool {
    for user in users {
        if user.jobs.len() > 0 {
            return true;
        }
    }

    false
}

fn compute_lottery_winner<'a>(users: &'a mut Vec<User>) -> &mut User {
    let weights: Vec<usize> = users.iter().map(|user| user.tickets).collect();
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(&weights).unwrap();

    &mut users[dist.sample(&mut rng)]
}

fn simulate_lottery_scheduling(quantum: usize, users: &mut Vec<User>, compensatory: bool) {
    let mut total_work: usize = 0;

    while work_queue_is_not_empty(users) {
        let mut winner = compute_lottery_winner(users);
        let remainder = winner.do_work(quantum);

        total_work += quantum-remainder;

        if compensatory {
            // handle case with compensatory tickets
        } else {

        }
    }

    println!("Total work is: {}", total_work);
}

fn main() {
    let (quantum, mut users) = parse::get_user_input();

    println!("\nQuantum is {}\n", quantum);

    simulate_lottery_scheduling(quantum, &mut users, false);

    inspect_users(users);
}
