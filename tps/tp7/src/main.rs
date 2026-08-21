mod simulation;
mod leibniz;

use std::env;
use std::process;
use std::time::Instant;
use crate::simulation::{AsyncSimulation, Simulation, SimulationType, ThreadSimulation};

fn print_usage(program: &str) {
    eprintln!(
        "Usage:\n  {0} --type <async|thread> --operation <leibniz|io> --task-number <N>\n\n\
         Flags:\n  --type         Execution model (\"async\" or \"thread\")\n\
         --operation    Operation to perform (\"leibniz\" or \"io\")\n\
         --task-number  Number of tasks to perform (positive integer)",
        program
    );
}

fn get_arg(args: &[String], key: &str, prog: &str) -> String {
    for i in 0..args.len() {
        if args[i] == key {
            if let Some(val) = args.get(i + 1) {
                return val.clone();
            } else {
                eprintln!("Missing value for `{}`", key);
                print_usage(prog);
                process::exit(1);
            }
        }
    }
    eprintln!("Missing or invalid `{}`", key);
    print_usage(prog);
    process::exit(1);
}

#[tokio::main]
async fn main(){
    let before_exec = Instant::now();
    operate().await;
    let current = before_exec.elapsed();
    println!("Elapsed time: `{}`", current.as_millis())
}

async fn operate() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];
    let async_simulation = AsyncSimulation{};
    let thread_simulation = ThreadSimulation{};

    let exec_type = SimulationType::from_string(get_arg(&args, "--type", prog));
    let operation = get_arg(&args, "--operation", prog);
    match exec_type {
        SimulationType::ASYNC => match operation.as_str() {
            "leibniz" => {
                let (term, divisions) = get_term_and_divisions(&args, prog);
                async_simulation.calculate_leibniz_term(term,divisions).await
            },
            "io" => async_simulation.simulate_io_tasks(get_task_number(&args, prog)).await,
            _ => {
                eprintln!("Invalid operation: `{}`", operation);
                print_usage(prog);
                process::exit(1);
            }
        },
        SimulationType::THREAD => match operation.as_str() {
            "leibniz" => {
                let (term, divisions) = get_term_and_divisions(&args, prog);
                thread_simulation.calculate_leibniz_term(term, divisions).await
            }
            "io" => {
                thread_simulation.simulate_io_tasks(get_task_number(&args, prog)).await;
            }
            _ => {
                eprintln!("Invalid operation: `{}`", operation);
                print_usage(prog);
                process::exit(1);
            }
        },
    }
}

fn get_term_and_divisions(args: &[String], prog: &str) -> (usize, usize) {
    let term: usize = get_arg(args, "--term", prog).parse().unwrap_or_else(|_|{
        eprintln!("`--term` must be a positive integer");
        print_usage(prog);
        process::exit(1)
    });

    let divisions: usize = get_arg(&args, "--divisions", prog).parse().unwrap_or_else(|_|{
        eprintln!("`--divisions` must be a positive integer");
        print_usage(prog);
        process::exit(1)
    });

    (term, divisions)
}

fn get_task_number(args: &[String], prog: &str) -> usize {
    let task_number: usize = get_arg(&args, "--task-number", prog)
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("`--task-number` must be a positive integer");
            print_usage(prog);
            process::exit(1)
        });
    task_number
}