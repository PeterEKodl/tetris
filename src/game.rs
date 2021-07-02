use std::time::Duration;

use crate::render;
use rustbox::*;

pub mod grid;
use grid::*;

pub mod brick;
use brick::*;

pub struct Game
{
    pub score:usize,
    grid: Grid,
    preview: Brick,
    brick: Brick,
    brick_x: isize,
    brick_y: isize,
    move_interval: Duration,
    elapsed: Duration,
    pub game_over: bool,
    cleared_rows_total: usize
}

impl Game
{
    
    const INITIAL_MOVE_INTERVAL: Duration = Duration::from_millis(750);
    const MIN_MOVE_INTERVAL: Duration = Duration::from_millis(200);

    pub fn new(grid_width: usize, grid_height: usize) -> Self
    {
        Self
        {
            score:0,
            grid: Grid::new(grid_width, grid_height),
            preview: Brick::new_rand(),
            brick: Brick::new_rand(),
            brick_x:((grid_width-4)/2) as isize,
            brick_y:0,
            move_interval: Self::INITIAL_MOVE_INTERVAL,
            elapsed: Duration::ZERO,
            game_over: false,
            cleared_rows_total: 0
        }
    }

    pub fn update(&mut self, delta: Duration)
    {
        let mut drop_ready = false;
        self.elapsed += delta;
        if self.elapsed >= self.move_interval
        {
            self.elapsed -= self.move_interval;
            drop_ready = true; 
        }
        
        
        if drop_ready
        {
            self.move_brick_down(); 
        }

    }

    pub fn rotate(&mut self, rotation: actions::Rotation)
    {
        let mut brick_rotated = self.brick.clone();
        brick_rotated.rotate(rotation);

        if !check_overlap(&brick_rotated, &self.grid, self.brick_x, self.brick_y)
        {
            self.brick.rotate(rotation);
        } else
        {
            // If the rotation fails, the game will try to shift the brick to either side to fit
            // it.
            // Checks the offsets -1 and 1 and then -2 and 2
            for f in 1..=2
            {
                for offset in -1..=1
                {
                    if offset != 0
                    {
                        if !check_overlap(&brick_rotated, &self.grid, self.brick_x + offset*f, self.brick_y)
                        {
                            self.brick.rotate(rotation);
                            self.brick_x += offset*f;
                            return;
                        }
                    }
                }
            }
        }

    }

    pub fn shift(&mut self, direction: actions::Shift)
    {
        match direction
        {
            actions::Shift::Left => 
            {
                if !check_overlap(&self.brick, &self.grid, self.brick_x - 1, self.brick_y)
                {
                    self.brick_x -= 1;
                }
            },
            actions::Shift::Right => 
            {
                if !check_overlap(&self.brick, &self.grid, self.brick_x + 1, self.brick_y)
                {
                    self.brick_x += 1;
                }
            },
            actions::Shift::Down =>
            {
                self.move_brick_down();
                // Resets the elapsed time counter to allow for better control of the brick.
                self.elapsed = Duration::ZERO;
            }
        }
    }

    fn move_brick_down(&mut self)
    {
        if !check_overlap(&self.brick, &self.grid, self.brick_x, self.brick_y + 1)
        {
            self.brick_y += 1;
        } else 
        {
            self.grid.add_brick(&self.brick, self.brick_x, self.brick_y);
            let rows_cleared = self.grid.search_rows();
            self.cleared_rows_total += rows_cleared as usize;

            // Decreases the move interval based on the level
            self.move_interval = (Self::INITIAL_MOVE_INTERVAL - Duration::from_millis((self.get_level() * 40) as u64))
                .max(Self::MIN_MOVE_INTERVAL);
            
            self.score += match rows_cleared
            {
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => 0
            } * (self.get_level() + 1);

            if self.grid.check_lose()
            {
                self.game_over = true;
                return;
            }

            // Creates a new brick
            self.brick_y = 0;
            self.brick_x = ((self.grid.width-4)/2) as isize;
            self.brick = self.preview;
            self.preview = Brick::new_rand();
        }

    }

