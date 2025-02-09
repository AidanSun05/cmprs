mod args;
mod compress;
mod compress_jpg;
mod compress_png;
mod files;

use args::Args;
use clap::Parser;
use colored::Colorize;
use compress::compress_with_output;
use crossbeam_deque::{Injector, Stealer, Worker};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Clone)]
struct SharedData {
    injector: Arc<Injector<String>>,
    stealers: Arc<Vec<Stealer<String>>>,
    orig_sizes: Arc<Mutex<Vec<usize>>>,
    new_sizes: Arc<Mutex<Vec<usize>>>,
    args: Arc<Args>,
}

fn scope_fn(
    injector: Arc<Injector<String>>,
    stealers: Arc<Vec<Stealer<String>>>,
    worker: Worker<String>,
    args: Arc<Args>,
) -> (usize, usize) {
    let mut local_orig = 0;
    let mut local_new = 0;

    loop {
        if let Some(task) = worker.pop() {
            // Process tasks from the local queue
            compress_with_output(&task, &mut local_orig, &mut local_new, &args);
        } else if let Some(task) = injector.steal_batch_and_pop(&worker).success() {
            // Steal tasks from the global injector
            compress_with_output(&task, &mut local_orig, &mut local_new, &args);
        } else {
            // Steal tasks from other threads
            let mut stolen_task = None;
            for stealer in stealers.iter() {
                if let Some(task) = stealer.steal().success() {
                    stolen_task = Some(task);
                    break;
                }
            }
            if let Some(task) = stolen_task {
                compress_with_output(&task, &mut local_orig, &mut local_new, &args);
            } else {
                break; // No more tasks
            }
        }
    }

    (local_orig, local_new)
}

fn main() {
    let args = Args::parse();
    let paths: HashSet<_> = args.paths.iter().flat_map(|s| files::get_glob(s)).collect();

    if paths.is_empty() {
        println!("No input files, exiting.");
        return;
    }

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let num_workers = std::cmp::min(args.jobs.unwrap_or(num_threads), paths.len());
    println!("Compression with up to {} threads.", num_workers);

    // Create local worker queues for each thread
    let mut workers = vec![];
    let mut stealers = vec![];

    for _ in 0..num_workers {
        let worker = Worker::<String>::new_fifo();
        stealers.push(worker.stealer());
        workers.push(worker);
    }

    // Create a global injector
    let shared = SharedData {
        injector: Arc::new(Injector::new()),
        stealers: Arc::new(stealers),
        orig_sizes: Arc::new(Mutex::new(vec![0; num_workers])),
        new_sizes: Arc::new(Mutex::new(vec![0; num_workers])),
        args: Arc::new(args),
    };

    // Populate the global injector with tasks
    for i in paths {
        shared.injector.push(i);
    }

    let start = Instant::now();

    thread::scope(|scope| {
        for (i, worker) in workers.into_iter().enumerate() {
            let shared = shared.clone();

            scope.spawn(move || {
                let (local_orig, local_new) =
                    scope_fn(shared.injector, shared.stealers, worker, shared.args);

                // Update size counts
                shared.orig_sizes.lock().unwrap()[i] = local_orig;
                shared.new_sizes.lock().unwrap()[i] = local_new;
            });
        }
    });

    let duration = start.elapsed();
    let total_orig: usize = shared.orig_sizes.lock().unwrap().iter().sum();
    let total_new: usize = shared.new_sizes.lock().unwrap().iter().sum();
    let diff = total_orig - total_new;
    let (formatted_size, size_prefix) = files::format_size(diff);

    let saved_percent = if total_orig == 0 {
        0.0
    } else {
        (diff as f64 / total_orig as f64) * 100.0
    };

    println!(
        "{}: {:?}, {}: {:.2} {}B ({:.2}%)",
        "Total time".green().bold(),
        duration,
        "saved".green().bold(),
        formatted_size,
        size_prefix,
        saved_percent
    );
}
