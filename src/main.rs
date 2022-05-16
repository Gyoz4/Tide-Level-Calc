use std::io::{self, stdout, Write};

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
        let top_time_slot = time_slot(&mut height_arr, min_height);
        if top_time_slot == 10.0 {
            println!("Couldn't find a fitting time slot");
            pause();
            break 'outer;
        }

        let mut x:[f32; 3] = [0.0; 3];
        //x[0] = time_step * top_time_slot;

        let mut y:[f32; 3] = [0.0; 3];

        // x[2] = ((y[1] - y[2])*x[0] + (y[2] - y[0])*x[1]) / (y[1] - y[0]);

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

fn time_slot (arr: &mut [f32], min: f32) -> f32{
    if arr[0] > min { return 0.0;} 
    else if arr[1] > min {return 1.0;}
    else if arr[2] > min {return 2.0;}
    else if arr[3] > min {return 3.0;}
    else if arr[4] > min {return 4.0;}
    else if arr[5] > min {return 5.0;}

    return 10.0;
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    io::Read::read(&mut io::stdin(), &mut [0]).unwrap();
}
