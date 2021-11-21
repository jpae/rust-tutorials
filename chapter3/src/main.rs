use std::io;

fn read_input() -> String {
    let mut selection = String::new();
    
    io::stdin()
    .read_line(&mut selection)
    .expect("Failed to read line");

    return selection
}

fn f_to_c() {
    println!("Please enter a temperature:");

    let temp_f: f64 = read_input().trim().parse().expect("Please type a number!");

    let temp_c = (temp_f - 32.0) * 5.0 / 9.0;

    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);
}

fn fibonacci(num: u32) -> u64 {
    let mut f: (u64, u64) = (0, 1);

    if num == 0 { return f.0 }

    for step in 0..num {
        if step > 1 {
            f = (f.1, f.0 + f.1);
        }
    }
    f.1 + f.0
}

fn fib() {
    println!("Please enter a number for Fibonacci:");

    let n: u32 = read_input().trim().parse().expect("Please type a positive number!");

    println!("The {} index Fibonacci is {}", n, fibonacci(n));
}

fn christmas_lyrics() {
    let days = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eigth", "Ninth", "Tenth", "Eleventh", "Twelveth"];
    let gifts = ["A song and a Christmas tree", "Two candy canes", "Three boughs of holly", "Four colored lights", "A shining star",
                 "Little silver bells", "Candles a-glowing", "Gold and silver tinsel", "A guardian angel", "Some mistletoe",
                 "Gifts for one and all", "All their good wishes"];
    let mut previous_lyrics = Vec::new();
    
    for (ndx, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas", day);
        println!("My good friends brought to me");
        previous_lyrics.push(gifts[ndx]);

        for lyric in previous_lyrics.iter().rev() {
            println!("{}", lyric);
        }

        println!();
    } 
}

fn main() {

    loop {
        println!("\n1. Convert temperatures between Fahrenheit and Celsius");
        println!("2. Generate the nth Fibonacci number");
        println!("3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”");
        println!("4. Quit");
        println!("Please select an exercise:");
    
        match read_input().trim().parse() {
            Ok(num) => {
                println!();
                match num {
                    1 => f_to_c(),
                    2 => fib(),
                    3 => christmas_lyrics(),
                    4 => break,
                    _ => println!("ERROR: Unknown selection"),
                }
            }
            Err(_) => continue,
        };
    }
}
