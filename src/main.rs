use std::fs::File;
use std::fmt;
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
        name: String::from("\nprocess1"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(4),
        safe_state: false,
    };

    let mut process_two = Process {
        name: String::from("\nprocess2"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(3),
        safe_state: false,
    };

    let mut process_three = Process {
        name: String::from("\nprocess3"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(2),
        safe_state: false,
    };

    let mut process_four = Process {
        name: String::from("\nprocess4"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(1),
        safe_state: false,
    };

    let mut process_five = Process {
        name: String::from("\nprocess5"),
        need: Vec::new(),
        allocated: file.remove(0),
        required_resources: file.remove(0),
        safe_state: false,        
    };

    // println!("{:?}", process_one);
    // println!("{:?}", process_two);
    // println!("{:?}", process_three);
    // println!("{:?}", process_four);
    // println!("{:?}", process_five);

    let mut procs: Vec<Process> = Vec::with_capacity(5);
    procs.push(process_one);
    procs.push(process_two);
    procs.push(process_three);
    procs.push(process_four);
    procs.push(process_five);
    let procs = calc_need(procs.clone());
    println!("{:?}", procs);
}

#[derive(Debug, Clone)]
pub struct Process {
    name: String,
    allocated: Vec<i16>,
    required_resources: Vec<i16>,
    need: Vec<i16>,
    safe_state: bool,
}

#[derive(Debug, Clone)]
pub struct FinishedProc {
    available: Vec<i16>,
    process: Process,
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

//Used to display 
impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "name: {}, allocated: {:#?}, required_resources: {:#?}, need: {:#?}, safe_state: {}'b\n'",
            self.name,
            self.allocated,
            self.required_resources,
            self.need,
            self.safe_state,
        )
    }
}
