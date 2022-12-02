use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have file contents");

    let split_results = contents
        .lines()
        .map(|x| x.split_ascii_whitespace())
        .map(|split_result| 
            match split_result.collect::<Vec<&str>>()[..] {
                ["A", "X"] => 1 + 3,
                ["A", "Y"] => 2 + 6,
                ["A", "Z"] => 3 + 0,
                ["B", "X"] => 1 + 0,
                ["B", "Y"] => 2 + 3,
                ["B", "Z"] => 3 + 6,
                ["C", "X"] => 1 + 6,
                ["C", "Y"] => 2 + 0,
                ["C", "Z"] => 3 + 3,
                _ => panic!("Didn't expect this case")
            }
        );
    
    println!("Total result: {0}", split_results.sum::<i32>());
}
