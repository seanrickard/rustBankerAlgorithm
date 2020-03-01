use std::fs::File;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::io;
use std::io::Write; //to enable flush

<<<<<<< Updated upstream
fn f1() {
    println!("In f1");
}
 
fn f2() {
    println!("In f2");
}
 
fn f_unknown() {
    println!("In unknown function");
}

fn main() {    

    print!("Choose the file to process: "); 
    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read choice");

    println!("You chose: {}", choice);

    //**********************************************/        
        //let user_choice = 2;
     
        //Execute function based on user_choice's value
        match choice {
            // "1" => f1(),
            // "2" => f2(),
            _ => f_unknown(),
        }
    //**********************************************/

        //text file 1
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
=======
fn main() {

    //loop {

        print!("Choose the file to process: "); 
        io::stdout().flush().unwrap();
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read choice");

        let choice: u32 = choice.trim().parse()
            .expect("Please type a number!");

            // let trimmed = choice.trim();
            // match trimmed.parse::<u32>() {
            //     Ok(i) => println!("Your integer input: {}", i),
            //     Err(..) => println!("{} is not an integer.", trimmed),
            // };

        println!("You chose: {}", choice);
        //break;

        FileChoice(&choice); //function call that will choose which file to run

    //     }

    //     // // // let choice = 0u8;
    //     // match choice n {
    //     //     Number { value: 1, .. } => println!("One"),
    //     //     // 2 => println!("two"),
    //     //     // 3 => println!("three"),        
    //     //     _ => (),
    //     // }  
    // }           
    
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
>>>>>>> Stashed changes

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

<<<<<<< Updated upstream
#[derive(Debug, Clone)]
=======


fn FileChoice(choice: &u32) {
    match choice {
        1 => println!("ONE"),
        2 => println!("TWO"),
        _ => println!("Invalid"),
    }
}



#[derive(Debug, Clone, PartialEq)]
>>>>>>> Stashed changes
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





<<<<<<< Updated upstream
=======
    state
}
>>>>>>> Stashed changes
