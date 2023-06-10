extern crate pretty_env_logger;
#[macro_use] extern crate log;

use rand::Rng;
use windows::Win32::UI::WindowsAndMessaging::{SHOW_WINDOW_CMD, EnumWindows, GetWindowRect, GetClassNameA, SetWindowPos, ShowWindow, SET_WINDOW_POS_FLAGS, IsWindowVisible, GetWindowLongA, WINDOW_LONG_PTR_INDEX};
use windows::Win32::Foundation::{LPARAM, HWND, BOOL, RECT};

unsafe extern "system" fn callback(handle: HWND, _: LPARAM) -> BOOL {

	// let mut buffer = Vec::with_capacity(100 as usize);

	// let mut amount: u32 = 0;

	// unsafe { GetUserNameA(PSTR(std::ptr::null_mut()), &mut amount) };

	// unsafe { GetUserNameA(PSTR(buffer.as_mut_ptr() as *mut u8), &mut amount) };

	// unsafe { buffer.set_len(amount as usize); }
	// let username = String::from_utf8(buffer).unwrap();

	let mut buffer = [0 as u8; 100];
	GetClassNameA(handle, &mut buffer);
	let name = String::from_utf8(buffer.to_vec()).unwrap();
	
	let mut rng = rand::thread_rng();

	let style = GetWindowLongA(handle, WINDOW_LONG_PTR_INDEX(-16));
	if (style & 0x00000000 == 0) && ((style & 0x00800000 == 0x00800000) || (style & 0x00C00000 == 0x00C00000)) {
		info!("Window handle: {}", name);
		if IsWindowVisible(handle).into() {
			ShowWindow(handle, SHOW_WINDOW_CMD(4));
		}
		
		let mut r = RECT::default();
		GetWindowRect(handle, &mut r);

		SetWindowPos(handle, None, r.left+rng.gen_range(-20..20), r.top+rng.gen_range(-5..5), 5, 5, SET_WINDOW_POS_FLAGS(1));
	}

	return BOOL(true as i32);
}

fn main() {

	pretty_env_logger::init();
	
	loop {
		unsafe {

			EnumWindows(Some(callback), LPARAM(0));
			// std::thread::sleep(std::time::Duration::from_millis(10));

		}
	}


}
