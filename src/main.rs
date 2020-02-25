use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = BufReader::new(File::open("input1.txt").unwrap());
    let mut file: Vec<Vec<i16>> = file
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let mut available = file.remove(0);

    println!("Available resources: {:?}", available);

    let mut process_one = Process {
        name: String::from("process1"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(4),
        safe_state: false,
    };

    let mut process_two = Process {
        name: String::from("process2"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(3),
        safe_state: false,
    };

    let mut process_three = Process {
        name: String::from("process3"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(2),
        safe_state: false,
    };

    let mut process_four = Process {
        name: String::from("process4"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(1),
        safe_state: false,
    };

    let mut process_five = Process {
        name: String::from("process5"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(0),
        safe_state: false,
    };

    let mut procs: Vec<Process> = Vec::with_capacity(5);
    let mut ran_procs: Vec<Process> = Vec::with_capacity(5);

    procs.push(process_one.clone());
    procs.push(process_two);
    procs.push(process_three);
    procs.push(process_four);
    procs.push(process_five);
    let mut procs = calc_need(procs.clone());
    print_process_list(procs.clone());
    let process_one = procs.remove(0);
    let process_one =
        can_process_run_safely(process_one.clone(), available.clone(), procs.clone().len());
    println!("process one: {:?}", process_one.clone());
}

#[derive(Debug, Clone)]
pub struct Process {
    name: String,
    allocated: Vec<i16>,
    required_resources: Vec<i16>,
    need: Vec<i16>,
    safe_state: bool,
}

pub fn calc_need(processes: Vec<Process>) -> Vec<Process> {
    let mut procs = processes.clone();
    for x in procs.iter_mut() {
        for (j, k) in x.allocated.iter().zip(x.required_resources.iter()) {
            let temp = j - k;
            let temp_2 = temp.abs();
            x.need.push(temp_2);
        }
    }
    procs
}

pub fn print_process_list(processes: Vec<Process>) {
    let procs = processes.clone();
    for x in procs.iter() {
        println!("{:?}", x);
    }
}

pub fn can_process_run_safely(process: Process, available: Vec<i16>, length: usize) -> Process {
    let mut counter = 0;
    let mut proc_clone = process.clone();
    let mut aval_clone = available.clone();
    for (j, k) in proc_clone.need.iter_mut().zip(aval_clone.iter_mut()) {
        if j <= k {
            counter += 1;
            if counter == length {
                proc_clone.safe_state = true;
            }
        } else if j > k {
            break;
        }
    }

    proc_clone
}
