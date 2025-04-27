mod algorithms;
mod report;
mod runner;
mod dataset;
mod metrics;

use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use crate::algorithms::{
    base::SortAlgorithm,
    bubble::BubbleSort,
    insertion::InsertionSort,
    merge::MergeSort,
    quick::QuickSort,
    selection::SelectionSort,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(
        short = 's',
        long = "sizes",
        value_delimiter = ',',
        default_value = "1000,100000"
    )]
    sizes: Vec<usize>,
    #[arg(short, long, default_value = "rust_results.json")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let algorithms: Vec<Box<dyn SortAlgorithm>> = vec![
        Box::new(InsertionSort),
        Box::new(SelectionSort),
        Box::new(BubbleSort),
        Box::new(MergeSort),
        Box::new(QuickSort),
    ];

    let mp = MultiProgress::new();
    let bar_style = ProgressStyle::with_template(
        "{spinner:.yellow} {msg:<15} {bar:40.cyan/blue} {pos}/{len} {elapsed_precise}",
    )?
        .progress_chars("█▉▊▋▌▍▎▏  ")
        .tick_chars("⠋⠙⠚⠞⠖⠦⠴⠲⠳⠓");

    use std::sync::Mutex;
    let results = Mutex::new(Vec::new());

    rayon::scope(|s| {
        for algorithm in &algorithms {
            let sizes = args.sizes.clone();
            let algo = algorithm.as_ref();
            let pb = mp.add(ProgressBar::new(args.sizes.len() as u64));
            let results = &results;
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            pb.set_style(bar_style.clone());
            pb.set_message(format!("{:<15}", algo.name()));

            s.spawn(move |_| {
                for (idx, &n) in sizes.iter().enumerate() {
                    pb.set_message(format!("{:<15} (N = {})", algo.name(), n));
                    let rec = runner::run_one(algo, n).expect("Benchmark failed.");
                    results.lock().unwrap().push(rec);
                    pb.inc(1);
                    pb.set_position(idx as u64 + 1);
                }
                pb.finish_with_message(format!("{:<15} ✔ finished", algo.name()));
            });
        }
    });

    mp.clear()?;
    let guard = results.into_inner().unwrap();

    report::to_json(&args.output, &guard)?;
    println!("Saved { } results to {}.", guard.len(), args.output);
    println!("Done!");
    Ok(())
}

#[cfg(test)]
mod tests;