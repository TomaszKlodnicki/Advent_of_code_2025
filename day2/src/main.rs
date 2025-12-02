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

struct IdRange {
    start: u64,
    end: u64,
}

fn load_file() -> Vec<IdRange> {
    let mut contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    contents.retain(|c| c != '\n');

    let data = contents
        .split(",")
        .filter(|c| !c.is_empty())
        .map(|c| {
            let values: Vec<u64> = c.split("-").map(|v| v.parse::<u64>().unwrap()).collect();
            IdRange {
                start: values[0],
                end: values[1],
            }
        })
        .collect();

    data
}

fn part_1(data: &Vec<IdRange>) -> u64 {
    let mut output: u64 = 0;

    for range in data {
        for i in range.start..(range.end + 1) {
            let i_str = i.to_string();

            if i_str.len() % 2 == 0 {
                let (first, secnd) = i_str.split_at(i_str.len() / 2);
                if first == secnd {
                    output += i;
                }
            }
        }
    }

    output
}

fn part_2(data: &Vec<IdRange>) -> u64 {
    let mut output: u64 = 0;

    for range in data {
        for i in range.start..(range.end + 1) {
            let i_str = i.to_string();

            for j in 2..(i_str.len() + 1) {
                let strings_len = i_str.len() / j;

                let first_chunk = i_str.as_bytes().chunks(strings_len).nth(0).unwrap();

                if i_str
                    .as_bytes()
                    .chunks(strings_len)
                    .all(|c| c == first_chunk)
                {
                    output += i;
                    break;
                }
            }
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
