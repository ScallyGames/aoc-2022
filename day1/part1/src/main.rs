use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt")
        .expect("Should have file contents");

    let elves_content = contents.split("\n\n");

    let calory_count_per_elf = elves_content.map(
        |x| x.lines().filter_map::<i32, _>(|w| w.parse().ok()).sum::<i32>());

    for entry in calory_count_per_elf.clone() {
        println!("{0}", entry);
    }

    println!("{0} elves", calory_count_per_elf.clone().count());
    println!("Highest calory count: {0}", calory_count_per_elf.clone().max().expect("Should have a max value"));

    println!("Highest calory count on elf: {0}", calory_count_per_elf
        .clone()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap_or_default().0
    );

}