    fn get_level(&self) -> usize
    {
         self.cleared_rows_total / 5
    }


}

impl render::Render for Game
{
    fn render(&self, rustbox:&RustBox)
    {
        let window_width = self.grid.width + 2;
        let window_height = self.grid.height + 2;

        // Center the playing field.
        let corner_x = (rustbox.width() - window_width)/2;
        let corner_y = (rustbox.height() - window_height)/2;

        // Draw the grid frame
        render::render_box(corner_x, corner_y, self.grid.width+2, self.grid.height+2,
                           Color::Blue, rustbox);

        // Draw bricks that have been placed
        for h in 0..self.grid.height
        {
            for w in 0..self.grid.width
            {
                let id = self.grid.get(w, h);
                if id != 0
                {
                    let texture = brick_texture(id as usize);
                    rustbox.print_char(w + corner_x + 1, h + corner_y + 1, RB_REVERSE, texture.0, Color::Default, texture.1);
                } else
                {
                    if w % 2 == 0
                    {
                        rustbox.print_char(w + corner_x + 1, h + corner_y + 1, RB_NORMAL, Color::White, Color::Default, '.');
                    } else 
                    {
                        rustbox.print_char(w + corner_x + 1, h + corner_y + 1, RB_NORMAL, Color::White, Color::Default, '|');
                    }
                }
            }
        }
        render::render_filled_box(corner_x+1, corner_y+1, self.grid.width, 4, Color::White, '+', rustbox);
        render_brick((corner_x as isize + 1 + self.brick_x) as usize, (corner_y as isize + 1 + self.brick_y) as usize, &self.brick, rustbox);
        
        // Draw score
        rustbox.print(corner_x, corner_y-2, 
                      RB_NORMAL, Color::White, Color::Default, &format!("SCORE: {}", self.score));

        // Draw level
        rustbox.print(corner_x, corner_y-1, RB_NORMAL, Color::White, Color::Default, &format!("LVL: {}", self.get_level()));
        
        // Draw the preview display.
        let corner_x = corner_x + self.grid.width + 1; 
        render::render_box(corner_x, corner_y, 6, 6, Color::Blue, rustbox);

        render_brick(corner_x + 1, corner_y + 1, &self.preview, rustbox);


    }
}

fn check_overlap(brick: &Brick, grid: &Grid, brick_x: isize, brick_y: isize) -> bool
{
    for y in 0..brick.len
    {
        for x in 0..brick.len
        {
            if brick.get(x, y) == 1
            {
                let grid_x = x as isize + brick_x;
                let grid_y = y as isize + brick_y;
                if (0..(grid.width as isize)).contains(&grid_x) && (0..(grid.height as isize)).contains(&grid_y)
                {
                    if grid.get(grid_x as usize, grid_y as usize) != 0
                    {
                        return true;
                    }
                } else 
                {
                    return true;
                }
                
            }
        }
    }
    return false;
}

fn brick_texture(id:usize) -> (Color, char)
{
    match id
    {
        1 => (Color::Cyan, ' '),
        2 => (Color::Green, ' '),
        3 => (Color::Red, ' '),
        4 => (Color::Red, ' '),
        5 => (Color::Green, ' '),
        6 => (Color::Yellow, ' '),
        7 => (Color::Magenta, ' '),
        0|_ => (Color::Black, ' '),
    }
}

fn render_brick(x:usize, y:usize, brick:&Brick, rustbox:&RustBox)
{
    for h in 0..brick.len
    {
        for w in 0..brick.len
        {
            if brick.get(w, h) == 1
            {
                let texture = brick_texture(brick.id + 1);
                rustbox.print_char(x + w, y + h, RB_REVERSE, texture.0, Color::Default, texture.1);
            }
        }
    }
}

pub mod actions
{
    #[derive(Clone, Copy)]
    pub enum Rotation
    {
        Clockwise,
        CounterClockwise
    }

    #[derive(Clone, Copy)]
    pub enum Shift
    {
        Left,
        Right,
        Down
    }
}
