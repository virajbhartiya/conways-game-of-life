/*
    Conway's Game of Life
    Author: Viraj Bhartiya
*/

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 30;
const INITIAL_ALIVE_PROBABILITY: f32 = 0.3;
const GENERATIONS_PER_SECOND: u64 = 10;

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    age: u8,
}

struct GridStats {
    generation: u64,
    alive_cells: usize,
    dead_cells: usize,
    total_cells: usize,
    alive_percentage: f32,
}

fn main() -> crossterm::Result<()> {
    let mut grid = initialize_grid();
    let mut generation = 0;

    loop {
        clear_screen()?;
        render_grid(&grid)?;
        let stats = calculate_grid_stats(&grid, generation);
        print_stats(&stats);
        grid = update_grid(&grid);
        generation += 1;
        thread::sleep(Duration::from_millis(1000 / GENERATIONS_PER_SECOND));
    }
}

fn initialize_grid() -> Vec<Vec<Cell>> {
    let mut rng = rand::thread_rng();
    let mut grid = vec![
        vec![
            Cell {
                alive: false,
                age: 0
            };
            WIDTH
        ];
        HEIGHT
    ];

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            cell.alive = rng.gen::<f32>() < INITIAL_ALIVE_PROBABILITY;
        }
    }

    let glider = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    for (x, y) in glider.iter() {
        grid[*y][*x].alive = true;
    }

    grid
}

fn render_grid(grid: &Vec<Vec<Cell>>) -> crossterm::Result<()> {
    let mut stdout = stdout();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            execute!(
                stdout,
                MoveTo(x as u16, y as u16),
                SetForegroundColor(cell_color(cell)),
            )?;
            print!("{}", if cell.alive { "■" } else { " " });
        }
    }
    stdout.flush()?;
    Ok(())
}

fn cell_color(cell: &Cell) -> Color {
    if !cell.alive {
        return Color::Grey;
    }
    match cell.age {
        0..=1 => Color::White,
        2..=3 => Color::Yellow,
        4..=6 => Color::Green,
        7..=10 => Color::Cyan,
        11..=15 => Color::Blue,
        _ => Color::Magenta,
    }
}

fn update_grid(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_grid = grid.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let live_neighbors = count_live_neighbors(grid, x, y);
            let current_cell = &grid[y][x];

            new_grid[y][x] = match (current_cell.alive, live_neighbors) {
                (true, 2) | (true, 3) => Cell {
                    alive: true,
                    age: current_cell.age.saturating_add(1),
                },
                (false, 3) => Cell {
                    alive: true,
                    age: 0,
                },
                _ => Cell {
                    alive: false,
                    age: 0,
                },
            };
        }
    }

    new_grid
}

fn count_live_neighbors(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx + WIDTH as i32) % WIDTH as i32;
            let ny = (y as i32 + dy + HEIGHT as i32) % HEIGHT as i32;
            if grid[ny as usize][nx as usize].alive {
                count += 1;
            }
        }
    }
    count
}

fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))
}

fn calculate_grid_stats(grid: &Vec<Vec<Cell>>, generation: u64) -> GridStats {
    let total_cells = WIDTH * HEIGHT;
    let alive_cells = grid.iter().flatten().filter(|&cell| cell.alive).count();
    let dead_cells = total_cells - alive_cells;
    let alive_percentage = (alive_cells as f32 / total_cells as f32) * 100.0;

    GridStats {
        generation,
        alive_cells,
        dead_cells,
        total_cells,
        alive_percentage,
    }
}

fn print_stats(stats: &GridStats) {
    println!("Generation: {}", stats.generation);
    println!(
        "Alive cells: {} ({:.2}%)",
        stats.alive_cells, stats.alive_percentage
    );
    println!("Dead cells: {}", stats.dead_cells);
    println!("Total cells: {}", stats.total_cells);
}
