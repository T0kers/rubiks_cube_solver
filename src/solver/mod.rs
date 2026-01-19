// use crate::cube::Cube;

use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::{sync::OnceLock, usize::MAX};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::cube::{Algorithm, Cube, Turn, TurnDir, Twist, cubie::{CornerOrientation, EdgeId, EdgePos}};


// Define the table type (make it serializable)
#[derive(Serialize, Deserialize, Debug)]
pub struct LookupTable(pub Vec<u8>);

static CORNER_PERMUTATION_TABLE: OnceLock<LookupTable> = OnceLock::new();
const CORNER_PERMUTATION_TABLE_FILE: &str = "tables/corner_permutation.bin";

static CORNER_ORIENTATION_TABLE: OnceLock<LookupTable> = OnceLock::new();
const CORNER_ORIENTATION_TABLE_FILE: &str = "tables/corner_orientation.bin";

pub fn get_permutation_table() -> &'static LookupTable {
    CORNER_PERMUTATION_TABLE.get_or_init(|| {
        let path = Path::new(CORNER_PERMUTATION_TABLE_FILE);
        
        // Try to load from file
        if path.exists() {
            println!("Loading lookup table from file...");
            let data = fs::read(path).expect("Failed to read table file");
            bincode::deserialize(&data).expect("Failed to deserialize table")
        } else {
            println!("Computing lookup table (this may take time)...");
            let table = compute_permutation_table();
            
            // Serialize and save to file
            let data = bincode::serialize(&table).expect("Failed to serialize table");
            fs::write(path, data).expect("Failed to write table file");
            println!("Lookup table saved to file.");
            
            table
        }
    })
}

fn compute_permutation_table() -> LookupTable {
    let mut cube = Cube::new_solved();
    let mut table = vec![std::u8::MAX; 8*7*6*5*4*3*2*1];

    let mut depth = 0;

    while table.contains(&std::u8::MAX) {
        println!("Calculating values for depth {}", depth);
        permutation_table_compute(&mut cube, depth, 0, None, &mut table);
        depth += 1;
    }
    
    LookupTable(table)
}

fn permutation_table_compute(cube: &mut Cube, depth: u8, move_count: u8, prev_turn: Option<Turn>, table: &mut Vec<u8>) {
    if move_count == depth {
        let i = encode_permutation(&cube.corners);
        if table[i] == std::u8::MAX {
            table[i] = depth;
        }
        return;
    }
    for twist in Twist::allowed_moves_from_moveset(&GroupInfo::G1_MOVESET, prev_turn) {
        cube.twist(twist);

        permutation_table_compute(cube, depth, move_count + 1, Some(twist.turn), table);

        cube.twist(twist.inverse());
    }
}

pub fn get_orientation_table() -> &'static LookupTable {
    CORNER_ORIENTATION_TABLE.get_or_init(|| {
        let path = Path::new(CORNER_ORIENTATION_TABLE_FILE);
        
        // Try to load from file
        if path.exists() {
            println!("Loading lookup table from file...");
            let data = fs::read(path).expect("Failed to read table file");
            bincode::deserialize(&data).expect("Failed to deserialize table")
        } else {
            println!("Computing lookup table (this may take time)...");
            let table = compute_orientation_lookup_table();
            
            // Serialize and save to file
            let data = bincode::serialize(&table).expect("Failed to serialize table");
            fs::write(path, data).expect("Failed to write table file");
            println!("Lookup table saved to file.");
            
            table
        }
    })
}

fn compute_orientation_lookup_table() -> LookupTable {
    let mut table = vec![std::u8::MAX; 3usize.pow(7) * 2usize.pow(11)];

    let depth = 0;

    let mut dequeue: VecDeque<(Cube, u8)> = VecDeque::new();

    let cube = Cube::new_solved();
    let orient = cube.get_orientation();
    table[orient] = depth;

    dequeue.push_back((cube, depth + 1));

    while let Some((mut cube, depth)) = dequeue.pop_front() {
        for twist in Twist::ALL_MOVES {
            cube.twist(twist);

            let orient = cube.get_orientation();
            if table[orient] == std::u8::MAX {
                table[orient] = depth;
                dequeue.push_back((cube.clone(), depth + 1));
            }

            cube.twist(twist.inverse());
        }
    }
    assert!(!table.contains(&std::u8::MAX));

    LookupTable(table)
}

fn corner_orientation_heuristic(cube: &Cube) -> usize {
    let mut sum = 0;
    for corner in cube.corners {
        sum += corner.orientation as usize;
    }
    sum.div_ceil(3)
}

fn edge_orientation_heuristic(cube: &Cube) -> usize {
    let mut sum = 0;
    for edge in cube.edges {
        sum += edge.flipped as usize;
    }
    sum.div_ceil(3)
}

fn pattern_heuristic(cube: &Cube) -> usize {
    get_orientation_table().0[cube.get_orientation()] as usize
}

fn g1_heuristic(cube: &Cube) -> usize {
    std::cmp::max(std::cmp::max(corner_orientation_heuristic(cube), edge_orientation_heuristic(cube)), pattern_heuristic(cube))
}

fn solved_heuristic(cube: &Cube) -> usize {
    let i = encode_permutation(&cube.corners);
    get_permutation_table().0[i] as usize
}

// Calculates the right inversion count (Lehmer code) 
// and converts to integer using factorial numbering system
// https://en.wikipedia.org/wiki/Factorial_number_system
// https://en.wikipedia.org/wiki/Lehmer_code
pub fn encode_permutation<T: Copy + Into<usize>, const N: usize>(permutation: &[T; N]) -> usize {
    let perm = permutation.map(|t| t.into());
    let mut factoradic: [usize; N] = [0; N]; // last element is not needed, but rust cant do math with generic parameters :(
    for (i, pi) in perm.iter().take(perm.len() - 1).enumerate() { // skips last because no elements are after
        for pj in perm.iter().skip(i + 1) {
            if pj < pi { factoradic[i] += 1; }
        }
    }

    factoradic_to_decimal(&factoradic)
}

