use aoc2023::{read_input, InputType};
use itertools::Itertools;
use mathru::{
    algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    },
    matrix, vector,
};
use timed::timed;

const DAY: u8 = 24;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input, 200000000000000, 400000000000000));
    println!("Part2: {}", part2(&real_input));
}

#[timed]
fn part1(input: &str, min_coord: isize, max_coord: isize) -> usize {
    let linear_coefficients = input
        .trim()
        .lines()
        .map(|line| {
            let (px, py, pz, vx, vy, vz) = line
                .replace("@", ",")
                .split(",")
                .flat_map(|segment| segment.trim().parse::<f64>())
                .collect_tuple()
                .unwrap();

            ((px, py), (vx, vy))
        })
        .collect_vec();

    let count = linear_coefficients
        .iter()
        .combinations(2)
        .filter(|lines| {
            let lines: Vec<_> = lines.iter().collect();
            let ((px1, py1), (vx1, vy1)) = lines[0];
            let ((px2, py2), (vx2, vy2)) = lines[1];

            let a: General<f64> = matrix![
                *vx1, -(*vx2);
                *vy1, -(*vy2)
            ];

            let b: Vector<f64> = vector![
                px2 - px1;
                py2 - py1
            ];

            // Solve it directly
            let coefficients: Vector<f64> = match a.solve(&b) {
                Ok(v) => v,
                Err(_) => return false,
            };

            let t = coefficients[0];
            let s = coefficients[1];

            let crossed_in_past = t < 0.0 || s < 0.0;

            if crossed_in_past {
                return false;
            }

            let intersection1 = (*px1 as f64 + t * *vx1 as f64, *py1 as f64 + t * *vy1 as f64);
            // let intersection2 = (*px2 as f64 + t * *vx2 as f64, *py2 as f64 + t * *vy2 as f64);
            // dbg!((t, s, intersection1));
            // if intersection1.0 != intersection2.0 || intersection1.1 != intersection2.1 {
            //     return false;
            // }

            let crossed_in_area = intersection1.0 >= min_coord as f64
                && intersection1.0 <= max_coord as f64
                && intersection1.1 >= min_coord as f64
                && intersection1.1 <= max_coord as f64;

            crossed_in_area
        })
        .count();

    count
}

#[timed]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 2;
        let result = part1(&get_test_input(), 7, 27);
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = "";
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}

// y1 = a1 * x1 + b1
// y2 = a2 * x2 + b2

// a1 = (y1 - b1) / x1
// b1 = y1 - a1*x1

// a1 * x1 + b1 = y1
// a2 * x2 + b2 = y2

// l1 : (y1, x1) = (px1, py1) + t*(vx1, vy1)
// l2 : (y2, x2) = (px2, py2) + t*(vx2, vy2)

// (px1, py1) + t*(vx1, vy1) = (px2, py2) + t*(vx2, vy2)
// t*(vx1, vy1) - t*(vx2, vy2) = (px2, py2) - (px1, py1)
// t*(vx1 - vx2, vy1 - vy2) = (px2 - px1, py2 - py1)
// t = ((px2 - px1) / (vx1 - vx2), (py2 - py1) / (vy1 - vy2))
