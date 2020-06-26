use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::fmt::Error;
use std::thread as thread;
use std::time::Duration;
use gol_builder::GoLBuilder;
use gol_builder::BuildGol;
use pancurses::Window;

/// Defines a wrapper to multiple calls to pancurses' `addch`.
trait WideWrapper {
    /// Adds a char to the window and advances.
    fn addch_wide(&self, ch: char);
}

impl WideWrapper for Window {
#[cfg(unix)]
    /// Adds a char to the window and advances.
    ///
    /// ## Remarks
    ///
    /// I see this as terribly ugly hack. But it's saturday morning, and I want
    /// to commit&push this kata.
    fn addch_wide(&self, ch: char) {
        let mut utf8 = vec![0; ch.len_utf8()];
        ch.encode_utf8(&mut utf8);

        for b in &utf8 {
            self.addch(*b as chtype);
        }
    }
#[cfg(windows)]
    fn addch_wide(&self, ch: char) {
        self.addch(ch);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct MatGol {
    pub rows: usize,
    pub cols: usize,
    pub matrix: Vec<u8>
}

impl MatGol {
    /// Gets the cell at `(row, col)` and pads borders with 0s.
    fn at(&self, row: i32, col: i32) -> u8 {
        if
            row < 0 ||
            col < 0 ||
            row >= self.rows as i32 ||
            col >= self.cols as i32 {
            return 0
        }

        self.matrix[row as usize * self.cols + col as usize]
    }

    /// Gets the cell at `(row, col)` without checks.
    fn at_unchecked(&self, row: usize, col: usize) -> u8 {
        self.matrix[row * self.cols + col]
    }

    /// Gets a mutable reference to the cell at `(row, col)`.
    fn at_mut(&mut self, row: usize, col: usize) -> &mut u8 {
/*        if
            row < 0 ||
            col < 0 ||
            row >= self.rows ||
            col >= self.cols {
            return 0
        }
*/
        &mut self.matrix[row * self.cols + col]
    }

    /// Renders this matrix on a curses `Window`.
    fn curses_render(&self, w: &Window) {
        let charmap: [char; 4] = [' ', '▀', '▄', '█'];

        for ((r1, r2), y) in
            (0..self.rows).step_by(2)
            .zip((1..self.rows).step_by(2))
            .zip(0..) {
            w.mv(y, 0);

            for c in 0..self.cols {
                w.addch_wide(charmap[
                    (self.at_unchecked(r1, c) +
                    self.at_unchecked(r2, c) * 2) as usize
                ]);
            }
        }
    }
}

impl GoLBuilder for MatGol {
    fn empty(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![0; cols * rows]
        }
    }

    fn live(mut self, row: usize, col: usize) -> Self {
        *self.at_mut(row, col) = 1;
        self
    }

    fn dead(mut self, row: usize, col: usize) -> Self {
        *self.at_mut(row, col) = 0;
        self
    }
}

impl Display for MatGol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for (row, col) in (0..self.rows).zip(0..self.cols) {
            if self.at_unchecked(row, col) == 1u8 {
                write!(f, "♦.")?;
            } else {
                write!(f, "..")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

/// Creates the next frame. The algorithm uses a convolution matrix. A cool thing
/// to do wuold be to check if it gets vectorized.
fn next(gol: MatGol) -> MatGol {
    let mut result = MatGol::empty(gol.rows, gol.cols);
    let filter = [
        [1,1,1],
        [1,10,1],
        [1,1,1]];

    for row in 0..gol.rows {
        for col in 0..gol.cols {
            let mut convolution = 0u8;

            for conv_row in 0..3 {
                for conv_col in 0..3 {
                    convolution += filter[conv_row][conv_col] * gol.at(
                        row as i32 + conv_row as i32 - 1,
                        col as i32 + conv_col as i32 - 1
                    );
                }
            }

            let is_alive = match convolution {
                3 => true,
                12 | 13 => true,
                _ => false,
            };

            if is_alive {
                result = result.live(row, col);
            }
        }
    }

    result
}

fn main() {
    let mut model: MatGol ="\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    .............+..................................................................\n\
    ..............+.................................................................\n\
    ............+++.................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................\n\
    ................................................................................"
    .build_gol()
    .unwrap();

    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::cbreak();
    pancurses::noecho();
    window.nodelay(true);
    window.keypad(true);

    for _ in 0.. {
        window.clear();
        model = next(model);
        model.curses_render(&window);
        window.refresh();
        thread::sleep(Duration::from_millis(100));
    }

    pancurses::endwin();
}
