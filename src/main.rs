use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SIZE: [f32; 2] = [600., 600.];
const BOARD_SIZE: usize = 8;

enum Color {
    Black,
    White,
}
#[derive(Copy, Clone)]
enum Piece {
    Empty,
    Black(Type),
    White(Type),
}
#[derive(Copy, Clone)]
enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
struct State {
    board: [[Piece; BOARD_SIZE]; BOARD_SIZE],
    color: Color,
}
impl State {
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
        }
    }
    fn draw_board(&mut self, ctx: &mut Context) {
        let square_size: [f32; 2] = [SIZE[0] / BOARD_SIZE as f32, SIZE[1] / BOARD_SIZE as f32];
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 0.,
                y: 0.,
                w: square_size[0],
                h: square_size[1],
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
                        j as f32 * square_size[0] * 2.,
                        i as f32 * square_size[1] * 2.,
                    ),),
                )
                .unwrap();
                graphics::draw(
                    ctx,
                    &square,
                    (na::Point2::new(
                        j as f32 * square_size[0] * 2. + square_size[0],
                        i as f32 * square_size[1] * 2. + square_size[1],
                    ),),
                )
                .unwrap();
            }
        }
    }
    fn draw_piece(&mut self, ctx: &mut Context, pos: [f32; 2]) {
        let piece = self.board[pos[1] as usize][pos[0] as usize];
        let square_size: [f32; 2] = [SIZE[0] / BOARD_SIZE as f32, SIZE[1] / BOARD_SIZE as f32];
        let color: graphics::Color;
        match piece {
            Piece::Empty => return,
            Piece::Black(_) => color = graphics::BLACK,
            Piece::White(_) => color = graphics::WHITE,
            _ => unreachable!(),
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [square_size[0] / 2., square_size[1] / 2.],
            square_size[0] * 0.4,
            0.1,
            color,
        )
        .unwrap();
        let border = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.),
            [square_size[0] / 2., square_size[1] / 2.],
            square_size[0] * 0.4,
            0.1,
            [0.5, 0.5, 0.5, 1.0].into(),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &circle,
            (na::Point2::new(
                pos[0] * square_size[0],
                pos[1] * square_size[1],
            ),),
        )
        .unwrap();
        graphics::draw(
            ctx,
            &border,
            (na::Point2::new(
                pos[0] * square_size[0],
                pos[1] * square_size[1],
            ),),
        )
        .unwrap();
    }
}
impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_board(ctx);
        self.draw_piece(ctx, [1., 0.]);
        graphics::present(ctx)?;
        Ok(())
    }
}
fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup {
            title: "Chess".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
            width: SIZE[0],
            height: SIZE[1],
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
