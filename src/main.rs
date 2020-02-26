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

    let mut procs: Vec<Process> = Vec::with_capacity(5);
    let mut finished: Vec<Process> = Vec::with_capacity(5);
    let mut unable_to_run: Vec<Process> = Vec::new();

    procs.push(process_one.clone());
    procs.push(process_two.clone());
    procs.push(process_three.clone());
    procs.push(process_four.clone());
    procs.push(process_five.clone());
    let mut counter = 0;

    procs = calc_need(procs.clone());

    while finished.len() < procs.clone().len() {

        
        //procs = get_unsafe_procs();
        for x in procs.clone().iter_mut() {
            println!("Checking process  {:?} {}", counter, "if safe to run.");
            x.safe_state = check_safe_state(x.clone().need, available.clone());
           
            println!("{:?}", x.safe_state);
            if x.safe_state == true {
                available = update_aval(x.clone().allocated, available.clone());
                
                let ( procs, finished) =
                    remove_safe_process_add_to_finished(procs.clone(), finished.clone());
                println!("Finished: {:?}", finished);
                println!("Processes: {:?}", procs);
                break;
            }
            counter += 1;
            //println!("{}", counter);
            break;
        }
    }
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
            let temp = k - j;
            x.need.push(temp);
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

pub fn update_aval(mut allocated: Vec<i16>, mut available: Vec<i16>) -> Vec<i16> {
    let mut new_aval: Vec<i16> = Vec::new();
    for (x, y) in allocated.iter_mut().zip(available.iter_mut()) {
        let temp = *x + *y;
        new_aval.push(temp);
    }

    new_aval
}

pub fn remove_safe_process_add_to_finished(
    mut processes: Vec<Process>,
    mut finished: Vec<Process>,
) -> (Vec<Process>, Vec<Process>) {
    for (x, y) in processes.clone().iter_mut().enumerate() {
        if y.safe_state == true {
            finished.push(processes.remove(x));
        }
    }
    (processes, finished)
}

pub fn check_safe_state(mut need: Vec<i16>, mut available: Vec<i16>) -> bool {
    let mut counter = 0;
    let mut state: bool = false;

    for (x, y) in need.clone().iter_mut().zip(available.iter_mut()) {
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

// pub fn update_aval(
//     mut processes: Vec<Process>,
//     mut resource: Vec<i16>,
// ) -> (Vec<Process>, Vec<i16>) {
//     let mut new_aval: Vec<i16> = Vec::new();
//     let mut temp = 0;

//     for x in processes.iter_mut() {
//         for (i, j) in x.allocated.iter().zip(resource.clone().iter()) {
//             if temp < resource.clone().len() {
//                 new_aval.insert(temp, i + j);
//                 println!("After insert{:?} {:?}", new_aval, x);
//                 temp += 1;
//             }
//             println!("After if statement");
//         }
//         resource = new_aval.clone();
//     }
//     (processes, resource)
// }

// pub fn can_process_run_safely(process: Process, available: Vec<i16>) -> Process {
//     let mut proc = process.clone();
//     println!("i hate this {:?}", proc);
//     for x in proc.need.iter {

//         for (j, k) in process.need.iter().zip(available.iter()) {
//         println!("jJJJJJJJJJJJJJJJ {:?}", j);

//         let temp = j - k;
//         let temp_2 = temp.abs();
//        // x.need.push(temp_2);
//     }
// }
//     proc
// }
