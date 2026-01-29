use rubiks_cube_solver::{cube::{Cube, algs::{Algorithm}}, solver::solver};
use std::{io::{self, Write}};


fn main() {
    let mut cube = Cube::new_solved();
    loop {
        println!("\n{}", cube);
        let line = match read_line() {
            Some(l) => l,
            None => break,
        };

        for part in line.split(";").map(str::trim).filter(|s| !s.is_empty()) {
            if let Some(cmd) = Command::parse(&part) {
                if let Err(e) = cmd.execute(&mut cube) {
                    eprintln!("Error: {e}");
                }
            } else {
                let alg = Algorithm::from_str(&part);
                cube.apply_algorithm(alg);
            }
            println!("------------")
        }
    }
}

pub fn read_line() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).ok()? == 0 {
        return None;
    }
    Some(input.trim_end().to_string())
}

pub struct Command {
    pub kind: CommandKind,
    pub args: Vec<String>,
}

pub enum CommandKind {
    Quit,
    Echo,
    Solve,
    Reset,
    Alg,
    Scramble,
}


impl Command {
    pub fn parse(line: &str) -> Option<Command> {
        let line = line.strip_prefix('!')?;
        let mut parts = line.split_whitespace();

        let name = parts.next()?;
        let args: Vec<String> = parts.map(String::from).collect();

        let kind = match name {
            "quit" | "exit" => CommandKind::Quit,
            "echo" => CommandKind::Echo,
            "solve" => CommandKind::Solve,
            "reset" => CommandKind::Reset,
            "alg" => CommandKind::Alg,
            "scramble" => CommandKind::Scramble,
            _ => return None,
        };

        Some(Command { kind, args })
    }


    pub fn execute(self, cube: &mut Cube) -> Result<(), String> {
        match self.kind {
            CommandKind::Quit => {
                std::process::exit(0);
            }
            CommandKind::Echo => {
                println!("{}", self.args.join(" "));
                Ok(())
            }
            CommandKind::Solve => {
                let solution = solver(cube);
                println!("Found solution:");
                println!("{} (Move count: {})", solution, solution.twists.len());
                Ok(())
            }
            CommandKind::Reset => {
                *cube = Cube::new_solved();
                Ok(())
            }
            CommandKind::Alg => {
                todo!("Execute alg {}", self.args[0])
            }
            CommandKind::Scramble => {
                let length = self.args.get(0)
                    .ok_or("No arguments provided.")?
                    .parse::<usize>()
                    .map_err(|_| "scramble argument must be a number".to_string())?;

                let mut rng = rand::rng();
                let scramble = Algorithm::new_random(&mut rng, length);
                println!("Scramble: {}", scramble);
                cube.apply_algorithm(scramble);

                Ok(())
            }
        }
    }
}
