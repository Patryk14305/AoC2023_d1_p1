use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let mut sum:u32 = 0;
    let input = read_lines("input.txt");

    for text_line in input{
        let mut digits: Vec<u32> = vec![];
        let chars = text_line.as_str().chars();

        for char in chars {
            if char.is_ascii_digit(){
                digits.push(char.to_digit(10).unwrap());

//  Verify digit check:                
           //     println!("{} is digit", char)
            }
            // else {
            //     println!("{} is not a digit", char)
            // }
        }
        sum+=digits.first().unwrap()*10 + digits.last().unwrap();
    }

    println!("{}", sum)
}
