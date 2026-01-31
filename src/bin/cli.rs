use rubiks_cube_solver::{cube::{Cube, algs::{Algorithm}}, solver::solver};
use std::{collections::HashMap, fs, io::{self, Write}, path::Path};


fn main() -> io::Result<()> {
    let path = Path::new("./algs");
    let mut registry = AlgRegistry::new();
    read_alg_txt_files(path, String::new(), &mut registry)?;

    let mut cube = Cube::new_solved();
    loop {
        println!("\n{}", cube);
        let line = match read_line() {
            Some(l) => l,
            None => break,
        };

        for part in line.split(";").map(str::trim).filter(|s| !s.is_empty()) {
            if let Some(cmd) = Command::parse(&part) {
                if let Err(e) = cmd.execute(&mut cube, &registry) {
                    eprintln!("Error: {e}");
                }
            } else {
                let alg = Algorithm::from_str(&part);
                cube.apply_algorithm(&alg);
            }
            println!("------------")
        }
    }
    Ok(())
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


    pub fn execute(self, cube: &mut Cube, registry: &AlgRegistry) -> Result<(), String> {
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
                let alg = registry.get(&self.args[0]).ok_or("Algorithm does not exist.")?;
                cube.apply_algorithm(alg);
                Ok(())
            }
            CommandKind::Scramble => {
                let length = self.args.get(0)
                    .ok_or("No arguments provided.")?
                    .parse::<usize>()
                    .map_err(|_| "scramble argument must be a number".to_string())?;

                let mut rng = rand::rng();
                let scramble = Algorithm::new_random(&mut rng, length);
                println!("Scramble: {}", scramble);
                cube.apply_algorithm(&scramble);

                Ok(())
            }
        }
    }
}

pub struct AlgRegistry {
    by_name: HashMap<String, Algorithm>,
}

impl AlgRegistry {
    pub fn new() -> Self {
        Self { by_name: HashMap::new() }
    }
    pub fn get(&self, name: &str) -> Option<&Algorithm> {
        self.by_name.get(name)
    }
    pub fn insert(&mut self, name: String, alg: Algorithm) -> Option<Algorithm> {
        self.by_name.insert(name, alg)
    }
}

fn read_alg_txt_files(dir: &Path, dir_string: String, registry: &mut AlgRegistry) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // recurse into subdirectory
            let prefix = format!("{}{}.", dir_string, path.file_name().unwrap().to_str().unwrap());
            read_alg_txt_files(&path, prefix, registry)?;

        } else if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            let contents = fs::read_to_string(&path)?;
            let prefix = format!("{}{}.", dir_string, path.file_stem().unwrap().to_str().unwrap());

            for line in contents.split("\n") {
                let name_alg = line.split(":").collect::<Vec<&str>>();
                let name = format!("{}{}", prefix, name_alg[0]);
                let alg = Algorithm::from_str(name_alg[1]);
                println!("{}: {}", name, alg);
                registry.insert(name, alg);
            }
        }
    }
    Ok(())
}