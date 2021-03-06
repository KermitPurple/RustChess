// TODO: More Docs <20-12-20, Shane McDonough> //
//! This is a chess program.
//! it uses the ggez graphics and game library to.
//! create a graphics user interface and get mouse input.
//!
//! IMPORTANT: alsa-sys v0.1.2, a package used by ggez causes a panic in some linux distros.
//! This can be solved by typing ```sudo apt install libsdl2-dev``` into the command line.
//!
//! This project is a collaboration between Patrick and Shane McDonough.

use ggez::event;
use ggez::graphics;
use ggez::input;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

/// The size of the main window in pixels.
/// The first number is the x coordinate and the second is the y.
const WINDOW_SIZE: [f32; 2] = [700., 700.];
/// The number of tiles across the board.
const BOARD_SIZE: usize = 8;
/// The first number is the x coordinate and the second is the y.
/// the size of a single tile in pixels.
const SQUARE_SIZE: [f32; 2] = [
    WINDOW_SIZE[0] / BOARD_SIZE as f32,
    WINDOW_SIZE[1] / BOARD_SIZE as f32,
];

/// The two different colors a chess piece can be.
#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

/// An enum that represents a spot on a chess board.
/// Holds the team information and which type of piece it is.
/// Alternatively, It could represent and empty space on the chessboard.
#[derive(Copy, Clone)]
enum Piece {
    Empty,
    Black(Type),
    White(Type),
}

/// An enum that represents each type of chess piece there is.
/// Does not identify team at all.
#[derive(Copy, Clone)]
enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// This is the current game state.
struct State {
    /// board represents the pieces are and their location in the chess board.
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    /// color represents which team currently has a turn.
    color: Color,
    /// the position of the currently selected piece
    selected_pos: Option<[f32; 2]>,
}

impl State {
    /// creates a new State with all pieces in the correct starting position.
    fn new() -> Self {
        Self {
            board: [
                [
                    Piece::White(Type::Rook),
                    Piece::White(Type::Knight),
                    Piece::White(Type::Bishop),
                    Piece::White(Type::Queen),
                    Piece::White(Type::King),
                    Piece::White(Type::Bishop),
                    Piece::White(Type::Knight),
                    Piece::White(Type::Rook),
                ],
                [Piece::White(Type::Pawn); BOARD_SIZE],
                [Piece::Empty; BOARD_SIZE],
                [Piece::Empty; BOARD_SIZE],
                [Piece::Empty; BOARD_SIZE],
                [Piece::Empty; BOARD_SIZE],
                [Piece::Black(Type::Pawn); BOARD_SIZE],
                [
                    Piece::Black(Type::Rook),
                    Piece::Black(Type::Knight),
                    Piece::Black(Type::Bishop),
                    Piece::Black(Type::Queen),
                    Piece::Black(Type::King),
                    Piece::Black(Type::Bishop),
                    Piece::Black(Type::Knight),
                    Piece::Black(Type::Rook),
                ],
            ],
            color: Color::Black,
            selected_pos: None,
        }
    }