fn factoradic_to_decimal<const N: usize>(factoradic: &[usize; N]) -> usize {
    let mut res = 0;
    let mut factorial = 1;
    for (i, n) in factoradic.iter().rev().enumerate().skip(1) {
        factorial *= i;
        res += n * factorial;
    }
    res
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum DfsResult {
    Found, Excess(usize)
}

fn is_g1(cube: &Cube) -> bool {
    for (i, edge) in cube.edges.iter().enumerate() {
        if edge.flipped { return false;}
        if [EdgePos::BL as usize, EdgePos::BR as usize, EdgePos::FR as usize, EdgePos::FL as usize].contains(&i) {
            if ![EdgeId::BO, EdgeId::BR, EdgeId::GR, EdgeId::GO].contains(&edge.id) {
                return false;
            }
        }
    }
    for corner in cube.corners {
        if corner.orientation != CornerOrientation::Zero { return false; }
    }
    true
}

pub struct GroupInfo {
    pub check: fn(&Cube) -> bool,
    pub heuristic: fn(cube: &Cube) -> usize,
    pub moveset: Vec<Twist>
}

impl GroupInfo {
    pub fn allowed_moves(&self, prev: Option<Turn>) -> impl Iterator<Item = Twist> {
        Twist::allowed_moves_from_moveset(&self.moveset, prev)
    }
    pub const G1_MOVESET: [Twist; 10] = [
        Twist::new(Turn::U, TurnDir::One),
        Twist::new(Turn::U, TurnDir::Two),
        Twist::new(Turn::U, TurnDir::Prime),
        Twist::new(Turn::D, TurnDir::One),
        Twist::new(Turn::D, TurnDir::Two),
        Twist::new(Turn::D, TurnDir::Prime),
        Twist::new(Turn::F, TurnDir::Two),
        Twist::new(Turn::B, TurnDir::Two),
        Twist::new(Turn::L, TurnDir::Two),
        Twist::new(Turn::R, TurnDir::Two),
    ];
}

pub fn solver(cube: &mut Cube) -> Algorithm {
    let start_time = Instant::now();
    let mut alg = group_solver(cube, &GroupInfo { check: is_g1, heuristic: g1_heuristic, moveset: Twist::ALL_MOVES.to_vec() });
    println!("Reached g1 in {:?}: {}", start_time.elapsed(), alg);
    let mut alg2 = group_solver(cube, &GroupInfo { check: Cube::is_solved, heuristic: solved_heuristic, moveset: GroupInfo::G1_MOVESET.to_vec() });
    println!("Solved in {:?}: {}", start_time.elapsed(), alg2);
    alg.append(&mut alg2);
    alg
}

pub fn group_solver(cube: &mut Cube, g_info: &GroupInfo) -> Algorithm {
    let mut bound = (g_info.heuristic)(cube);
    let mut solution = vec![];
    loop {
        println!("Checking bound: {}", bound);
        let result = dfs(cube, 0, bound, None, g_info, &mut solution);
        match result {
            DfsResult::Found => {
                solution.reverse();
                return Algorithm::new(solution);
            }
            DfsResult::Excess(v) => {
                bound = v
            }
        }
    }
}

fn dfs(cube: &mut Cube, g: usize, bound: usize, prev_turn: Option<Turn>, g_info: &GroupInfo, solution: &mut Vec<Twist>) -> DfsResult {
    let f = g + (g_info.heuristic)(cube);
    if f > bound {
        return DfsResult::Excess(f);
    }

    if (g_info.check)(cube) {
        return DfsResult::Found;
    }

    let mut min_excess = MAX;
    for twist in g_info.allowed_moves(prev_turn) {
        cube.twist(twist);
        let t = dfs(cube, g + 1, bound, Some(twist.turn), g_info, solution);

        match t {
            DfsResult::Found => {
                solution.push(twist);
                return DfsResult::Found;
            }
            DfsResult::Excess(v) => {
                min_excess = std::cmp::min(min_excess, v);
            }
        }

        cube.twist(twist.inverse());
    }
    return DfsResult::Excess(min_excess)
}

// https://chatgpt.com/c/6966bb49-2688-832f-8326-ed8b014494ec



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uniqueness_of_encoded_permutation() {
        let mut perm = [0; 8];
        let mut encoded_perms = vec![];
        // options needs to have same amout of elements as perm (not strictly enforced)
        uniqueness_of_encoded_permutation_helper(&mut perm, vec![0, 1, 2, 3, 4, 5, 6, 7], &mut encoded_perms);
        
        println!("{:?}", encoded_perms.iter().take(100).collect::<Vec<_>>());
        // unique check
        for (i, a) in encoded_perms.iter().enumerate() {
            for b in encoded_perms.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
    }

    fn uniqueness_of_encoded_permutation_helper<const N: usize>(perm: &mut [usize; N], options: Vec<usize>, encoded_perms: &mut Vec<usize>) {
        if options.is_empty() {
            encoded_perms.push(encode_permutation(perm));
            return;
        }
        let perm_idx = N - options.len();
        for (i, c) in options.iter().enumerate() {
            let options_without_c: Vec<usize> = options.iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, x)| x.clone())
                .collect();
            perm[perm_idx] = *c;
            uniqueness_of_encoded_permutation_helper(perm, options_without_c, encoded_perms);
        }
    }
}