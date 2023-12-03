use std::fs::read_to_string;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part_1() {
    println!("Part 1");
    
    let cubes_limit = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut sum = 0;
    for line in read_lines("input.txt") {

        let mut possible = true;

        let parts = line.split(":").collect::<Vec<&str>>();

        let game_info = parts[0];
        let id = game_info.split(" ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<i32>().unwrap_or(0);

        let game_data = parts[1].split(";");

        for data in game_data {
            let cubes = data.trim()
                            .split(", ")
                            .collect::<Vec<&str>>();

            for cube in cubes {
                let cube_parts = cube.split(" ")
                                    .collect::<Vec<&str>>();

                let qtd = cube_parts[0].parse::<i32>().unwrap_or(0);
                let color = cube_parts[1];
                if qtd > cubes_limit[color] {
                    possible = false;
                }
            }
        }

        if possible {
            sum += id;
        }
    }

    println!("Sum IDs = {sum}");
}

fn part_2() {
    println!("Part 2");
    

    let mut sum = 0;
    for line in read_lines("input_2.txt") {

        let mut cubes_min = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let parts = line.split(":").collect::<Vec<&str>>();

        let game_data = parts[1].split(";");

        for data in game_data {
            let cubes = data.trim()
                            .split(", ")
                            .collect::<Vec<&str>>();

            for cube in cubes {
                let cube_parts = cube.split(" ")
                                    .collect::<Vec<&str>>();

                let qtd = cube_parts[0].parse::<i32>().unwrap_or(0);
                let color = cube_parts[1];
                if qtd > cubes_min[color] {
                    if let Some(c) = cubes_min.get_mut(color) {
                        *c = qtd;
                    }
                }
            }
        }

        let mut mult = 1;
        for (_, qtd) in cubes_min {
            mult *= qtd
        }
        sum += mult;
    }

    println!("Sum = {sum}");

}

fn main() {
    part_1();
    part_2();
}
