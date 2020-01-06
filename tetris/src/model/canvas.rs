// 前提是chess_map和chess_piece都在mod.rs里“注册过了”
use super::CanvasPixel;
use super::CanvasPiece;

#[derive(Debug)]
pub struct Canvas {
	pub pixels: Vec<Vec<CanvasPixel>>,
}