use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn main() -> io::Result<()> {
    let mut negativeArray: [u8; 200000] = [0; 200000];
    let mut positiveArray: [u8; 200000] = [0; 200000];

    let mut freq: i32 = 0;

    

    while true {
        
        let f = File::open("input.txt")?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            if(freq < 0){
                let negFreq = freq * -1;
                negativeArray[negFreq as usize] += 1;
                if negativeArray[negFreq as usize] > 1 {
                    println!("first repeated frequency is {}", freq );
                    return Ok(());
                }
            } else {
                positiveArray[freq as usize] += 1;
                if positiveArray[freq as usize] > 1 {
                    println!("first repeated frequency is {}", freq );
                    return Ok(());

                }
            }
            
            // println!("{}", line?);

            let num : i32;
            
            match line {
                Ok(n) => num = n.trim().parse().unwrap(),
                Err(e) => {
                    println!("Something went wrong with the line" );
                    println!("Error: {}", e );
                    return Ok(());
                }
            }

            
            freq += num;
        }

    }

    println!(" Output number: {}", freq);


    // println!("You typed: {}", input.trim());

    Ok(())

    // println!("Hello, world!");
}
