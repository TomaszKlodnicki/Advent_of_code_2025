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

struct Com {
    direction: bool,
    value: i32,
}

fn load_file() -> Vec<Com> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data: Vec<Com> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (direction_str, value_str) = s.split_at(1);
            let dir: bool = match direction_str {
                "L" => false,
                "R" => true,
                _ => panic!("cant conver't!"),
            };
            let val: i32 = i32::from_str_radix(value_str, 10).expect("can't convert value!");

            Com {
                direction: dir,
                value: val,
            }
        })
        .collect();

    data
}

fn part_1(data: &Vec<Com>) -> u32 {
    let mut output: u32 = 0;

    let mut dial_position: i32 = 50;
    for command in data {
        if command.direction {
            dial_position += command.value;
        } else {
            dial_position -= command.value;
        }

        while dial_position < 0 {
            dial_position = 100 + dial_position;
        }

        dial_position = dial_position % 100;

        if dial_position == 0 {
            output += 1;
        }
    }

    output
}

fn part_2(data: &Vec<Com>) -> u32 {
    let mut output: u32 = 0;

    let mut dial_position: i32 = 50;
    for command in data {
        if command.direction {
            dial_position += command.value;

            let dial_mod = dial_position % 100;
            output += ((dial_position - dial_mod) / 100) as u32;
            dial_position = dial_mod;
        } else {
            if dial_position == 0 && command.value != 0 {
                output -= 1;
            }

            dial_position -= command.value;

            if dial_position == 0 {
                output += 1;
            }

            while dial_position < 0 {
                dial_position = 100 + dial_position;
                output += 1;

                if dial_position == 0 {
                    output += 1;
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
