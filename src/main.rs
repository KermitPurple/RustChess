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
fn main() {
    println!("Hello, world!");
}
