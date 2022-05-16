use std::io;

fn main() {
    'outer: loop {
        let reader = io::stdin();

        ////////////////////////////////////////////////////////////////
        // get all of the values neede by the program
        let mut input = String::new();
        println!("Enter the Low tide time: ");
        let mut low_time:f32 = 0.0;
        reader.read_line(&mut input).ok().expect("couldn't get input");
        low_time = input.trim().parse().expect("Please enter a number");

        let mut input = String::new();
        println!("Enter the Low tide height: ");
        let mut low_height:f32 = 0.0;
        reader.read_line(&mut input).ok().expect("couldn't get input");
        low_height = input.trim().parse().expect("Please enter a number");

        let mut input = String::new();
        println!("Enter the High tide time: ");
        let mut high_time:f32 = 0.0;
        reader.read_line(&mut input).expect("couldn't get input");
        high_time = input.trim().parse().expect("Please enter a number");

        let mut input = String::new();
        println!("Enter the High tide height: ");
        let mut high_height:f32 = 0.0;
        reader.read_line(&mut input).expect("couldn't get input");
        high_height = input.trim().parse().expect("Please enter a number");

        let mut input = String::new();
        println!("Enter the minimum desired height: ");
        let mut min_height:f32 = 0.0;
        reader.read_line(&mut input).expect("couldn't get input");
        min_height = input.trim().parse().expect("Please enter a number");

        println!("\nLow time: {}\nLow height: {}\nHigh time: {}\nHigh height: {}\nMin height: {}", low_time, low_height, high_time, high_height, min_height);

        ////////////////////////////////////////////////////////////////
        // getting intermediate working values        
        let time_step: f32 = (high_time - low_time) / 6.0;
        let height_step: f32 = (high_height - low_height) / 12.0;

        let mut height_arr: [f32; 6] = [0.0; 6];
        height_arr[0] = low_height + time_step;
        height_arr[1] = height_arr[0] + 2.0 * time_step;
        height_arr[2] = height_arr[1] + 3.0 * time_step;
        height_arr[3] = height_arr[2] + 3.0 * time_step;
        height_arr[4] = height_arr[3] + 2.0 * time_step;
        height_arr[5] = height_arr[4] + 1.0 * time_step;

        //TODO: calculate float time

        /////////////////////////////////////////////////////////////////
        // output the time

        //TODO: convert float time into hh:mm

        ////////////////////////////////////////////////////////////////
        // exit program
        println!("\n'q' to end, any other button to continue...");
        let mut exit = String::new();
        reader.read_line(&mut exit).expect("couldn't get input");
        if exit.trim() == "q" || exit.trim() == "Q" {break 'outer;}
    }    
}

