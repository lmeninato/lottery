use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Stats {
    time_running: usize,
    ctx_switches: usize,
}

#[derive(Debug)]
struct Job {
    work: usize,
    remaining: usize
}

fn create_job(work: usize) -> Job {
    Job {
        work: work,
        remaining: work
    }
} 

#[derive(Debug)]
struct User {
    name: String,
    jobs: Vec<Job>,
    tickets: usize,
}

fn inspect(user: User){
    println!("User: {} ({} tickets) has jobs:", user.name, user.tickets);

    for (i, job) in user.jobs.iter().enumerate() {
        println!("Job {}", i);
        println!("  Total work to do: {}", job.work);
        println!("  Work remaining: {}", job.remaining);
    }
    println!("");
}

fn parse_line<T>() -> T where T: FromStr {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Error reading line from stdin");
    let line: String = a_str.chars().filter(|c| !c.is_whitespace()).collect();

    let result = match line.parse::<T>() {
        Ok(v) => Some(v),
        Err(_) => Option::None
    };

    if let None = result {
        print_help_message_and_exit();
    }

    result.unwrap()
}

fn parse_numbers(n: usize) -> Vec<usize> {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Error reading line from stdin");
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
    let help_message = 
"Please enter the quantum, users and corresponding jobs and tickets.
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

fn get_user_input() -> (usize, Vec<User>){
    println!("Please input (or via stdin redirection) the scheduler configuration.");

    let quantum = parse_line::<usize>();
    let num_users = parse_line::<usize>();

    let mut users: Vec<User> = vec![];
    for _ in 0..num_users {
        let name = parse_line::<String>();
        let tickets = parse_line::<usize>();
        let num_jobs = parse_line::<usize>();
        let jobs = parse_numbers(num_jobs)
            .into_iter()
            .map(|work| create_job(work))
            .collect();

        users.push(User {
            name: String::from(name),
            jobs: jobs,
            tickets: tickets
        });
    }

    (quantum, users)
}

fn main() {
    let (quantum, users) = get_user_input();

    println!("Quantum is {}", quantum);

    for user in users {
        inspect(user);
    }
}
