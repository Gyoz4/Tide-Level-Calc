use std::io;

fn main() {
    'outer: loop {
        // get all of the values neede by the program
        println!("Enter the Low tide time: ");
        let mut low_time = String::new();
        io::stdin().read_line(&mut low_time).expect("couldn't get input");
        low_time = low_time.trim().parse().expect("Please enter a number");

        println!("Enter the Low tide height: ");
        let mut low_height = String::new();
        io::stdin().read_line(&mut low_height).expect("couldn't get input");
        low_height = low_height.trim().parse().expect("Please enter a number");

        println!("Enter the High tide time: ");
        let mut high_time = String::new();
        io::stdin().read_line(&mut high_time).expect("couldn't get input");
        high_time = high_time.trim().parse().expect("Please enter a number");

        println!("Enter the High tide height: ");
        let mut high_height = String::new();
        io::stdin().read_line(&mut high_height).expect("couldn't get input");
        high_height = high_height.trim().parse().expect("Please enter a number");

        println!("Enter the minimum desired height: ");
        let mut min_height = String::new();
        io::stdin().read_line(&mut min_height).expect("couldn't get input");
        min_height = min_height.trim().parse().expect("Please enter a number");

        println!("\nLow time: {}Low height: {}\nHigh time: {}\nHigh height: {}\nMin height: {}", low_time, low_height, high_time, high_height, min_height);


        println!("\n'q' to end, any other button to continue...");
        let mut exit = String::new();
        io::stdin().read_line(&mut exit).expect("couldn't get input");
        if exit.trim() == "q" || exit.trim() == "Q" {break 'outer;}
    }    
}

