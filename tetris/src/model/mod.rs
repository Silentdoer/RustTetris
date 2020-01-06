// 这里不要pub，然后再后面用pub use进行re-export，就能令外部只能model::CanvasPiece而不能model::canvas_piece::CanvasPiece
mod canvas_pixel;
mod canvas;
mod canvas_piece;
mod coordinate;

pub use canvas_pixel::CanvasPixel;
pub use canvas::Canvas;
pub use canvas_piece::{CanvasPiece, PieceType, PieceDirection};
pub use coordinate::Coordinate;

use crate::enumerate::Color;
use crossterm::style;
use crossterm::style::{style};
use crate::{goto, clear};
use std::process::exit;

pub trait Colorable {
	fn color(&self) -> &Color;
}

impl Colorable for CanvasPixel {
	fn color(&self) -> &Color {
		&self.color
	}
}

pub trait ShowSelf {
	fn show_self(&self);
}

impl ShowSelf for CanvasPixel {

	fn show_self(&self) {
		match &self.color() {
			Color::Red => print!("{}", style(format!("{}", self.symbol)).with(style::Color::Red)),
			Color::White => print!("{}", style(format!("{}", self.symbol)).with(style::Color::White)),
			Color::Green => print!("{}", style(format!("{}", self.symbol)).with(style::Color::Green)),
		}
	}

	/*fn show_self_with_coordinate(&self) {

	}*/
}

impl ShowSelf for Canvas {

	fn show_self(&self) {
		let _ = goto(0, 0);
		for i in self.pixels.iter() {
			for j in i.iter() {
				j.show_self();
			}
			println!("\r");
		}
	}
}

/// Canvas目前充当游戏的Context类
impl Canvas {
	pub fn failure(&self) {
		clear();
		let _ = goto(0, 0);
		println!("游戏结束!\r");
		exit(1);
	}
}
