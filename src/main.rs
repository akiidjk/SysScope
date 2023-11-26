mod utilities;

use std::process::exit;
use std::sync::atomic::Ordering;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

use clap::Parser;
use colored::Colorize;
use sysinfo::*;
use indicatif::{ProgressBar, ProgressStyle};
use ctrlc;

use crate::utilities::*;


#[derive(Parser, Debug)]
#[clap(name = "ProjectVirgil", version = "0.1.0", about = "A simple benchmark tool for see the resource consumption of a process")]
#[command(author, version, about, long_about = None)]

struct Args {
    #[arg(name="pid",short, long, default_value = "0", help="The pid of process to monitoring")] //, about = "The process ID to monitor" error_message = "The pid is an integer not a string"
    pid: Pid,

    #[arg(name="duration",short, long, default_value = "0", help="The duration of benchmark")]
    duration: u32,

    #[arg(name="update",short, long, default_value = "200", help="The time of update in ms")]
    update: u64,
}
static  BANNER: &str = r#"
______                     _     ___  ___              _
| ___ \                   | |    |  \/  |             | |
| |_/ /  ___  _ __    ___ | |__  | .  . |  __ _  _ __ | | __
| ___ \ / _ \| '_ \  / __|| '_ \ | |\/| | / _` || '__|| |/ /
| |_/ /|  __/| | | || (__ | | | || |  | || (_| || |   |   <
\____/  \___||_| |_| \___||_| |_|\_|  |_/ \__,_||_|   |_|\_\
"#;

enum Results {
    Max(f32),
    Average(Vec<f32>),
}


//* MAIN FUNCTION *//
fn display(sys: &mut System, pid: Pid, pb: &ProgressBar, hashmap: &mut HashMap<&str, Results>) {
    sys.refresh_all();
    let number_cores = sys.physical_core_count().unwrap_or(1);
    if let Some(process) = sys.process(pid) {

        let cpu_usage = process.cpu_usage() / number_cores as f32;
        let ram_usage = process.memory() as f32 / 1_048_576.0;
        let ram_usage_percent = process.memory() as f64 / sys.total_memory() as f64 * 100.0;

        // Update max value
        mod_value(cpu_usage, hashmap, true, "max_cpu_percent");
        mod_value(ram_usage, hashmap, true, "max_ram_value");
        mod_value(ram_usage_percent as f32, hashmap, true, "max_ram_percent");

        // Update average value
        mod_value(cpu_usage, hashmap, false, "average_cpu_percent");
        mod_value(ram_usage, hashmap, false, "average_ram_value");
        mod_value(ram_usage_percent as f32, hashmap, false, "average_ram_percent");

        print!(" {} : {} \x1B[K \n","Name".green().bold() , process.name());
        print!("{}",format!("{} : {:.3}% \x1B[K\n","Use CPU".green().bold() , (process.cpu_usage() / number_cores as f32)));
        print!("{}",format!("{} : {:.3}MB \x1B[K \n","Use Memory".green().bold() , (process.memory() as f32 / 1_048_576.0)));
        print!("{}",format!("{} : {:.3}% \x1B[K \n","Use Memory".green().bold() , ram_usage_percent));

        pb.set_position(process.cpu_usage() as u64 / number_cores as u64);

    } else {
        println!("The process with pid {} is terminated or not exist", pid);
    }
}

fn limit(pid: Pid, duration: u32, update: u64){
    let mut sys = System::new_all();
    let mut final_value = HashMap::new();
    final_value.insert("max_cpu_percent", Results::Max(0.0));
    final_value.insert("max_ram_value", Results::Max(0.0));
    final_value.insert("max_ram_percent", Results::Max(0.0));
    final_value.insert("average_cpu_percent", Results::Average(vec![]));
    final_value.insert("average_ram_value", Results::Average(vec![]));
    final_value.insert("average_ram_percent", Results::Average(vec![]));

    let pb = ProgressBar::new(100);
    let bar_style = ProgressStyle::default_bar()
        .progress_chars("##-")
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}%")
        .unwrap();

    pb.set_style(bar_style);

    let duration = Duration::from_secs(duration as u64);
    let start = Instant::now();

    loop {
        display(&mut sys, pid, &pb,&mut final_value);
        write_hashmap(&final_value).expect("Error while writing in the file");
        clear_last_lines(5);
        if start.elapsed() >= duration {
            break;
        }
        thread::sleep(Duration::from_millis(update));
    }
    println!("{}\n{}"," - [FINAL VALUE] - ".yellow(), get_hashmap_string(&final_value));
    remove_file();
    exit(0);
}

fn no_limit(pid: Pid, update: u64){
    let mut sys = System::new_all();
    let pb = ProgressBar::new(100);
    let mut final_value = HashMap::new();
    final_value.insert("max_cpu_percent", Results::Max(0.0));
    final_value.insert("max_ram_value", Results::Max(0.0));
    final_value.insert("max_ram_percent", Results::Max(0.0));
    final_value.insert("average_cpu_percent", Results::Average(vec![]));
    final_value.insert("average_ram_value", Results::Average(vec![]));
    final_value.insert("average_ram_percent", Results::Average(vec![]));
    let bar_style = ProgressStyle::default_bar()
        .progress_chars("##-")
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}%")
        .unwrap();
    pb.set_style(bar_style);
    loop {
        display(&mut sys, pid, &pb, &mut final_value);
        write_hashmap(&final_value).expect("Error while writing the file");
        clear_last_lines(5);
        thread::sleep(Duration::from_millis(update));
    }

}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let args = Args::parse();
    let pid: Pid = args.pid;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        match read_hashmap() {
            Ok(content) => println!("\n {}\n{}"," - [FINAL VALUE] - ".yellow(), content),
            Err(e) => eprintln!("Error while reading the file: {}", e),
        }
        remove_file();
        exit(0)
    }).expect("Error in setting up the CTRL-C handler.");

    while running.load(Ordering::SeqCst) {
        println!("{}", BANNER.cyan());
        match args.duration {
            0 => no_limit(pid, args.update),
            _ => limit(pid, args.duration, args.update),
        }
    }
}