    /// Draws the white tiles of the chess board against the black background.
    fn draw_board(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, graphics::BLACK);
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 0.,
                y: 0.,
                w: SQUARE_SIZE[0],
                h: SQUARE_SIZE[1],
            },
            graphics::WHITE,
        )
        .unwrap();
        graphics::draw(ctx, &square, (na::Point2::new(0., 0.),)).unwrap();
        for i in 0..4 {
            for j in 0..4 {
                graphics::draw(
                    ctx,
                    &square,
                    (na::Point2::new(
                        j as f32 * SQUARE_SIZE[0] * 2.,
                        i as f32 * SQUARE_SIZE[1] * 2.,
                    ),),
                )
                .unwrap();
                graphics::draw(
                    ctx,
                    &square,
                    (na::Point2::new(
                        j as f32 * SQUARE_SIZE[0] * 2. + SQUARE_SIZE[0],
                        i as f32 * SQUARE_SIZE[1] * 2. + SQUARE_SIZE[1],
                    ),),
                )
                .unwrap();
            }
        }
    }

    /// Draws the chess piece that occupies the given position.
    fn draw_piece(&mut self, ctx: &mut Context, pos: [f32; 2]) {
        let piece = self.board[pos[1] as usize][pos[0] as usize];
        let color: graphics::Color;
        let text_color: graphics::Color;
        match piece {
            Piece::Empty => return,
            Piece::Black(_) => {
                color = [0.2, 0.2, 0.2, 1.0].into();
                text_color = [0.8, 0.8, 0.8, 1.0].into()
            }
            Piece::White(_) => {
                color = [0.8, 0.8, 0.8, 1.0].into();
                text_color = [0.2, 0.2, 0.2, 1.0].into()
            }
            _ => unreachable!(),
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [SQUARE_SIZE[0] / 2., SQUARE_SIZE[1] / 2.],
            SQUARE_SIZE[0] * 0.4,
            0.1,
            color,
        )
        .unwrap();
        let border = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.),
            [SQUARE_SIZE[0] / 2., SQUARE_SIZE[1] / 2.],
            SQUARE_SIZE[0] * 0.4,
            0.1,
            [0.5, 0.5, 0.5, 1.0].into(),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &circle,
            (na::Point2::new(
                pos[0] * SQUARE_SIZE[0],
                pos[1] * SQUARE_SIZE[1],
            ),),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &border,
            (na::Point2::new(
                pos[0] * SQUARE_SIZE[0],
                pos[1] * SQUARE_SIZE[1],
            ),),
        )
        .unwrap();
        let text_fragment: graphics::TextFragment;
        match piece {
            Piece::Black(Type::Pawn) | Piece::White(Type::Pawn) => return,
            Piece::Black(Type::Rook) | Piece::White(Type::Rook) => {
                text_fragment = graphics::TextFragment::new("R")
            }
            Piece::Black(Type::Knight) | Piece::White(Type::Knight) => {
                text_fragment = graphics::TextFragment::new("N")
            }
            Piece::Black(Type::Bishop) | Piece::White(Type::Bishop) => {
                text_fragment = graphics::TextFragment::new("B")
            }
            Piece::Black(Type::Queen) | Piece::White(Type::Queen) => {
                text_fragment = graphics::TextFragment::new("Q")
            }
            Piece::Black(Type::King) | Piece::White(Type::King) => {
                text_fragment = graphics::TextFragment::new("K")
            }
            _ => return,
        }
        graphics::draw(
            ctx,
            graphics::Text::new(
                text_fragment
                    .color(text_color)
                    .scale(graphics::Scale { x: 40., y: 40. }),
            )
            .set_bounds(SQUARE_SIZE, graphics::Align::Center),
            (na::Point2::new(
                pos[0] * SQUARE_SIZE[0],
                pos[1] * SQUARE_SIZE[1] + SQUARE_SIZE[1] / 4.,
            ),),
        )
        .unwrap();
    }

    /// Draws every chess piece on the board.
    fn draw_pieces(&mut self, ctx: &mut Context) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                self.draw_piece(ctx, [j as f32, i as f32]);
            }
        }
    }

    /// gets the index of the current square that the mouse is hovering over.
    fn get_current_square(&mut self, ctx: &mut Context) -> [f32; 2] {
        let pos = input::mouse::position(ctx);
        [
            (pos.x / SQUARE_SIZE[0]) as usize as f32,
            (pos.y / SQUARE_SIZE[1]) as usize as f32,
        ]
    }

    /// highlights the square at the given position.
    fn highlight_square(&mut self, ctx: &mut Context, pos: [f32; 2], color: graphics::Color) {
        let highlight = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 0.,
                y: 0.,
                w: SQUARE_SIZE[0],
                h: SQUARE_SIZE[1],
            },
            color,
        )
        .unwrap();
        graphics::draw(
            ctx,
            &highlight,
            (na::Point2::new(
                pos[0] * SQUARE_SIZE[0],
                pos[1] * SQUARE_SIZE[1],
            ),),
        )
        .unwrap();
    }

    /// lists the coordinates of valid moves
    fn get_valid_moves(&mut self, pos: [f32; 2]) -> Vec<[f32; 2]> {
        // TODO: Make piece logic more efficient and not stupid <20-12-20, Shane McDonough>
        // TODO: Make pawns able to kill by sliding past <20-12-20, Shane McDonough>
        // TODO: Create king logic <20-12-20, Shane McDonough>
        let mut v: Vec<[f32; 2]> = vec![];
        let piece = self.board[pos[1] as usize][pos[0] as usize];
        match piece {
            Piece::Black(Type::Pawn) => {
                self.push_move([pos[0], pos[1] - 1.], false, &mut v);
                // starting line
                if pos[1] == 6. {
                    self.push_move([pos[0], pos[1] - 2.], false, &mut v);
                }
                let mut new_pos = [pos[0] + 1., pos[1] - 1.];
                if !self.point_out_of_bounds(new_pos) {
                    match self.board[new_pos[1] as usize][new_pos[0] as usize] {
                        Piece::White(_) => v.push(new_pos),
                        _ => (),
                    }
                    match self.board[new_pos[1] as usize][new_pos[0] as usize + 1] {
                        Piece::White(_) => v.push(new_pos),
                        _ => (),
                    }
                }
                new_pos = [pos[0] - 1., pos[1] - 1.];
                if !self.point_out_of_bounds(new_pos) {
                    match self.board[new_pos[1] as usize][new_pos[0] as usize] {
                        Piece::White(_) => v.push(new_pos),
                        _ => (),
                    }
                    match self.board[new_pos[1] as usize][new_pos[0] as usize + 1] {
                        Piece::White(_) => v.push(new_pos),
                        _ => (),
                    }
                }
            }
            Piece::White(Type::Pawn) => {
                self.push_move([pos[0], pos[1] + 1.], false, &mut v);
                // starting line
                if pos[1] == 1. {
                    self.push_move([pos[0], pos[1] + 2.], false, &mut v);
                }
                let mut new_pos = [pos[0] + 1., pos[1] + 1.];
                if !self.point_out_of_bounds(new_pos) {
                    match self.board[new_pos[1] as usize][new_pos[0] as usize] {
                        Piece::Black(_) => v.push(new_pos),
                        _ => (),
                    }
                }
                new_pos = [pos[0] - 1., pos[1] + 1.];
                if !self.point_out_of_bounds(new_pos) {
                    match self.board[new_pos[1] as usize][new_pos[0] as usize] {
                        Piece::Black(_) => v.push(new_pos),
                        _ => (),
                    }
                }
            }
            Piece::Black(Type::Knight) | Piece::White(Type::Knight) => {
                self.push_move([pos[0] + 2., pos[1] + 1.], true, &mut v);
                self.push_move([pos[0] - 2., pos[1] + 1.], true, &mut v);
                self.push_move([pos[0] + 2., pos[1] - 1.], true, &mut v);
                self.push_move([pos[0] - 2., pos[1] - 1.], true, &mut v);
                self.push_move([pos[0] + 1., pos[1] + 2.], true, &mut v);
                self.push_move([pos[0] - 1., pos[1] + 2.], true, &mut v);
                self.push_move([pos[0] + 1., pos[1] - 2.], true, &mut v);
                self.push_move([pos[0] - 1., pos[1] - 2.], true, &mut v);
            }
            Piece::Black(Type::Rook) | Piece::White(Type::Rook) => {
                let mut offset = 1.;
                while self.push_move([pos[0] + offset, pos[1]], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1]], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = -1.;
                while self.push_move([pos[0] + offset, pos[1]], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1]], false) {
                        break;
                    }
                    offset -= 1.;
                }
                offset = 1.;
                while self.push_move([pos[0], pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0], pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = -1.;
                while self.push_move([pos[0], pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0], pos[1] + offset], false) {
                        break;
                    }
                    offset -= 1.;
                }
            }
            Piece::Black(Type::Bishop) | Piece::White(Type::Bishop) => {
                let mut offset = 1.;
                while self.push_move([pos[0] + offset, pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] - offset, pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0] - offset, pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] + offset, pos[1] - offset], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1] - offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] - offset, pos[1] - offset], true, &mut v) {
                    if !self.can_move_to([pos[0] - offset, pos[1] - offset], false) {
                        break;
                    }
                    offset += 1.;
                }
            }
            Piece::Black(Type::Queen) | Piece::White(Type::Queen) => {
                let mut offset = 1.;
                while self.push_move([pos[0] + offset, pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] - offset, pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0] - offset, pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] + offset, pos[1] - offset], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1] - offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] - offset, pos[1] - offset], true, &mut v) {
                    if !self.can_move_to([pos[0] - offset, pos[1] - offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = 1.;
                while self.push_move([pos[0] + offset, pos[1]], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1]], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = -1.;
                while self.push_move([pos[0] + offset, pos[1]], true, &mut v) {
                    if !self.can_move_to([pos[0] + offset, pos[1]], false) {
                        break;
                    }
                    offset -= 1.;
                }
                offset = 1.;
                while self.push_move([pos[0], pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0], pos[1] + offset], false) {
                        break;
                    }
                    offset += 1.;
                }
                offset = -1.;
                while self.push_move([pos[0], pos[1] + offset], true, &mut v) {
                    if !self.can_move_to([pos[0], pos[1] + offset], false) {
                        break;
                    }
                    offset -= 1.;
                }
            }
            Piece::Black(Type::King) | Piece::White(Type::King) => {
                // TODO: Add logic stoping from king from making illegal moves <20-12-20, Shane McDonough>
                let mut new_pos = [pos[0] + 1., pos[1]];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0] + 1., pos[1] + 1.];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0], pos[1] + 1.];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0] - 1., pos[1] + 1.];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0] - 1., pos[1]];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0] - 1., pos[1] - 1.];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0], pos[1] - 1.];
                self.push_move(new_pos, true, &mut v);
                new_pos = [pos[0] + 1., pos[1] - 1.];
                self.push_move(new_pos, true, &mut v);
            }
            _ => (),
        };
        v
    }

    /// true if a point is outside of the chess board
    fn point_out_of_bounds(&mut self, pos: [f32; 2]) -> bool {
        pos[0] < 0. || pos[0] >= BOARD_SIZE as f32 || pos[1] < 0. || pos[1] >= BOARD_SIZE as f32
    }

    /// checks if a space is available to be inhabited
    fn can_move_to(&mut self, pos: [f32; 2], can_kill: bool) -> bool {
        if self.point_out_of_bounds(pos) {
            return false;
        }
        let piece = self.board[pos[1] as usize][pos[0] as usize];
        match piece {
            Piece::Empty => true,
            Piece::Black(_) => {
                if self.color == Color::Black {
                    false
                } else {
                    can_kill
                }
            }
            Piece::White(_) => {
                if self.color == Color::White {
                    false
                } else {
                    can_kill
                }
            }
        }
    }

    /// Checks if a new point can be moved to then pushed to a vector
    fn push_move(&mut self, new_pos: [f32; 2], can_kill: bool, v: &mut Vec<[f32; 2]>) -> bool {
        if self.can_move_to(new_pos, can_kill) {
            v.push(new_pos);
            return true;
        }
        false
    }

    /// move the piece in the position self.selected_pos to the argument pos
    fn move_selected_piece(&mut self, pos: [f32; 2]) -> bool {
        let s_pos = self.selected_pos.unwrap();
        let moves = self.get_valid_moves(self.selected_pos.unwrap());
        if moves.contains(&pos) {
            self.board[pos[1] as usize][pos[0] as usize] =
                self.board[s_pos[1] as usize][s_pos[0] as usize];
            self.board[s_pos[1] as usize][s_pos[0] as usize] = Piece::Empty;
            return true;
        }
        false
    }

    /// checks if the piece being clicked on is of the right team
    fn is_piece_selectable(&mut self, pos: [f32; 2]) -> bool {
        match self.board[pos[1] as usize][pos[0] as usize] {
            Piece::Black(_) => self.color == Color::Black,
            Piece::White(_) => self.color == Color::White,
            Piece::Empty => false,
            _ => unreachable!(),
        }
    }
}

