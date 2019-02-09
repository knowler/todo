use structopt::StructOpt;

use todo::{Cli, Cli::*, Project, Task};

fn main() {
    match Cli::from_args() {
        New { task } => {
            Task::create(task);
        }
        Start { project } => {
            Project::create(project);
        }
        Stop => println!("Stopping project"),
        Focus { task } => println!("{}", task),
        Unfocus { complete } => println!("Unfocusing task"),
    };
}
