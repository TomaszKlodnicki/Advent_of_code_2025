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

fn load_file() -> Vec<Vec<char>> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data = contents.lines().map(|s| s.chars().collect()).collect();

    data
}

fn check_if_can_be_picked_up(data_set: &Vec<Vec<char>>, row: usize, collumn: usize) -> bool {
    match data_set[row][collumn] {
        '.' => false,
        '@' => {
            let mut object_near = 0;
            let data_set_cols = data_set[0].len() as i32;
            let data_set_rows = data_set.len() as i32;

            for i in (-1)..2 {
                for j in (-1)..2 {
                    let r: i32 = row as i32 + i;
                    let c: i32 = collumn as i32 + j;
                    if r < data_set_rows
                        && r >= 0
                        && c < data_set_cols
                        && c >= 0
                        && !(c == collumn as i32 && r == row as i32)
                    {
                        if data_set[r as usize][c as usize] == '@' {
                            object_near += 1;
                        }
                    }
                }
            }
            object_near < 4
        }
        _ => panic!("[row][collumn] element should by available in data_set"),
    }
}

fn part_1(data: &Vec<Vec<char>>) -> u64 {
    let mut output: u64 = 0;

    for row in 0..data[0].len() {
        for collumn in 0..data[0].len() {
            if check_if_can_be_picked_up(&data, row, collumn) {
                output += 1;
            }
        }
    }

    output
}

fn part_2(data: &mut Vec<Vec<char>>) -> u64 {
    let mut output: u64 = 0;

    let mut change: bool = true;
    while change {
        change = false;
        for row in 0..data[0].len() {
            for collumn in 0..data[0].len() {
                if check_if_can_be_picked_up(&data, row, collumn) {
                    data[row][collumn] = '.';
                    output += 1;
                    change = true;
                }
            }
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
