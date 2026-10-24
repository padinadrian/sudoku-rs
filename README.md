# sudoku-rs
### Sudoku solver and game player, built in Rust

## Supported Features

### Solver
* Solve any sudoku puzzle.
* Puzzles can be loaded from file.
* See `examples` folder for examples of how to format game files.

### Player
* (Not yet implemented)

## Usage

```
sudoku-rs - Sudoku solver and game player, built in Rust

Usage: sudoku [COMMAND]

Commands:
  play   Play a game (not yet implemented)
  solve  Solve an existing puzzle from file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### solve

```
Solve an existing puzzle from file

Usage: sudoku solve [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>

Options:
  -o, --output <OUTPUT>  Optional output file to save result
  -h, --help             Print help
```

## License

This project is licensed under the MIT license

More details can be found in the [license](LICENSE) file.

