use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let groups = contents.split("\n\n");

    // part 1
    // let mut highest: i32 = 0;
    // for group in groups {
    //     let lines = group.split("\n");

    //     let group_total: i32 = lines.map(|line| i32::from_str(line).unwrap()).sum();

    //     if group_total > highest {
    //         highest = group_total
    //     }
    // }

    // println!("Highest value: {}", highest);

    // part 2

    fn group_sum(group: &str) -> i32 {
        let lines = group.split("\n");

        let group_total: i32 = lines.map(|line| i32::from_str(line).unwrap()).sum();

        return group_total;
    }

    let mut group_totals: Vec<i32> = groups.map(group_sum).collect();
    group_totals.sort_unstable();
    group_totals.reverse();

    println!(
        "Highest three: {}, {}, {}",
        group_totals[0], group_totals[1], group_totals[2]
    );

    println!(
        "Highest three sum: {}",
        group_totals[0] + group_totals[1] + group_totals[2]
    );
}
