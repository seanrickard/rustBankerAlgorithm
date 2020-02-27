use std::collections::VecDeque;
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

    // println!("Available resources: {:?}", available);

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

    let mut procs: VecDeque<Process> = VecDeque::with_capacity(5);
    let mut finished: VecDeque<Process> = VecDeque::with_capacity(5);
    let mut non_runnable: VecDeque<Process> = VecDeque::with_capacity(5);

    procs.push_front(process_one.clone());
    procs.push_back(process_two.clone());
    procs.push_back(process_three.clone());
    procs.push_back(process_four.clone());
    procs.push_back(process_five.clone());

    procs = calc_need(procs);
    let total_procs = procs.clone().len();
    while &finished.len() < &(total_procs) {
        for x in procs.clone().iter_mut() {
            //println!("Checking process  {:?} {}", counter, "if safe to run.");
            //println!("{:?}", x);
            //println!("{:?}", available);
            x.safe_state = check_safe_state(&x.need, &available);
            let temp = x.clone();

            println!("{:?}", x.safe_state);
            if x.safe_state == true {
                //println!("{:?}", x);
                available = update_aval(&x.allocated, &available);
                finished.push_back(temp);
                procs.pop_front();
                // println!("{:?}", available);
                println!("Finished: {:?}", finished);
                println!("Processes: {:?}", procs);
                println!("Processes: {:?}", available);
            }
            if x.safe_state == false {
                non_runnable.push_front(procs.pop_front().unwrap());
                break;
            }
            if procs.len() == 0 && non_runnable.len() != 0 {
                procs.push_front(non_runnable.pop_front().unwrap());
                println!("{:?}", procs);
                break;
            }

            //println!("{}", counter);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    name: String,
    allocated: Vec<i16>,
    required_resources: Vec<i16>,
    need: Vec<i16>,
    safe_state: bool,
}

pub fn calc_need(processes: VecDeque<Process>) -> VecDeque<Process> {
    let mut procs = processes.clone();
    for x in procs.iter_mut() {
        for (j, k) in x.allocated.iter().zip(x.required_resources.iter()) {
            let temp = k - j;
            x.need.push(temp);
        }
    }
    procs
}

pub fn print_process_list(processes: VecDeque<Process>) {
    let procs = processes.clone();
    for x in procs.iter() {
        println!("{:?}", x);
    }
}

pub fn update_aval(allocated: &Vec<i16>, available: &Vec<i16>) -> Vec<i16> {
    let mut new_aval: Vec<i16> = Vec::new();
    for (x, y) in allocated.iter().zip(available.iter()) {
        let temp = *x + *y;
        new_aval.push(temp);
    }

    new_aval
}

pub fn check_safe_state(need: &Vec<i16>, available: &Vec<i16>) -> bool {
    let mut counter = 0;
    let mut state: bool = false;

    for (x, y) in need.iter().zip(available.iter()) {
        if x <= y {
            //println!("here");
            if counter == need.len() - 1 {
                state = true;
            }
            counter += 1;
        }
    }

    state
}
