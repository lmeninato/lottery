use rand::distributions::WeightedIndex;
use rand::prelude::*;

use std::collections::HashMap;

mod parse;
mod user;

pub use user::User;

fn inspect(user: &User) {
    println!("User: {} ({} tickets) has jobs:", user.name, user.tickets);

    for (i, job) in user.jobs.iter().enumerate() {
        println!("Job {}", i);
        println!("  Work: {}", job.work);
        println!("  Remaining: {}", job.remaining);
    }
    println!("");
}

fn inspect_users(users: &Vec<User>) {
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

fn compute_lottery_winner(users: &mut Vec<User>) -> &mut User {
    let weights: Vec<usize> = users.iter().map(|user| user.tickets).collect();
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(&weights).unwrap();

    &mut users[dist.sample(&mut rng)]
}

fn print_summary(
    num_lotteries: usize,
    total_work: usize,
    winner_map: HashMap<String, usize>,
    expected_wins: HashMap<String, f32>,
    work_map: HashMap<String, usize>,
) {
    println!("There were {} lotteries", num_lotteries);

    for (winner, num_wins) in &winner_map {
        let expected = expected_wins.get(winner).unwrap();
        println!(
            "User: {} won {} times (expected {:.2})",
            winner, num_wins, expected
        );

        let work = work_map.get(winner).unwrap();
        let cpu_share = 100.0 * (*work as f32) / total_work as f32;
        println!(
            "User: {} performed {} units of work ({:.2}%)",
            winner, work, cpu_share
        );
    }
}

fn get_expected_wins(iterations: usize, users: &mut Vec<User>) -> HashMap<String, f32> {
    let mut map_expected_wins: HashMap<String, f32> = HashMap::new();
    // wlog assume users have the same amount of total work
    let total_tickets = users
        .iter_mut()
        .map(|user| user.get_tickets())
        .fold(0, |sum, i| sum + i);

    for user in users {
        let expected_wins = (user.tickets as f32 / total_tickets as f32) * (iterations as f32);
        map_expected_wins.insert(user.get_name(), expected_wins);
    }

    map_expected_wins
}

fn simulate_lottery_scheduling(
    quantum: usize,
    iterations: usize,
    users: &mut Vec<User>,
    compensatory: bool,
) {
    let mut total_work: usize = 0;
    let mut winners_over_time: Vec<(usize, String)> = vec![];
    let mut num_lotteries: usize = 0;
    let mut times_won: HashMap<String, usize> = HashMap::new();
    let mut work_map: HashMap<String, usize> = HashMap::new();
    let expected_wins = get_expected_wins(iterations, users);
    let mut i = 0;

    while work_queue_is_not_empty(users) && i < iterations {
        let winner = compute_lottery_winner(users);
        let remainder = winner.do_work(quantum, compensatory);
        let work_performed = quantum - remainder;

        println!(
            "At time: {}, the winner was: {}",
            total_work,
            winner.get_name()
        );
        *times_won.entry(winner.get_name()).or_insert(0) += 1;
        *work_map.entry(winner.get_name()).or_insert(0) += work_performed;

        winners_over_time.push((total_work, winner.get_name()));

        total_work += work_performed;

        num_lotteries += 1;
        i += 1;
    }

    print_summary(
        num_lotteries,
        total_work,
        times_won,
        expected_wins,
        work_map,
    );
}

fn main() {
    let (comp, quantum, iterations, mut users) = parse::get_user_input();

    println!("\nQuantum is {} (Comp tickets set to: {}) \n", quantum, comp);

    inspect_users(&users);

    simulate_lottery_scheduling(quantum, iterations, &mut users, comp);
}
