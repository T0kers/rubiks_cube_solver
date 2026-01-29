use rubiks_cube_solver::{cube::{Cube, algs::ConstAlgorithm}, solver::solver};


fn main() {
    let mut cube = Cube::new_solved();
    println!("{}", cube);

    // let mut prev_turn = None;
    // let mut rng = rand::rng();
    // for _ in 0..100 {
    //     let twist = Twist::new_random(&mut rng, prev_turn);
    //     print!("{} ", twist);
    //     cube.twist(twist);

    //     prev_turn = Some(twist.turn);
    // }

    cube.apply_const_algorithm(ConstAlgorithm::<11>::UA_PERM);

    println!("");
    println!("{}", cube);

    let sol = solver(&mut cube);
    println!("{} (Move count: {})", sol, sol.twists.len());
}

