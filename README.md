# Game of Life Simulation

This project implements Conway's Game of Life in Rust with additional features and statistics. The simulation runs in the terminal using the `crossterm` library for rendering.

## Features

- Terminal-based visualization
- Customizable grid size
- Random initial population
- Glider pattern initialization
- Cell aging and color representation
- Advanced statistics and metrics

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/virajbharrtiya/conway.git
   cd conway
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the simulation:

```
cargo run --release
```

The simulation will start automatically. Press `Ctrl+C` | `^ + C` to exit.

## Configuration

You can modify the following constants in `src/main.rs` to customize the simulation:

- `WIDTH`: Grid width (default: 80)
- `HEIGHT`: Grid height (default: 30)
- `INITIAL_ALIVE_PROBABILITY`: Probability of a cell being alive at initialization (default: 0.3)
- `GENERATIONS_PER_SECOND`: Number of generations simulated per second (default: 10)

## Statistics

The simulation displays the following statistics:

- Generation count
- Number of alive and dead cells
- Percentage of alive cells
- Number of stable patterns
- Number of oscillators
- Size of the largest cluster
- Entropy of the grid

## Implementation Details

The project is implemented in Rust and uses the following main components:

- `Cell` struct: Represents individual cells with alive status and age
- `GridStats` struct: Holds calculated statistics for the current grid state
- Main loop: Handles grid initialization, updating, rendering, and statistics calculation
- Helper functions: Implement Game of Life rules, neighbor counting, and advanced statistics calculations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
