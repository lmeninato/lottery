use std::io;
use std::str::FromStr;

pub use super::user::{create_job, Job, User};

fn parse_line<T>() -> T
where
    T: FromStr,
{
    let mut a_str = String::new();
    io::stdin()
        .read_line(&mut a_str)
        .expect("Error reading line from stdin");
    let line: String = a_str.chars().filter(|c| !c.is_whitespace()).collect();

    let result = match line.parse::<T>() {
        Ok(v) => Some(v),
        Err(_) => Option::None,
    };

    if let None = result {
        print_help_message_and_exit();
    }

    result.unwrap()
}

fn parse_numbers(n: usize) -> Vec<usize> {
    let mut a_str = String::new();
    io::stdin()
        .read_line(&mut a_str)
        .expect("Error reading line from stdin");
    let a_iter = a_str.split_whitespace();

    let mut result = vec![];

    for num in a_iter {
        match num.parse::<usize>() {
            Ok(n) => result.push(n),
            Err(_) => {}
        }
    }

    if result.len() != n {
        print_help_message_and_exit();
    }

    result
}

fn print_help_message_and_exit() {
    let help_message = "Please input (or via stdin redirection) the scheduler configuration.

Please enter the quantum, users and corresponding jobs and tickets.
For instance:

10
1
Amy 
100
3
250 200 300

Will create the single user Amy with 100 tickets, and three jobs with
respective work times 250, 200, and 300 with a quantum of 10.
";
    println!("{}", help_message);
    panic!("Exiting...");
}

pub fn get_user_input() -> (usize, Vec<User>) {
    let quantum = parse_line::<usize>();
    let num_users = parse_line::<usize>();

    let mut users: Vec<User> = vec![];
    for _ in 0..num_users {
        let name = parse_line::<String>();
        let tickets = parse_line::<usize>();
        let num_jobs = parse_line::<usize>();
        let mut jobs: Vec<Job> = parse_numbers(num_jobs)
            .into_iter()
            .map(|work| create_job(work))
            .collect();

        users.push(User {
            name: String::from(name),
            jobs: jobs,
            tickets: tickets,
        });
    }

    (quantum, users)
}
