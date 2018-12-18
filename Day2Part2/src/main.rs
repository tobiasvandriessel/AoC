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
        Ok(string) => {
            assert_eq!(string, String::from("fgij"));
            // println!("Checksum: {}", num );
            Ok(())
        },
        Err(e) => {
            // println!("Error: {}", e);
            Err(e)
        }

    }
}

fn run_program(input_path: &str) -> io::Result<String> {
    // let mut freq: i32 = 0;
    let mut fileLines = 0;

    {
        let f = File::open(input_path)?;
        let reader = BufReader::new(f);

        for _ in reader.lines() {        
            fileLines += 1;
        }
    }

    let f = File::open(input_path)?;
    let reader = BufReader::new(f);

    let mut stringsVec = vec![String::new(); fileLines];
    // let stringsArray : [String; fileLines] = [""; fileLines];

    for (i, line) in reader.lines().enumerate() {        
        match line {
            Ok(n) => {  
                stringsVec[i] = n;
            },
            Err(e) => {
                println!("Something went wrong with the line" );
                println!("Error: {}", e );
                return Err(e);
            }
        }
    }

    for (i, string) in stringsVec.iter().enumerate() {    
        for j in (i + 1)..stringsVec.len() {
            let mut matching_chars = 0;

            {
                let mut chars_1 = string.chars();
                let mut chars_2 = stringsVec[j].chars();

                let mut char_1 = chars_1.next();
                let mut char_2 = chars_2.next();

                while char_1 != None {
                    if char_1.unwrap() == char_2.unwrap() {
                        matching_chars += 1;
                    }

                    char_1 = chars_1.next();
                    char_2 = chars_2.next();
                }
            }

            if matching_chars == string.len() - 1 {

                let mut chars_1 = string.chars();
                let mut chars_2 = stringsVec[j].chars();

                let mut char_1 = chars_1.next();
                let mut char_2 = chars_2.next();

                let mut result = String::new();

                while char_1 != None {
                    if (char_1.unwrap() == char_2.unwrap()) {
                        result.push(char_1.unwrap());
                    } 

                    char_1 = chars_1.next();
                    char_2 = chars_2.next();
                }

                return Ok(result);
                
            }


            
        }
        // match line {
        //     Ok(n) => {  
        //         stringsVec[i] = n;
        //     },
        //     Err(e) => {
        //         println!("Something went wrong with the line" );
        //         println!("Error: {}", e );
        //         return Err(e);
        //     }
        // }
    }

    Ok(String::new())

    // println!(" Output number: {}", freq);


    // println!("You typed: {}", input.trim());

    // Ok(())
}
