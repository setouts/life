#[macro_use]
extern crate lazy_static;

use core::time;
use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
    thread,
};

use futures::lock::Mutex;

//TODO: Multi-threaded game of life rendered in the console.
// Must make a solid rust foundation before advancing on to bigger things.
// Multi-threading is probably overkill, unless I make the simulation complex enough.
// So I guess I must make the simulation complex enough.
// Eventually, should probably switch to rendering to a OpenGL context or canvas?

//TODO: Try to do this functionally.

const RENDER_UPDATE_MS: u64 = 100;
const BOARD_X_SIZE: usize = 100;
const BOARD_Y_SIZE: usize = 100;

#[derive(Clone)]
enum CellState {
    Dead,
    Alive,
}

type Board = Arc<Mutex<Vec<Vec<CellState>>>>;

async fn gen_random_board(the_board: Board) -> anyhow::Result<Board> {
    the_board.lock().await;

    Ok(the_board)
}

fn main() {
    let render_ms_duration = time::Duration::from_millis(RENDER_UPDATE_MS);

    let mut the_board: Board = Arc::new(Mutex::new(vec![
        vec![CellState::Dead; BOARD_X_SIZE];
        BOARD_Y_SIZE
    ]));

    the_board = gen_random_board(the_board)?;

    let mut x = 0;
    loop {
        cod::pixel('o', x, 0);
        cod::flush();

        x += 1;

        thread::sleep(render_ms_duration);

        cod::clear();
    }
}

// ░ █ ▒
