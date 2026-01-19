use crate::{cube::{Turn, TurnDir, Twist}, solver::{get_orientation_table, solver}};

pub mod cube;
pub mod solver;

// U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2
const SUPERFLIP: [Twist; 20] = [
    Twist::new(Turn::U, TurnDir::One),
    Twist::new(Turn::R, TurnDir::Two),
    Twist::new(Turn::F, TurnDir::One),
    Twist::new(Turn::B, TurnDir::One),
    Twist::new(Turn::R, TurnDir::One),
    Twist::new(Turn::B, TurnDir::Two),
    Twist::new(Turn::R, TurnDir::One),
    Twist::new(Turn::U, TurnDir::Two),
    Twist::new(Turn::L, TurnDir::One),
    Twist::new(Turn::B, TurnDir::Two),
    Twist::new(Turn::R, TurnDir::One),
    Twist::new(Turn::U, TurnDir::Prime),
    Twist::new(Turn::D, TurnDir::Prime),
    Twist::new(Turn::R, TurnDir::Two),
    Twist::new(Turn::F, TurnDir::One),
    Twist::new(Turn::R, TurnDir::Prime),
    Twist::new(Turn::L, TurnDir::One),
    Twist::new(Turn::B, TurnDir::Two),
    Twist::new(Turn::U, TurnDir::Two),
    Twist::new(Turn::F, TurnDir::Two),
];

fn main() {
    let mut cube = cube::Cube::new_solved();
    println!("{}", cube);

    let mut prev_turn = None;
    for _ in 0..100 {
        let twist = cube::Twist::new_random(prev_turn);
        print!("{} ", twist);
        cube.twist(twist);

        prev_turn = Some(twist.turn);
    }

    // for twist in SUPERFLIP {
    //     cube.twist(twist);
    // }

    println!("");
    println!("{}", cube);

    let sol = solver(&mut cube);
    println!("{}", sol);
    // let table = get_orientation_table();
    // println!("{:?}", table);
}


// Todo
// Optimize getting to g1, one time it took 241, it is very scaramble based.