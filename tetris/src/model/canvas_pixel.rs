use crate::enumerate::Color;

// 如果是pub(super)表示只有上一级可见，但是上一级不能对这个类进行pub的re-export
#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
pub struct CanvasPixel {
	pub color: Color,
	pub symbol: String,
	pub x: i32,
	pub y: i32,
}
