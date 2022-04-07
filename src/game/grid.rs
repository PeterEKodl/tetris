use super::brick::Brick;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    content: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            content: vec![0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.content[x + y * self.width]
    }

    pub fn add_brick(&mut self, brick: &Brick, grid_x: isize, grid_y: isize) {
        for h in 0..brick.len {
            for w in 0..brick.len {
                if brick.get(w, h) == 1 {
                    let x = (grid_x + w as isize) as usize;
                    let y = (grid_y + h as isize) as usize;
                    self.content[x + y * self.width] = brick.id as u8 + 1;
                }
            }
        }
    }

    pub fn search_rows(&mut self) -> u8 {
        let mut row_cleared = false;
        let mut cleared_rows = 0;

        for row in 0..self.height {
            // Scan for any 0 tiles. If there are none, the row is cleared.
            if self.row_is_full(row) {
                // Fill the row with 0, clearing it.
                self.content[row * self.width..(row + 1) * self.width].fill(0);
                row_cleared = true;
                cleared_rows += 1;
            }
        }

        if row_cleared {
            self.move_tiles_down();
        }

        cleared_rows
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        let row_lower = row1.min(row2);
        let row_upper = row1.max(row2);
        let (left, right) = self.content.split_at_mut((row_lower + 1) * self.width);
        let row_difference = row_upper - (row_lower + 1);
        (&mut left[row_lower * self.width..]).swap_with_slice(
            &mut right[row_difference * self.width..(row_difference + 1) * self.width],
        );
    }

    fn row_is_full(&self, row: usize) -> bool {
        !self.content[row * self.width..(row + 1) * self.width]
            .iter()
            .any(|elem| *elem == 0)
    }

    fn row_is_clear(&self, row: usize) -> bool {
        !self.content[row * self.width..(row + 1) * self.width]
            .iter()
            .any(|elem| *elem != 0)
    }

    fn move_tiles_down(&mut self) {
        /* This algorithm  searches for a clear row and a non clear row above it.
         * Once it finds a pair, it is swapped and both pointers move upwards, repeating
         * this process until either of them reaches the top.
         */

        let mut clear_row: usize = 0;

        // Finds the first clear row from the bottom
        for row in (0..self.height).rev() {
            if self.row_is_clear(row) {
                clear_row = row;
                break;
            }
        }
        // The row that will be swapped with the clear row.
        let mut swap_row: usize = clear_row - 1;

        'outer: loop {
            while self.row_is_clear(swap_row) {
                // The top row has the index zero
                if swap_row == 0 {
                    break 'outer;
                }
                swap_row -= 1;
            }

            self.swap_rows(clear_row, swap_row);
            clear_row -= 1;
            swap_row -= 1;
            if clear_row == 0 {
                break 'outer;
            }
        }
    }

    pub fn check_lose(&self) -> bool {
        for row in 0..4 {
            if !self.row_is_clear(row) {
                return true;
            }
        }
        false
    }
}
