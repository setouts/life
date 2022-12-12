#![feature(exclusive_range_pattern)]
#![feature(let_chains)]
#[macro_use]
extern crate lazy_static;

use core::time;
use std::{
    cell::RefCell,
    ops::Rem,
    sync::{Arc, RwLock},
    thread,
};

use futures::{executor::block_on, lock::Mutex};
use rand::{distributions::Distribution, distributions::Uniform, thread_rng, Rng};

// Conway's Game of life with an extra rule rendered to the console.


const UPDATE_MS: u64 = 30;
const BOARD_X_SIZE: usize = 500;
const BOARD_Y_SIZE: usize = 17;

type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Dead,
    Dying,
    Alive,
}

type Board = Vec<CellState>;

type BoardMutex = Arc<Mutex<Board>>;

async fn gen_initial_board() -> Board {
    let sample_range = Uniform::from(0.0..=1.0);
    let mut rng = rand::thread_rng();

    let random_distribution_range: f64 = rng.gen_range(0.0..=1.0);

    let alive_or_dead = |_| match sample_range.sample(&mut rng) {
        sample => {
            if sample > random_distribution_range {
                CellState::Alive
            } else {
                CellState::Dead
            }
        }
    };

    let max_states = BOARD_X_SIZE * BOARD_Y_SIZE;

    let the_board: Board = (0..max_states).map(alive_or_dead).collect();

    the_board
}

fn get_position_at_index(index: usize) -> Position {
    let checked_remainder = |val: usize, by: usize| {
        if let Some(remainder) = val.checked_rem(by) {
            remainder
        } else {
            0
        }
    };

    (
        checked_remainder(index, BOARD_X_SIZE),
        (index / BOARD_X_SIZE),
    )
}

fn get_index_at_position(x: usize, y: usize) -> usize {
    (y * BOARD_X_SIZE) + x
}

async fn update(the_board: &Board) -> Board {
    let count_live_neighbors = |index| -> usize {
        let mut count = 0;
        let (self_x, self_y) = get_position_at_index(index);

        for x in self_x.saturating_sub(1)..=self_x.saturating_add(1) {
            for y in self_y.saturating_sub(1)..=self_y.saturating_add(1) {
                if (x == self_x) & (y == self_y) {
                    continue;
                }

                let potential_neighbor = the_board.get(get_index_at_position(x, y));

                if let Some(neighbor) = potential_neighbor {
                    match neighbor {
                        CellState::Alive | CellState::Dying => count += 1,
                        _ => continue,
                    }
                }
            }
        }

        count
    };

    the_board
        .iter()
        .enumerate()
        .map(|(index, state)| {
            let neighbors = count_live_neighbors(index);

            match neighbors {
                3 if state == &CellState::Dead => CellState::Alive,
                0..2 if state == &CellState::Alive => CellState::Dying,
                2..=3 if state == &CellState::Alive => CellState::Alive,
                6..=7 if state == &CellState::Dying => CellState::Alive,
                // More than 3 neighbors makes the cell dead.
                _ => CellState::Dead,
            }
        })
        .collect()
}

async fn render(the_board: &Board) {
    let draw_at_x = move |(index, cell_state): (usize, &CellState)| {
        let alive_or_dead_char = |&cell_state| match cell_state {
            CellState::Alive | CellState::Dying => '█',
            CellState::Dead => '░',
        };

        let (x, y) = get_position_at_index(index);

        cod::pixel(alive_or_dead_char(cell_state), x as u32, y as u32);
    };

    the_board.iter().enumerate().for_each(draw_at_x);
}

fn main() -> anyhow::Result<()> {
    let render_ms_duration = time::Duration::from_millis(UPDATE_MS);

    let mut board_state = block_on(gen_initial_board());

    loop {
        cod::flush();

        board_state = block_on(update(&board_state));

        block_on(render(&board_state));

        thread::sleep(render_ms_duration);

        cod::clear();
    }
}

// ░ █ ▒
