use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SIZE: [f32; 2] = [600., 600.];

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
    board: [[Piece; 8]; 8],
    color: Color,
}
impl State {
    fn new() -> Self {
        Self {
            board: [
                [Piece::White(Type::Rook), Piece::White(Type::Knight), Piece::White(Type::Bishop), Piece::White(Type::Queen), Piece::White(Type::King), Piece::White(Type::Bishop), Piece::White(Type::Knight), Piece::White(Type::Rook),],
                [Piece::White(Type::Pawn); 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Empty; 8],
                [Piece::Black(Type::Pawn); 8],
                [Piece::Black(Type::Rook), Piece::Black(Type::Knight), Piece::Black(Type::Bishop), Piece::Black(Type::Queen), Piece::Black(Type::King), Piece::Black(Type::Bishop), Piece::Black(Type::Knight), Piece::Black(Type::Rook),],
            ],
            color: Color::Black,
        }
    }
    fn draw_board(&mut self, ctx: &mut Context) {
        let square_size: [f32; 2] = [SIZE[0] / 8., SIZE[1] / 8.];
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
}
impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_board(ctx);
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