impl event::EventHandler for State {
    /// The game logic function.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// when a mouse button is clicked down
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: input::mouse::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if button == input::mouse::MouseButton::Left {
            let pos = self.get_current_square(ctx);
            if self.selected_pos == None {
                if !self.is_piece_selectable(pos) {
                    return;
                }
                self.selected_pos = Some(pos);
            } else {
                if self.move_selected_piece(pos) {
                    self.color = match self.color {
                        Color::Black => Color::White,
                        Color::White => Color::Black,
                        _ => unreachable!(),
                    };
                }
                self.selected_pos = None;
            }
        }
    }

    /// the function that draws everything to the screen.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_board(ctx);
        self.draw_pieces(ctx);
        let current_square_pos = self.get_current_square(ctx);
        self.highlight_square(ctx, current_square_pos, [1., 1., 0., 0.3].into());
        if self.selected_pos != None {
            self.highlight_square(ctx, self.selected_pos.unwrap(), [1., 0., 0., 0.3].into());
            let moves = self.get_valid_moves(self.selected_pos.unwrap());
            for m in moves {
                self.highlight_square(ctx, m, [0., 1., 0., 0.3].into());
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

/// Driver function
fn main() -> GameResult {
    let (mut ctx, mut event_loop) =
        ggez::ContextBuilder::new("Chess", "Patrick and Shane McDonough")
            .window_setup(ggez::conf::WindowSetup {
                title: "Chess".to_owned(),
                samples: ggez::conf::NumSamples::Zero,
                vsync: true,
                icon: "".to_owned(),
                srgb: true,
            })
            .window_mode(ggez::conf::WindowMode {
                width: WINDOW_SIZE[0],
                height: WINDOW_SIZE[1],
                maximized: false,
                fullscreen_type: ggez::conf::FullscreenType::Windowed,
                borderless: false,
                min_width: 0.0,
                max_width: 0.0,
                min_height: 0.0,
                max_height: 0.0,
                resizable: false,
            })
            .build()?;
    let state = &mut State::new();
    event::run(&mut ctx, &mut event_loop, state)
}
