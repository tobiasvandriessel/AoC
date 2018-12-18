use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> io::Result<()> {
    
    match run_program("input.txt") {
        Ok(num) => {
            println!("Checksum: {}", num );
            Ok(())
        },
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }

    }

    // println!("Hello, world!");
}

#[test]
fn working_code() -> io::Result<()> {
    match run_program("inputTest.txt") {
        Ok(num) => {
            assert_eq!(num, 12);
            // println!("Checksum: {}", num );
            Ok(())
        },
        Err(e) => {
            // println!("Error: {}", e);
            Err(e)
        }

    }
}

fn run_program(input_path: &str) -> io::Result<i32> {
    // let mut freq: i32 = 0;

    let mut amount_twos = 0;
    let mut amount_threes = 0;


    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {        
        let string;
        let mut letter_array: [u8; 26] = [0; 26];
        
        match line {
            Ok(n) => {  
                string = n.trim();
                let mut arr = [0; 2];
                for charac in string.chars() {
                    charac.encode_utf8(&mut arr);
                    assert!(arr[0] - 97 >= 0);
                    letter_array[(arr[0] - 97) as usize] += 1;
                    // println!("escape code {}", arr[0] - 97);
                    // letterArray[charac.encode_utf8()] += 1;
                }

                let mut exactly_two = false;
                let mut exactly_three = false;

                for i in 0..26 {
                    if letter_array[i] == 2 {
                        exactly_two = true;
                    } else if letter_array[i] == 3 {
                        exactly_three = true;
                    }
                }

                if exactly_two {
                    amount_twos += 1;
                }

                if exactly_three {
                    amount_threes += 1;
                }

                println!("New values of amountTwos: {} and amountThrees: {}", amount_twos, amount_threes );
            },
            Err(e) => {
                println!("Something went wrong with the line" );
                println!("Error: {}", e );
                return Err(e);
            }
        }

        
        
        // freq += num;
    }

    let checksum = amount_twos * amount_threes;

    println!("Checksum: {}",checksum );

    Ok(checksum)
    

    // println!(" Output number: {}", freq);


    // println!("You typed: {}", input.trim());

    // Ok(())
}
