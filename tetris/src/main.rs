mod model;
mod enumerate;
mod test;
mod constant;

use std::io::{stdout, Write};

use crate::model::{ShowSelf, Canvas, CanvasPixel, CanvasPiece, PieceType, PieceDirection};

use crate::constant::base_constants;

// TODO 自动将-转换成_
use tetris_common::platform::transform_symbol;

use crossterm::{
	cursor::MoveTo,
	event::{read, Event, KeyEvent, KeyModifiers, KeyCode, DisableMouseCapture, EnableMouseCapture},
	//screen::RawScreen,
	Result,
	execute,
	terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};

use crossterm::style::Colorize;
use crate::enumerate::Color;
use std::ops::Deref;
use std::thread::Thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::thread;

const HELP: &str = r#"按Ctrl+Q结束，按c然后按u开始游戏：
"#;

// 为什么不能晚点初始化，用啦lazy_static!感觉不知道怎么用。。
static mut CURR_PIECE: CanvasPiece = CanvasPiece {r#type: PieceType::I, direction: PieceDirection::Up, x: base_constants::INIT_COORDINATE_X, y: base_constants::INIT_COORDINATE_Y, canvas: Option::None, pixels: Vec::new() };

// Mutex::new(var)的var必须是不可变的值（即不能是mut的）；
lazy_static! {
	// 锁的创建其实可以写在主线程里
	static ref CURR_PIECE_LOCK: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

/*lazy_static!{
	static ref CURR_PIECE_LOCK: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}*/

/// 主线程
/*static mut MOCK_LOCK1: bool = true;

static mut MOCK_LOCK2: bool = false;*/

/*
* println!("{}", "██ ██☾☽██◀▶████▓▓██╲╱██◑◐██ღღ");
* println!("{}", "◁▷ ██  ☾☽██◁▶▉▉██╳╳><╱╲██♪♩▓▉");

// 注意，这个要直接运行exe文件，或者在外部的terminal运行可执行程序，不要在idea的terminal里运行
●○◇□◆■☆★☀☼♫♬♣♡♤♢✿❀☂☯
*/
fn print_events() -> Result<()> {

	let mut canvas = Canvas { pixels: Vec::with_capacity(22), };
	// 20行
	for i in 0..canvas.pixels.capacity() {
        // 10列
        let mut y = Vec::with_capacity(12) as Vec<CanvasPixel>;
        for j in 0..y.capacity() {
			// 在Linux里水平方向占两个位置的符号：＊〄我〠【不是，貌似跟终端有关，不同的终端貌似显示效果也不同。。，但是用了ubuntu和deepin的都差不多，
			// deepin的终端还更坑，还是用ubuntu自带的吧，而且要使用文件上说明用的是什么终端，最好版本也带上。。】
			if i == canvas.pixels.capacity() - 1 {
				y.push(CanvasPixel { color: Color::Red, symbol: transform_symbol("□"), x: j as i32, y: i as i32 });
			} else if j == 0 || j == y.capacity() - 1 {
				y.push(CanvasPixel { color: Color::Red, symbol: transform_symbol("□"), x: j as i32, y: i as i32 });
			} else {
				y.push(CanvasPixel { color: Color::Green, symbol: transform_symbol(" "), x: j as i32, y: i as i32 });
			}
        }
		canvas.pixels.push(y);
	}

	//region
	// rust里的锁和java里的很不一样，比较蛋疼，它的Mutex只能是在单线程下用，如果是多线程下用要Arc包一层，而且需要加锁的区域是一个对象
	// 这一部分一开始没看懂，后来想了下其实就是lock(obj){}而已，只不过这个obj可以是this，也可以是一个外部对象
	//let curr_piece = Arc::new(Mutex::new(CanvasPiece::next(Some(canvas))));
	//endregion

	unsafe {
		CURR_PIECE = CanvasPiece::next(Some(canvas));
	}
	let canvas: &Canvas;
	unsafe {
		canvas = CURR_PIECE.canvas.as_ref().unwrap();
	}

	let mut game_started = false;
	let lock_clone = CURR_PIECE_LOCK.clone();
	loop {
		// Blocking read, is a Event type
		// 要换成异步读取
		let event = read()?;

		// 没用的锁，想找个java里的两个线程之间可以互相唤醒的半天每找到，obj.wait(),obj.notifyAll()
		//let lock_clone = Arc::clone(&*CURR_PIECE_LOCK);

		// 一读取到数据就加锁，防止出现问题，这里unwrap()获得的是锁这个对象本身（即0这个数值）
		// 这里应该是每一轮loop结束都会释放锁吧？
		//let _ = lock_clone.lock();

		//println!("Event::{:?}\r", event);

		// 这里还是会有并发问题，毕竟在判断lock2和设置lock1的过程中可能子线程正好在判断发现lock1是false，不过概率已经很小了
		// 主要是rust的锁真几把操蛋。
		/*unsafe {
			// 子线程拿到了锁
			while MOCK_LOCK2 {
				//
			}
			// 主线程拿到锁
			MOCK_LOCK1 = true;
		}*/

		// guard必须存在，它销毁了则锁释放
		let guard = lock_clone.lock().unwrap();

		if event == Event::Key(KeyCode::Char('c').into()) {
			clear();
			let _ = goto(0, 0);
		}

		if event == Event::Key(KeyCode::Char('p').into()) {
			println!("{}", "□〠".len());
		}

		// 前景色和背景色
		if event == Event::Key(KeyCode::Char('f').into()) {
			// TODO concat!宏可以实现编译期间将两个&str连接为一个新的&str，且是'static
			println!("{}", concat!("red foreground color\r", "***concat***").red());
			println!("{}", "white foreground color\r".white());
			println!("{}", "green background color\r".on_green());
			println!("{}", "white background color\r".on_white());
		}

		if event == Event::Key(KeyCode::Char('m').into()) {
			let _ = goto(0, 0);
		}

		if event == Event::Key(KeyCode::Char('y').into()) {
			println!("{}", "▉▉▉▉▓▓▓▓▓▓██▉▉☾☽██◀▶██████╲╱██◑◐██ღღ88");
			println!("{}", "◇◆●○■□██◁▶▉▉██╳╳><╱╲██♪♩▓▉88");
		}

		if event == Event::Key(KeyCode::Char('z').into()) {
			unsafe {
				let string = format!("{}{}", "\r", CURR_PIECE.y);
				print!("{}", transform_symbol(string.deref()));
			}
			flush_output();
		}

		if event == Event::Key(KeyCode::Up.into()) {
			unsafe {
				// 不用unsafe会提示curr_piece可能被并发修改，但是我确实只用了一个线程因此unsafe【还有个问题，为什么用drop方法会提示必须用mut定义？】
				CURR_PIECE.rotate();
			}
			print!("{}", transform_symbol("\r↑"));
			flush_output();
		}

		if event == Event::Key(KeyCode::Right.into()) {
			unsafe {
				// 不用unsafe会提示curr_piece可能被并发修改，但是我确实只用了一个线程因此unsafe【还有个问题，为什么用drop方法会提示必须用mut定义？】
				CURR_PIECE.horizontal_move(false);
			}
			print!("{}", transform_symbol("\r→"));
			flush_output();
		}

		if event == Event::Key(KeyCode::Down.into()) {
			unsafe {
				// 不用unsafe会提示curr_piece可能被并发修改，但是我确实只用了一个线程因此unsafe【还有个问题，为什么用drop方法会提示必须用mut定义？】
				CURR_PIECE.dropdown();
			}
			print!("{}", transform_symbol("\r↓"));
			flush_output();
		}

		if event == Event::Key(KeyCode::Left.into()) {
			unsafe {
				// 不用unsafe会提示curr_piece可能被并发修改，但是我确实只用了一个线程因此unsafe【还有个问题，为什么用drop方法会提示必须用mut定义？】
				CURR_PIECE.horizontal_move(true);
			}
			print!("{}", transform_symbol("\r←"));
			flush_output();
		}

		// 按下u后开始游戏
		if event == Event::Key(KeyCode::Char('u').into()) {
			if game_started {
				continue;
			}
			game_started = true;
			canvas.show_self();
			let lock_clone = CURR_PIECE_LOCK.clone();
			std::thread::spawn(move || {
				// 第一次自动下落等1.2秒
				//std::thread::sleep(Duration::from_millis(1200));
				loop {
					// 每1.5秒下落一次
					std::thread::sleep(Duration::from_millis(1500));
					let guard = lock_clone.lock().unwrap();
					/*unsafe {
						// 主线程获得了锁
						if MOCK_LOCK1 {
							continue;
						}
						MOCK_LOCK2 = true;
					}*/

					unsafe {
						CURR_PIECE.dropdown();
						// 逻辑执行完毕，释放锁
						//MOCK_LOCK2 = false;
					}
				}
			});
			//let _ = goto(0, 0);
		}

		/*if game_start {
			unsafe {
				//Thread::
				CURR_PIECE.dropdown();
			}
		}*/

		if event == Event::Key(KeyCode::Char('e').into()) {
			let piece = model::CanvasPixel { color: Color::Red, symbol: "★".to_owned(), x: 5, y: 5 };
			// 这里有个bug，第一次的时候不知道为什么不会输出，必须在后面加个println!("{}", "");才能够把第一次的给输出
			// 这个bug可能不是标准库里的，而是crossterm里的
			// 这里通过主动flush一下可以解决这个bug
			piece.show_self();
			flush_output();
		}

		if event == Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL)) {
			println!("game quit!\r");
			break;
		}

		/*unsafe {
			// 主线程释放”锁“
			MOCK_LOCK1 = false;
		}*/
	}

	Ok(())
}

fn goto(x: u16, y: u16) -> Result<()> {
	// Set the cursor to position X: 10, Y: 5 in the terminal
	execute!(stdout(), MoveTo(x, y))?;
	Ok(())
}

fn flush_output() {
	let _ = std::io::stdout().flush();
}

fn clear() {
	let _ = execute!(stdout(), Clear(ClearType::All));
}

// 这个Result但是crossterm里对std::result::Result的re-export，具体为：pub type Result<T> = std::result::Result<T, ErrorKind>;
fn main() -> Result<()> {
	println!("{}", HELP);

	// 可以防止Ctrl + C退出，而且这里还不能用_忽略返回值，但是又确实没有哪个地方用到了返回值，好神奇；
	// 如果说不需要忽略Ctrl+C而退出程序，那么这句话可以不加
	// 0.14.0开始不需要这个也是能防止Ctrl+C退出，而且0.14.0没有screen这个包了
	//let _r = RawScreen::into_raw_mode()?;
	enable_raw_mode()?;

	let mut stdout = stdout();
	execute!(stdout, EnableMouseCapture)?;

	if let Err(e) = print_events() {
		println!("Error: {:?}\r", e);
	}

	execute!(stdout, DisableMouseCapture)?;

	disable_raw_mode()
}
