enum Color {
    Black,
    White,
}
enum Piece {
    Empty,
    Black(Type),
    White(Type),
}
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
fn main() {
    println!("Hello, world!");
}
