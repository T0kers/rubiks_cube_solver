# Rubiks cube solver!


## CLI
Run the CLI by running the following in the terminal:
```
cargo run --bin cli --release
```

Type moves using standard cube notation, and the cube represented in the terminal updates. Example:
```
> R U R' U'
------------

    WWO
    WWG
    WWG
BOO GGY RRW BRR
OOO GGW BRR BBB
OOO GGG WRR BBB
    YYR
    YYY
    YYY
```

### Special commands
The CLI offers different special commands that are typed out like:
```
!<name> <arg1> <arg2> ...
```

#### `!quit / !exit`
Terminates the program.

#### `!echo <arg1> <arg2> ...`
Just for testing, just prints out the arguments.
```
> !echo hello world
hello world
```

#### `!solve`
Finds a solution to the current state of the cube using Kociemba's algorithm, which I implemented myself.

#### `!reset`
Resets the cube to the solved state.

#### `!alg <alg_name>`
(WIP: Is not implemented currently)
Executes algorithm to cube, custom algorithms can be inputted into file.
```
> !alg jperm
```

#### `!scramble <length>`
Does <length> random moves to the cube. Moves of the same or opposite face are not redundantly repeated.
```
> !scramble 20
```

### Multiple commands
Mutliple commands can be run in sequence on the same line by sepperating them with '`;`'. Example:

```
> !scramble 20; !solve; !echo yay
```
