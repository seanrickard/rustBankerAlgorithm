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
    let mut num = file.len() / 2;
    let mut counter = 0;

    let mut allocated: VecDeque<Vec<i16>> = VecDeque::new();
    let mut required: VecDeque<Vec<i16>> = VecDeque::new();
    let mut procs: VecDeque<Process> = VecDeque::with_capacity(5);

    while file.len() != 0 {
        if counter <= file.len() || num == 1 {
            allocated.push_back(file.remove(0));
            required.push_back(file.remove(num - 1));
            procs.push_back(Process {
                id: counter,
                need: Vec::new(),
                allocated: allocated.pop_front().unwrap(),
                required_resources: required.pop_front().unwrap(),
                safe_state: false,
            });
            num = num - 1;
            counter += 1;
        }
    }

    let mut finished: VecDeque<Process> = VecDeque::with_capacity(5);
    let mut non_runnable: VecDeque<Process> = VecDeque::with_capacity(5);

    procs = calc_need(procs);
    let total_procs = procs.clone().len();
    while &finished.len() < &(total_procs) {
        for x in procs.clone().iter_mut() {
            x.safe_state = check_safe_state(&x.need, &available);
            let temp = x.clone();

            if x.safe_state == true {
                available = update_aval(&x.allocated, &available);
                finished.push_back(temp);
                procs.pop_front();
            }
            if x.safe_state == false {
                non_runnable.push_front(procs.pop_front().unwrap());
                break;
            }
            if procs.len() == 0 && non_runnable.len() != 0 {
                procs.push_front(non_runnable.pop_front().unwrap());
                // println!("{:?}", procs);
                break;
            }
        }
        println!("Available resources: {:?}", available);

    }
    println!("Safe order: " );

    print_process_list(finished);

}

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    id: usize,
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
