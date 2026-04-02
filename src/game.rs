use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use crate::rotation::get_kicks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    I, O, T, S, Z, J, L
}

impl BlockType {
    pub fn structure(&self) -> Vec<IVec2> {
        // the first position is the pivot for rotation
        match self {
            BlockType::I => vec![IVec2::new(1, 0), IVec2::new(0, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
            BlockType::O => vec![IVec2::new(0, 0), IVec2::new(1, 0), IVec2::new(0, -1), IVec2::new(1, -1)],
            BlockType::T => vec![IVec2::new(1, 0), IVec2::new(0, 0), IVec2::new(2, 0), IVec2::new(1, -1)],
            BlockType::S => vec![IVec2::new(1, 0), IVec2::new(0, 0), IVec2::new(1, -1), IVec2::new(2, -1)],
            BlockType::Z => vec![IVec2::new(1, 0), IVec2::new(1, -1), IVec2::new(0, -1), IVec2::new(2, 0)],
            BlockType::J => vec![IVec2::new(1, 0), IVec2::new(0, 0), IVec2::new(2, 0), IVec2::new(0, -1)],
            BlockType::L => vec![IVec2::new(1, 0), IVec2::new(0, 0), IVec2::new(2, 0), IVec2::new(2, -1)],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            BlockType::I => Color::new(0.0, 1.0, 1.0, 1.0),
            BlockType::O => Color::new(1.0, 0.835, 0.0, 1.0),
            BlockType::T => Color::new(0.625, 0.0, 1.0, 1.0),
            BlockType::S => Color::new(0.0, 0.75, 0.0, 1.0),
            BlockType::Z => Color::new(1.0, 0.0, 0.0, 1.0),
            BlockType::J => Color::new(0.0, 0.2, 1.0, 1.0),
            BlockType::L => Color::new(1.0, 0.533, 0.0, 1.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rot {
    R0,
    R1,
    R2,
    R3,
}

impl Rot {
    pub fn cw(self) -> Self {
        match self {
            Rot::R0 => Rot::R1,
            Rot::R1 => Rot::R2,
            Rot::R2 => Rot::R3,
            Rot::R3 => Rot::R0,
        }
    }

    pub fn ccw(self) -> Self {
        match self {
            Rot::R0 => Rot::R3,
            Rot::R1 => Rot::R0,
            Rot::R2 => Rot::R1,
            Rot::R3 => Rot::R2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub structure: Vec<IVec2>,
    pub rotation: Rot,
    pub color: Color,
    pub block_type: BlockType,
}

pub struct Game {
    pub blocks: Vec<Block>,
    pub level: u32,
    pub score: u32,
    pub active_block_i: Option<usize>,
    pub next_block: Block,
    pub hold_block: Option<Block>,
    pub hold_locked: bool,
    pub width: i32,
    pub height: i32,
    pub last_gravity_update: f64,
    pub block_bag: Vec<BlockType>,

    // Variables for handling smooth movement when holding down left/right/down keys
    pub x_move_begin: f64,
    pub x_move_dir: Option<Direction>,
    pub x_last_move_update: f64,
    pub y_last_move_update: f64,

    // Variables for delayed placement when moving or rotating a block on the ground
    pub lowest_y: i32,
    pub n_resets: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut g = Game {
            blocks: vec![],
            level: 0,
            score: 0,
            active_block_i: None,
            next_block: Block {
                structure: vec![],
                rotation: Rot::R0,
                color: Color::new(0.0, 0.0, 0.0, 0.0),
                block_type: BlockType::I, // Placeholder, will be replaced immediately
            },
            hold_block: None,
            hold_locked: false,
            width: 10,
            height: 20,
            last_gravity_update: get_time(),
            x_move_begin: 0.0,
            x_move_dir: None,
            x_last_move_update: 0.0,
            y_last_move_update: 0.0,
            lowest_y: 0,
            n_resets: 0,
            block_bag: vec![],
        };

        g.next_block = g.gen_next();
        
        g
    }

    pub fn reset_game(&mut self) {
        *self = Self::new();
    }

    fn gen_next(&mut self) -> Block {
        // generate a random block
        if self.block_bag.is_empty() {
            self.block_bag = vec![BlockType::I, BlockType::O, BlockType::T, BlockType::S, BlockType::Z, BlockType::J, BlockType::L];
            self.block_bag.shuffle();
        }

        let block_type = self.block_bag.pop().unwrap();

        Block {
            structure: block_type.structure(),
            rotation: Rot::R0,
            color: block_type.color(),
            block_type,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active_block_i.is_some()
    }

    fn drop_block(&mut self, mut block: Block) {
        // Transform the next block structure to the top of the board.
        let highest_block = self.blocks.iter().filter_map(|b| b.structure.iter().map(|p| p.y).min()).min().unwrap_or(self.height);

        let spawn_y_offset = if highest_block == 3 {
            -1
        } else if highest_block <= 2 {
            -2
        } else {
            0
        };

        let spawn_x = match block.block_type {
            BlockType::O => (self.width / 2) - 1,
            _ => (self.width / 2) - 2,
        };
        
        let spawn_y = match block.block_type {
            BlockType::I => 0,
            _ => 1
        } + spawn_y_offset;

        for pos in &mut block.structure {
            pos.x += spawn_x;
            pos.y += spawn_y;
        }

        self.active_block_i = Some(self.blocks.len());
        self.blocks.push(block);

        self.lowest_y = self.blocks[self.active_block_i.unwrap()].structure.iter().map(|p| p.y).max().unwrap();
    }

    pub fn drop_next(&mut self) {
        let new_next = self.gen_next();
        let next = std::mem::replace(&mut self.next_block, new_next);
        self.drop_block(next);
    }

    pub fn move_active(&mut self, direction: Direction) -> bool {
        if let Some(i) = self.active_block_i {
            let delta: (i32, i32) = match direction {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
            };

            let valid = {
                self.blocks[i].structure.iter().all(|pos| {
                    let new_x = pos.x + delta.0;
                    let new_y = pos.y + delta.1;

                    new_x >= 0
                        && new_x < self.width
                        && new_y < self.height
                        && !self.blocks.iter().enumerate().any(|(j, b)| {
                            j != i && b.structure.iter().any(|p| p.x == new_x && p.y == new_y)
                        })
                })
            };

            if valid {
                for pos in &mut self.blocks[i].structure {
                    pos.x = pos.x + delta.0;
                    pos.y = pos.y + delta.1;
                }

                if direction != Direction::Down {
                    if self.is_grounded() {
                        self.reset();
                    }
                } else {
                    let block_lowest_y = self.blocks[i].structure.iter().map(|p| p.y).max().unwrap();

                    if block_lowest_y > self.lowest_y {
                        self.lowest_y = block_lowest_y;
                        self.n_resets = 0;
                        self.last_gravity_update = get_time();
                    }
                }
            }

            valid
        }
        else {
            false
        }
    }

    pub fn get_active_shadow(&self) -> Option<Vec<IVec2>> {
        if let Some(i) = self.active_block_i {
            let mut shadow = self.blocks[i].structure.clone();
            while shadow.iter().all(|pos| {
                let new_y = pos.y + 1;
                new_y < self.height
                    && !self.blocks.iter().enumerate().any(|(j, b)| {
                        j != i && b.structure.iter().any(|p| p.x == pos.x && p.y == new_y)
                    })
            }) {
                for pos in &mut shadow {
                    pos.y += 1;
                }
            }
            Some(shadow)
        } else {
            None
        }
    }

    fn get_pivot(&self, block: &Block) -> Vec2 {
        match block.block_type {
            BlockType::I => {
                block.structure[0].as_vec2() + match block.rotation {
                    Rot::R0 => Vec2::new(0.5, 0.5),
                    Rot::R1 => Vec2::new(-0.5, 0.5),
                    Rot::R2 => Vec2::new(-0.5, -0.5),
                    Rot::R3 => Vec2::new(0.5, -0.5),
                }
            }
            _ => Vec2::new(block.structure[0].x as f32, block.structure[0].y as f32),
        }
    }

    pub fn rotate_active(&mut self, clockwise: bool) -> bool {
        if let Some(i) = self.active_block_i {
            if self.blocks[i].block_type == BlockType::O {
                return true; // O block doesn't rotate
            }

            let old_rotation = self.blocks[i].rotation;
            let new_rotation = if clockwise {
                old_rotation.cw()
            } else {
                old_rotation.ccw()
            };

            let is_i = self.blocks[i].block_type == BlockType::I;
            let kicks = get_kicks(old_rotation, new_rotation, is_i);

            let pivot = self.get_pivot(&self.blocks[i]);

            let rotated_structure: Vec<IVec2> = self.blocks[i]
                .structure
                .iter()
                .map(|pos| {
                    let rel_x = pos.x as f32 - pivot.x;
                    let rel_y = pos.y as f32 - pivot.y;

                    let (rx, ry) = if clockwise {
                        (-rel_y, rel_x)
                    } else {
                        (rel_y, -rel_x)
                    };

                    IVec2::new((rx + pivot.x).round() as i32, (ry + pivot.y).round() as i32)
                })
                .collect();

            let valid_kick = kicks.iter().find(|kick| {
                rotated_structure.iter().all(|pos| {
                    let new_x = pos.x + kick.0;
                    let new_y = pos.y + kick.1;

                    new_x >= 0
                        && new_x < self.width
                        && new_y < self.height
                        && !self.blocks.iter().enumerate().any(|(j, b)| {
                            j != i && b.structure.iter().any(|p| p.x == new_x && p.y == new_y)
                        })
                })
            });

            if let Some(kick) = valid_kick {
                let block = &mut self.blocks[i];
                block.rotation = new_rotation;
                
                for (j, pos) in block.structure.iter_mut().enumerate() {
                    pos.x = rotated_structure[j].x + kick.0;
                    pos.y = rotated_structure[j].y + kick.1;
                }

                if self.is_grounded() {
                    self.reset();
                }

                return true;
            }
        }

        false
    }

    pub fn hold(&mut self) {
        if self.hold_locked {
            return;
        }

        if let Some(i) = self.active_block_i {
            let active = self.blocks.remove(i);

            if let Some(hold) = self.hold_block.take() {
                self.drop_block(hold);
            } else {
                self.drop_next();
            }

            self.hold_block = Some(Block {
                structure: active.block_type.structure(),
                rotation: Rot::R0,
                color: active.block_type.color(),
                block_type: active.block_type,
            });

            self.hold_locked = true;
        }
    }

    fn reset(&mut self) {
        if self.n_resets < 15 {
            self.n_resets += 1;
            self.last_gravity_update = get_time();
        }
    }

    pub fn update_gravity(&mut self) {
        let now = get_time();

        if self.is_grounded() {
            let elapsed = now - self.last_gravity_update;
            if elapsed >= 0.5 || self.n_resets >= 15 {
                self.place();
                self.n_resets = 0;
            }
        } else {
            let gravity_interval = (0.8 - 0.007 * (self.level as f64 - 1.0)).powf(self.level as f64 - 1.0);

            if now - self.last_gravity_update >= gravity_interval {
                self.move_active(Direction::Down);
            }
        }
    }

    fn is_grounded(&self) -> bool {
        if let Some(i) = self.active_block_i {
            self.blocks[i].structure.iter().any(|pos| {
                pos.y == self.height - 1
                    || self.blocks.iter().enumerate().any(|(j, b)| {
                        j != i && b.structure.iter().any(|p| p.x == pos.x && p.y == pos.y + 1)
                    })
            })
        } else {
            false
        }
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut y = self.height - 1;
        while y >= -2 {
            if self.blocks.iter().any(|b| b.structure.iter().any(|p| p.y == y)) {
                let mut line_full = true;
                for x in 0..self.width {
                    if !self.blocks.iter().any(|b| b.structure.iter().any(|p| p.x == x && p.y == y)) {
                        line_full = false;
                        break;
                    }
                }

                if line_full {
                    lines_cleared += 1;
                    // Remove all blocks in this line
                    for block in &mut self.blocks {
                        block.structure.retain(|p| p.y != y);
                    }

                    // Move all blocks above this line down
                    for block in &mut self.blocks {
                        for pos in &mut block.structure {
                            if pos.y < y {
                                pos.y += 1;
                            }
                        }
                    }

                    // Don't increment y, since we need to check the same line again after moving blocks down
                } else {
                    y -= 1;
                }
            } else {
                y -= 1;
            }
        }

        // Update score and level based on lines cleared
        self.score += match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };

        self.level = (self.score / 1000) + 1;
    }

    pub fn place(&mut self) -> bool {
        while self.move_active(Direction::Down) {}

        self.clear_lines();

        if let Some(i) = self.active_block_i {
            if self.blocks[i].structure.iter().any(|pos| pos.y == -2) {
                // Game over
                self.active_block_i = None;
                return false;
            }
        }

        self.drop_next();

        self.hold_locked = false;

        true
    }
}