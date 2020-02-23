use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    #[derive(Debug, Clone)]
    pub struct Process {
        process_name: String,
        allocated: Vec<i16>,
        required_resources: Vec<i16>,
        available_resources: Vec<i16>,
        need: Vec<i16>,
        safe_state: bool,
    }

    impl Process {
        fn available_resources(&mut self, new_aval: Vec<i16>) {
            self.available_resources = new_aval;
        }
        fn set_safe_state(&mut self, new_safe_state: bool) {
            self.safe_state = new_safe_state;
        }
    }
    // calc need of each process in list of processes
    // fn calc_need(processes: Vec<Process>) -> Vec<Process> {
    //     println!("hey i'm calcing");
    //     let temp_procs = processes.clone();
    //     for x in temp_procs.iter() {
    //         let mut ugh = x.clone();
    //         for (j, k) in ugh.allocated.iter().zip(ugh.required_resources.iter()) {
    //             let temp = j - k;
    //             let temp_2 = temp.abs();
    //             println!("{:?}", temp_2);
    //             ugh.need.push(temp_2);
    //             println!("{:?}", x);
    //         }
    //     }
    //     temp_procs
    // }

    fn calculate_need(allocated: Vec<i16>, required: Vec<i16>, mut process: Process) -> Process {
        for (x, y) in allocated.iter().zip(required.iter()) {
            let temp = x - y;
            let temp_2 = temp.abs();
            process.need.push(temp_2);
        }
        process
    }

    fn update_aval(aval: Vec<i16>, allocation: Vec<i16>, mut process: Process) -> Process {
        let mut new_aval: Vec<i16> = Vec::new();
        let mut temp = 0;
        for (x, y) in aval.iter().zip(allocation.iter()) {
            new_aval.insert(temp, x + y);
            temp += 1;
            process.available_resources(new_aval.clone());
        }
        process
    }

    fn can_process_run_safely(need: Vec<i16>, aval: Vec<i16>, mut process: Process) -> Process {
        let mut counter = 0;
        println!("COUNTER IS: {:?} ", counter);
        for (x, y) in need.iter().zip(aval.iter()) {
            println!("{:?} {:?}", x, y);
            if x <= y {
                counter += 1;
                if counter == aval.len() - 1 {
                    process.safe_state = true;
                }
            } else if x > y {
                println!("{:?} {:?}", x, y);
                break;
            }
        }

        process
    }
    let input_file1 = BufReader::new(File::open("input1.txt").unwrap());

    let mut file1_two_d: Vec<Vec<i16>> = input_file1
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let input_file2 = BufReader::new(File::open("input2.txt").unwrap());

    let mut file2_two_d: Vec<Vec<i16>> = input_file2
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let available_resources_two: Vec<i16> = file2_two_d.remove(0);

    let input_file3 = BufReader::new(File::open("input3.txt").unwrap());

    let mut file3_two_d: Vec<Vec<i16>> = input_file3
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let available_resources_three: Vec<i16> = file3_two_d.remove(0);

    let resource_clone: Vec<i16> = file1_two_d.remove(0);
    let mut process_one = Process {
        process_name: String::from("process1"),
        available_resources: resource_clone.clone(),
        need: Vec::new(),
        allocated: file1_two_d.remove(0),
        required_resources: file1_two_d.remove(4),
        safe_state: false,
    };

    let mut process_two = Process {
        process_name: String::from("process2"),
        available_resources: resource_clone.clone(),
        need: Vec::new(),
        allocated: file1_two_d.remove(0),
        required_resources: file1_two_d.remove(3),
        safe_state: false,
    };

    let mut process_three = Process {
        process_name: String::from("process3"),
        available_resources: resource_clone.clone(),
        need: Vec::new(),
        allocated: file1_two_d.remove(0),
        required_resources: file1_two_d.remove(2),
        safe_state: false,
    };

    let mut process_four = Process {
        process_name: String::from("process4"),
        available_resources: resource_clone.clone(),
        need: Vec::new(),
        allocated: file1_two_d.remove(0),
        required_resources: file1_two_d.remove(1),
        safe_state: false,
    };

    let mut process_five = Process {
        process_name: String::from("process5"),
        available_resources: resource_clone.clone(),
        need: Vec::new(),
        allocated: file1_two_d.remove(0),
        required_resources: file1_two_d.remove(0),
        safe_state: false,
    };

    let process_one = calculate_need(
        process_one.clone().allocated,
        process_one.clone().required_resources,
        process_one.clone(),
    );

    let process_one = update_aval(
        process_one.clone().available_resources,
        process_one.clone().allocated,
        process_one.clone(),
    );
    let process_two = calculate_need(
        process_two.clone().allocated,
        process_two.clone().required_resources,
        process_two.clone(),
    );

    let process_two = can_process_run_safely(
        process_two.clone().need,
        process_two.clone().available_resources,
        process_two.clone(),
    );

    let process_one = can_process_run_safely(
        process_one.clone().need,
        process_one.clone().available_resources,
        process_one.clone(),
    );

    // let mut container: Vec<Process> = Vec::new();
    // container.push(process_one.clone());
    // container.push(process_two.clone());
    // container = calc_need(container);
    // println!("{:?}", container);
}
