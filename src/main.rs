mod parse;
mod user;

pub use user::User;

fn inspect(user: User){
    println!("User: {} ({} tickets) has jobs:", user.name, user.tickets);

    for (i, job) in user.jobs.iter().enumerate() {
        println!("Job {}", i);
        println!("  Total work to do: {}", job.work);
        println!("  Work remaining: {}", job.remaining);
    }
    println!("");
}

fn main() {
    let (quantum, users) = parse::get_user_input();

    println!("\nQuantum is {}\n", quantum);

    for user in users {
        inspect(user);
    }
}
