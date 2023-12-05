use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Could not read file!");
    let mut pt1 = 0;
    let mut pt2 = 0;

    for line in input.lines() {
        let numbers = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
        pt1 += (first as u16)  * 10 + last as u16;  
    }

    for line in input.lines() {
        let line = &line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        let numbers = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];
       
        pt2 += (first as u32)  * 10 + last as u32;  
    }

    println!("part 1:{:?} part 2: {:?}",pt1, pt2);
}