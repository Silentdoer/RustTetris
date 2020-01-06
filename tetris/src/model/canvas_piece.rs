use std::default::Default;
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::model::{Canvas, CanvasPixel, ShowSelf, Coordinate};
use crate::transform_symbol;
use crate::enumerate::Color;
use crate::constant::base_constants;
//use super::ShowSelf;

// 如果全部用默认值，number类型0，bool类型false则可以直接用这个，不过这里有用到enum，所以没法这么做，要自己去实现Default trait
// 默认顺时针旋转
//#[derive(Default)]
pub struct CanvasPiece {
	/*顺时针旋转*/
	pub direction: PieceDirection,
	/*类似kotlin的``，type初始形状就和对应字母一样，且以初始形状左下角【主要是usize不好搞负数】为原点进行旋转，原点坐标为x y，原点在旋转过程中保持不变*/
	pub r#type: PieceType,
	/*所属map*/
	pub canvas: Option<Canvas>,
	/*Rust里下标的类型默认是usize，无符号和平台有关*/
	pub x: i32,
	pub y: i32,
	pub pixels: Vec<CanvasPixel>,
}

impl CanvasPiece {
	//  {r#type: PieceType::I, direction: PieceDirection::Up, x: 5, y: -1, canvas: Option::None }
	pub fn next(canvas: Option<Canvas>) -> Self {
		let piece_type = PieceType::next();
		let mut pixels = Vec::with_capacity(4) as Vec<CanvasPixel>;
		match piece_type {
			PieceType::I => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -3 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -4 });
			}
			PieceType::T => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X + 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
			}
			PieceType::Z => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X + 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -2 });
			}
			PieceType::S => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X + 1, y: -2 });
			}
			PieceType::O => {
				// O 不管，不能转也行，毕竟转了也是这样
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -2 });
			}
			PieceType::L => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X + 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -3 });
			}
			PieceType::J => {
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X - 1, y: -1 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -2 });
				pixels.push(CanvasPixel
					{ color: Color::Green, symbol: transform_symbol("□"), x: base_constants::INIT_COORDINATE_X, y: -3 });
			}
		}
		// 新的形状的方向永远是向上，这个没必要随机也不应该随机
		return CanvasPiece {
			direction: PieceDirection::Up,
			r#type: piece_type,
			x: base_constants::INIT_COORDINATE_X
			,
			y: base_constants::INIT_COORDINATE_Y,
			pixels,
			canvas,
		};
	}

	pub fn in_self(&self, pixel: &CanvasPixel) -> bool {
		// p属于piece中的某个pixel
		for p in self.pixels.iter() {
			if p.x == pixel.x && p.y == pixel.y {
				return true;
			}
		}
		false
	}

	/// 这里没有判断有其他方块挡住的情况，类似挡住马脚的情况，不过这个只是多写一些额外的坐标上的内容判断，对于属性rust没有太大帮助就懒得弄了
	pub fn can_rotate(&self) -> bool {
		// 对田型做特殊处理
		if self.y <= base_constants::INIT_COORDINATE_Y || self.r#type == PieceType::O {
			return false;
		}

		let canvas = self.canvas.as_ref().unwrap();
		for p in self.pixels.iter() {
			// 这里的坐标系y轴是反的，所以换成self.y - p.y
			let j = self.y - p.y + self.x;
			let k = p.x - self.x + self.y;
			if j <= 0 || j >= base_constants::CANVAS_WIDTH - 1 {
				return false;
			}
			if k >= base_constants::CANVAS_HEIGHT - 1 {
				return false;
			}
			if k >= 0 {
				// flag 如果k 不 as usize不但编译报错，还可能导致写代码时无法推断line的类型
				let line = canvas.pixels.get(k as usize).unwrap();
				if !self.in_self(line.get(j as usize).unwrap()) && line.get(j as usize).unwrap().symbol == transform_symbol("□") {
					return false;
				}
			}
		}
		return true;
	}

	// TODO 其实还需要个context类，同时包含canvas对象和curr_piece对象
	pub fn rotate(&mut self) {
		if self.can_rotate() {
			for p in self.pixels.iter() {
				// p.x一定大于0
				if p.y >= 0 {
					let line: &mut Vec<CanvasPixel> = self.canvas.as_mut().unwrap().pixels.get_mut(p.y as usize).unwrap();
					line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol(" "), x: p.x, y: p.y };
				}
			}

			for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
				// 这里的坐标系y轴是反的，所以换成self.y - p.y
				let j = self.y - p.y + self.x;
				let k = p.x - self.x + self.y;
				p.x = j;
				p.y = k;
				// j一定大于0
				if k >= 0 {
					let line: &mut Vec<CanvasPixel> = self.canvas.as_mut().unwrap().pixels.get_mut(k as usize).unwrap();
					line[j as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol("□"), x: j, y: k };
				}
			}
			self.canvas.as_ref().unwrap().show_self();
			self.direction = next_direction(&self.direction);
		}
	}

	pub fn can_dropdown(&self) -> bool {
		let canvas = self.canvas.as_ref().unwrap();
		for p in self.pixels.iter() {
			if p.y >= -1 && canvas.pixels.get((p.y + 1) as usize).unwrap().get(p.x as usize).unwrap().symbol == transform_symbol("□")
				&& !self.in_self(canvas.pixels.get((p.y + 1) as usize).unwrap().get(p.x as usize).unwrap()) {
				return false;
			}
		}
		true
	}

	/**
	* as_mut是用于获取可变的Option属性，get_mut是用于Vec获取可变的属性，而且get_mut或as_mut之后unwrap左侧变量定义可以省略mut关键字
	* Option或Result的as_mut是同时as_ref了的，即as_mut的类型是&mut T，而as_ref只是&T；而且标准库里的如Vec的get其实就是get出不可变的元素引用
	*/
	pub fn dropdown(&mut self) {
		if self.can_dropdown() {
			// flag 注意顺序，这句话如果在can_dropdown()上面会报错，因为先产生了mut引用，然后又产生非mut引用，而mut引用的生命周期还在所以可能出现修改非mut引用的情况
			let canvas = self.canvas.as_mut().unwrap();
			// Vec<Vec<Ac>>，这里Vec可以看成是struct，而它的每个元素是它的prop
			// flag 终于知道对于Vec as_mut()的作用是什么了，就是当元素是struct时可以对元素内部属性修改；
			for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
				if p.y >= 0 {
					let line: &mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
					line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol(" "), x: p.x, y: p.y };
				}
				p.y = p.y + 1;
			}
			for p in self.pixels.iter() {
				if p.y >= 0 {
					let line: &mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
					line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol("□"), x: p.x, y: p.y };
				}
			}
			self.y = self.y + 1;
			canvas.show_self();
		} else {
			// 当动不了的时候，只要piece存在一部分是小于 INIT_COORDINATE_Y就认为是输了
			for p in self.pixels.iter() {
				if p.y <= base_constants::INIT_COORDINATE_Y {
					let canvas = self.canvas.as_mut().unwrap();
					canvas.failure();
				}
			}

			self.success();


			let tmp = CanvasPiece::next(None);


			self.direction = tmp.direction;
			self.r#type = tmp.r#type;
			self.x = tmp.x;
			self.y = tmp.y;

			self.pixels = tmp.pixels;
		}
	}

	/// 判断是否可以消除并执行消除逻辑
	pub fn success(&mut self) {
		let canvas = self.canvas.as_mut().unwrap();
		self.pixels.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
		for p in self.pixels.iter() {
			let line = canvas.pixels.get_mut(p.y as usize).unwrap() as &mut Vec<CanvasPixel>;
			let mut flag = true;
			// 0 - 11的横向坐标
			for x in 1..line.len() - 1 {
				if line.get(x).unwrap().symbol == transform_symbol(" ") {
					flag = false;
					break;
				}
			}
			if flag {
				// 1 消除当行，将当行的上面的行都往下移动，这里有个问题，如果两行都被消除了，那么应该是先全部消除然后再下降，而且应该是从最上面的空行下降
				for x in 1..line.len() - 1 {
					line[x] = CanvasPixel { color: Color::Green, symbol: transform_symbol(" "), x: x as i32, y: p.y };
				}
				if p.y >= 1 {
					// 将上面的移下来（rust里for in .. 的in的值只能是从小到大，不能从大到小，fuck，自己用while实现吧
					let mut ln = p.y - 1;
					while ln >= 0 {
						let pixels = canvas.pixels.as_mut() as &mut Vec<Vec<CanvasPixel>>;
						// TODO 不用clone会提示不通过，因为rust无法确定vec.get两次获取的是否是同一个元素，因此它就是按同一个元素来处理的
						// TODO 对于同一个元素自然不能同时存在&和&mut（for循环里交错存在而不是先&完全用完然后才开始&mut）或两个&mut引用；
						let line = pixels.get(ln as usize).unwrap().clone();
						// &mut &Vec<CanvasPixel>
						let line2 = pixels.get_mut(ln as usize + 1).unwrap();
						for x in 1..line.len() - 1 {
							line2[x] = CanvasPixel { color: Color::Green, symbol: line.get(x).unwrap().symbol.clone(), x: x as i32, y: ln as i32 + 1 };
						}
						ln -= 1;
					}
				}
				canvas.show_self();
			}
		}
	}

	pub fn can_horizontal_move(&self, move_left: bool) -> bool {
		let canvas = self.canvas.as_ref().unwrap();
		if move_left {
			for p in self.pixels.iter() {
				if p.y >= 0 {
					let line = canvas.pixels.get(p.y as usize).unwrap();
					let pixel = line.get(p.x as usize - 1).unwrap();
					if !self.in_self(pixel) && pixel.symbol == transform_symbol("□") {
						return false;
					}
				}
				if p.x <= 1 {
					return false;
				}
			}
			return true;
		} else /*其他方向都认为是向右移动*/ {
			for p in self.pixels.iter() {
				if p.y >= 0 {
					let line = canvas.pixels.get(p.y as usize).unwrap();
					let pixel = line.get(p.x as usize + 1).unwrap();
					if !self.in_self(pixel) && pixel.symbol == transform_symbol("□") {
						return false;
					}
				}
				if p.x >= base_constants::CANVAS_WIDTH - 2 {
					return false;
				}
			}
			return true;
		}
	}

	pub fn horizontal_move(&mut self, move_left: bool) {
		if self.can_horizontal_move(move_left) {
			let canvas = self.canvas.as_mut().unwrap();
			for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
				if p.y >= 0 {
					let line: &mut Vec<CanvasPixel> = canvas.pixels.get_mut(p.y as usize).unwrap();
					line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol(" "), x: p.x, y: p.y };
				}
			}
			if move_left {
				for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
					p.x -= 1;
					if p.y >= 0 {
						let line: &mut Vec<CanvasPixel> = canvas.pixels.get_mut((p.y) as usize).unwrap();
						line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol("□"), x: p.x, y: p.y };
					}
				}
				self.x = self.x - 1;
			} else {
				for p in self.pixels.as_mut() as &mut Vec<CanvasPixel> {
					p.x += 1;
					if p.y >= 0 {
						let line: &mut Vec<CanvasPixel> = canvas.pixels.get_mut((p.y) as usize).unwrap();
						line[p.x as usize] = CanvasPixel { color: Color::Green, symbol: transform_symbol("□"), x: p.x, y: p.y };
					}
				}
				self.x = self.x + 1;
			}
			canvas.show_self();
		}
	}
}

fn next_direction(direction: &PieceDirection) -> PieceDirection {
	match direction {
		PieceDirection::Up => PieceDirection::Right,
		PieceDirection::Right => PieceDirection::Down,
		PieceDirection::Down => PieceDirection::Left,
		PieceDirection::Left => PieceDirection::Up,
	}
}

#[derive(Eq, PartialEq)]
pub enum PieceType {
	I = 1,
	O = 2,
	T = 3,
	J = 4,
	L = 5,
	S = 6,
	Z = 7,
}

impl PieceType {
	pub fn next() -> Self {
		return rand::random();
	}
}

impl Distribution<PieceType> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
		match rng.gen_range(1, 8) {
			1 => PieceType::I,
			2 => PieceType::O,
			3 => PieceType::T,
			4 => PieceType::J,
			5 => PieceType::L,
			6 => PieceType::S,
			7 => PieceType::Z,
			_ => PieceType::I,
		}
	}
}

/// Piece的方向
#[derive(Eq, PartialEq)]
pub enum PieceDirection {
	Up,
	Right,
	Down,
	Left,
}

impl Default for PieceDirection {
	fn default() -> Self { PieceDirection::Up }
}
