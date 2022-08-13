use screenshots::Screen;
use image::load_from_memory;
use show_image::create_window;
use quircs;
use quircs::Code;
use quircs::DecodeError;

#[show_image::main]
fn main() {
	let screens = Screen::all().unwrap();
	// Captire all screens
	for screen in screens {
	    let img = screen.capture().unwrap();
		println!("Took screenshot");
		let img = load_from_memory(img.buffer()).unwrap().into_luma8();

		let window = create_window("image", Default::default()).unwrap();
 		window.set_image("image-001", img.clone()).unwrap();

		let mut decoder = quircs::Quirc::default();
		let codes = decoder.identify(img.width() as usize, img.height() as usize, &img);
		for code in codes {
			match code {
				Ok(c) => decode_codes(c),
				Err(e) => println!("Error with code: {:?}", e),
			}
		}

	}
	println!("done");
	std::thread::sleep(std::time::Duration::new(60, 0))
}

fn decode_codes(code: Code) -> () {
	match code.decode() {
		Ok(res) => println!("{:?}", std::str::from_utf8(&res.payload).unwrap()),
		Err(e) => match e {
			DecodeError::DataUnderflow => println!("Found less bits than expected"),
			DecodeError::DataOverflow => println!("Found more bits than expected"),
			DecodeError::UnkownDataType => println!("Unknown data type"),
			DecodeError::DataEcc => println!("Code not complete, or corrupt"),
			DecodeError::InvalidVersion => println!("Unsupported code version"),
			DecodeError::InvalidGridSize => println!("Unsupported code size"),
			DecodeError::FormatEcc => println!("Format Error"),
		},
	}
}
