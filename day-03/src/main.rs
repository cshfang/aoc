use std::collections::HashMap;

mod reader;

fn main() {
    // let data = "data/sample";
    let data = "data/input";

    part_1(data);
    part_2(data);
}

fn part_1(dataset: &str) {
    let matrix = &map_matrix(reader::read_by_line(dataset).unwrap());
    let numbers = extract_numbers(matrix);
    let part_numbers = get_part_numbers(numbers, matrix);
    let mut total = 0;
    part_numbers.iter().for_each(|number| {
        let mut result = "".to_owned();
        number
            .iter()
            .for_each(|digit| result = format!("{result}{}", digit.content));
        total += result.parse::<i32>().unwrap();
    });
    println!("Part 1: {total}");
}

fn part_2(dataset: &str) {
    let matrix = &map_matrix(reader::read_by_line(dataset).unwrap());
    let numbers = extract_numbers(matrix);
    let gears = find_gears(&numbers, matrix);
    let mut total = 0;
    for gear in gears.iter() {
        if gear.1.len() == 2 {
            let mut gear_ratio = 1;
            for number in gear.1.iter() {
                let mut result = "".to_owned();
                for digit in number.iter() {
                    result = format!("{result}{}", digit.content);
                }
                gear_ratio = gear_ratio * result.parse::<i32>().unwrap();
            }
            total += gear_ratio;
        };
    }

    println!("Part 2: {total}");
}

fn find_gears<'a>(
    numbers: &'a Vec<Vec<Digit>>,
    matrix: &'a Vec<Vec<Digit>>,
) -> HashMap<(usize, usize), Vec<&'a Vec<Digit>>> {
    let mut gears: HashMap<(usize, usize), Vec<&Vec<Digit>>> = HashMap::new();
    numbers.iter().for_each(|number| {
        for digit in number.iter() {
            let gear = check_if_digit_part_number(digit, matrix).1;
            if gear.is_some() {
                if gears.contains_key(&gear.unwrap()) {
                    gears.get_mut(&gear.unwrap()).unwrap().push(number);
                } else {
                    gears.insert(gear.unwrap(), vec![number]);
                }
                break;
            }
        }
    });
    gears
}

fn get_part_numbers(numbers: Vec<Vec<Digit>>, matrix: &Vec<Vec<Digit>>) -> Vec<Vec<Digit>> {
    let mut part_numbers: Vec<Vec<Digit>> = Vec::new();
    numbers.iter().for_each(|number| {
        if check_if_part_number(number, matrix) {
            part_numbers.push(number.clone())
        }
    });
    part_numbers
}

fn check_if_part_number(number: &Vec<Digit>, matrix: &Vec<Vec<Digit>>) -> bool {
    number
        .iter()
        .any(|digit| check_if_digit_part_number(digit, matrix).0)
}

fn is_gear(character: char) -> bool {
    character == '*'
}

fn is_symbol(character: char) -> bool {
    !character.is_numeric() && character != '.'
}

fn check_if_digit_part_number(
    digit: &Digit,
    matrix: &Vec<Vec<Digit>>,
) -> (bool, Option<(usize, usize)>) {
    let max_y = matrix.len() as i32;
    let max_x = matrix[0].len() as i32;
    let up = (digit.y - 1) as usize;
    let down = (digit.y + 1) as usize;
    let left = (digit.x - 1) as usize;
    let right = (digit.x + 1) as usize;
    if digit.y > 0 {
        // check top left
        if digit.x > 0 {
            if is_symbol(matrix[up][left].content) {
                return (
                    true,
                    if is_gear(matrix[up][left].content) {
                        Some((up, left))
                    } else {
                        None
                    },
                );
            }
        }
        // check top
        if is_symbol(matrix[up][digit.x as usize].content) {
            return (
                true,
                if is_gear(matrix[up][digit.x as usize].content) {
                    Some((up, digit.x as usize))
                } else {
                    None
                },
            );
        }
        // check top right
        if digit.x < max_x - 1 {
            if is_symbol(matrix[up][right].content) {
                return (
                    true,
                    if is_gear(matrix[up][right].content) {
                        Some((up, right))
                    } else {
                        None
                    },
                );
            }
        }
    }
    // check left
    if digit.x > 0 {
        if is_symbol(matrix[digit.y as usize][left].content) {
            return (
                true,
                if is_gear(matrix[digit.y as usize][left].content) {
                    Some((digit.y as usize, left))
                } else {
                    None
                },
            );
        }
    }
    // check right
    if digit.x < max_x - 1 {
        if is_symbol(matrix[digit.y as usize][right].content) {
            return (
                true,
                if is_gear(matrix[digit.y as usize][right].content) {
                    Some((digit.y as usize, right))
                } else {
                    None
                },
            );
        }
    }
    if digit.y < max_y - 1 {
        // check bottom left
        if digit.x > 0 {
            if is_symbol(matrix[down][left].content) {
                return (
                    true,
                    if is_gear(matrix[down][left].content) {
                        Some((down, left))
                    } else {
                        None
                    },
                );
            }
        }
        // check bottom
        if is_symbol(matrix[down][digit.x as usize].content) {
            return (
                true,
                if is_gear(matrix[down][digit.x as usize].content) {
                    Some((down, digit.x as usize))
                } else {
                    None
                },
            );
        }
        // check bottom right
        if digit.x < max_x - 1 {
            if is_symbol(matrix[down][right].content) {
                return (
                    true,
                    if is_gear(matrix[down][right].content) {
                        Some((down, right))
                    } else {
                        None
                    },
                );
            }
        }
    }
    return (false, None);
}

fn extract_numbers(matrix: &Vec<Vec<Digit>>) -> Vec<Vec<Digit>> {
    let mut numbers: Vec<Vec<Digit>> = Vec::new();
    matrix.iter().for_each(|row| {
        let mut current: Vec<Digit> = Vec::new();
        // let is_part_number = false;
        row.iter().for_each(|digit| {
            if digit.content.is_numeric() {
                current.push(digit.clone());
            } else if !current.is_empty() {
                numbers.push(current.clone());
                current = Vec::new();
            }
        });
        if !current.is_empty() {
            numbers.push(current);
        }
    });
    numbers
}

fn map_matrix(lines: Vec<String>) -> Vec<Vec<Digit>> {
    let mut row = 0;
    return lines
        .iter()
        .map(|line| {
            let mut vector: Vec<Digit> = Vec::new();
            let mut col = 0;
            for char in line.chars() {
                vector.push(Digit {
                    content: char,
                    x: col,
                    y: row,
                });
                col += 1;
            }
            row += 1;
            vector
        })
        .collect::<Vec<Vec<Digit>>>();
}

#[derive(Debug, Clone)]
struct Digit {
    content: char,
    x: i32,
    y: i32,
}
