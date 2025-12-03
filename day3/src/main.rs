use std::fs;

static INPUT_FILENAME: &str = "input.txt";
static EXAMPLE_FILENAME: &str = "example.txt";

fn get_filename() -> &'static str {
    if cfg!(debug_assertions) {
        EXAMPLE_FILENAME
    } else {
        INPUT_FILENAME
    }
}

fn load_file() -> Vec<Vec<u8>> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data = contents
        .lines()
        .map(|s| s.chars().map(|s| s.to_digit(10).unwrap() as u8).collect())
        .collect();

    data
}

fn part_1(data: &Vec<Vec<u8>>) -> u64 {
    let mut output: u64 = 0;

    for line in data {
        let (first_index, first_value) = line[..line.len() - 1]
            .iter()
            .enumerate()
            .max_by_key(|&(index, &value)| value as usize * line.len() + (line.len() - index))
            .unwrap();

        let secnd_value = line[(first_index + 1)..line.len()].iter().max().unwrap();
        output += *first_value as u64 * 10 + *secnd_value as u64;
    }

    output
}

fn part_2(data: &Vec<Vec<u8>>) -> u64 {
    let mut output: u64 = 0;

    for line in data {
        assert!(line.len() >= 12);
        let mut last_index: i32 = -1;
        for i in 0..12 {
            let max_inx: usize = 11 - i;
            let start_index: usize = (last_index + 1) as usize;

            let (index, &value) = line[start_index..(line.len() - max_inx)]
                .iter()
                .enumerate()
                .max_by_key(|&(index_i, &value_v)| {
                    value_v as usize * line.len() + (line.len() - index_i)
                })
                .unwrap();

            last_index = (index + start_index) as i32;
            output += value as u64 * 10u64.pow(max_inx as u32);
        }
    }

    output
}

fn main() {
    let data = load_file();
    let part_1_solution = part_1(&data);
    println!("part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&data);
    println!("part 2: {:?}", part_2_solution);
}
