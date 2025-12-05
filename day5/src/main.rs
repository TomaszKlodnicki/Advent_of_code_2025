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

struct Database {
    ranges: Vec<[u64; 2]>,
    ingredients: Vec<u64>,
}

fn load_file() -> Database {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let mut split_datas = contents.split("\n\n").filter(|s| !s.is_empty());
    let rang = split_datas
        .next()
        .expect("incorect input")
        .lines()
        .map(|l| {
            let values: Vec<u64> = l
                .split("-")
                .map(|value| value.parse::<u64>().unwrap())
                .collect();

            [values[0], values[1]]
        })
        .collect();

    let ing = split_datas
        .next()
        .expect("incorect input")
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    Database {
        ranges: rang,
        ingredients: ing,
    }
}

fn part_1(data: &Database) -> u64 {
    let mut output: u64 = 0;

    for ing in &data.ingredients {
        for r in &data.ranges {
            if (r[0]..(r[1] + 1)).contains(ing) {
                output += 1;
                break;
            }
        }
    }

    output
}

fn part_2(data: &mut Database) -> u64 {
    let mut output: u64;

    data.ranges.sort_by(|a, b| a[0].cmp(&b[0]));
    let range = &data.ranges;
    output = (range[0][1] + 1) - range[0][0];

    for r_i in 1..range.len() {
        let mut start: u64 = range[r_i][0];
        let mut end: u64 = range[r_i][1] + 1;
        for befor in &range[0..r_i] {
            if (befor[0]..(befor[1] + 1)).contains(&start) {
                start = befor[1] + 1;
            }
            if (befor[0]..(befor[1] + 1)).contains(&end) {
                end = befor[0];
            }
        }
        if start < end {
            output += end - start;
        }
    }

    output
}

fn main() {
    let mut data = load_file();
    let part_1_solution = part_1(&data);
    println!("part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&mut data);
    println!("part 2: {:?}", part_2_solution);
}
