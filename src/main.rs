use rand::Rng;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageA, TranslateMessage, DispatchMessageA, MSG, SHOW_WINDOW_CMD, EnumWindows, WINDOWS_HOOK_ID, CallNextHookEx, SetWindowsHookExA, GetWindowRect, GetCursorPos, SetCursorPos, SetWindowPos, ShowWindow, SET_WINDOW_POS_FLAGS, IsWindowVisible, GetWindowLongA, WINDOW_LONG_PTR_INDEX};
use windows::Win32::Foundation::{LPARAM, HWND, BOOL, RECT, POINT, WPARAM, LRESULT, GetLastError};
use windows::Win32::Graphics::Gdi::{MonitorFromPoint, MONITOR_FROM_FLAGS};

static start_time: once_cell::sync::Lazy<std::time::Instant> = once_cell::sync::Lazy::new(|| std::time::Instant::now());

unsafe fn is_normal_window(handle: HWND) -> bool {

	let style = GetWindowLongA(handle, WINDOW_LONG_PTR_INDEX(-16));
	if (style & 0x00000000 == 0) && ((style & 0x00800000 == 0x00800000) || (style & 0x00C00000 == 0x00C00000)) {
		return true;
	}

	return false;

}

unsafe extern "system" fn callback(handle: HWND, _: LPARAM) -> BOOL {

	// let mut buffer = Vec::with_capacity(100 as usize);

	// let mut amount: u32 = 0;

	// unsafe { GetUserNameA(PSTR(std::ptr::null_mut()), &mut amount) };

	// unsafe { GetUserNameA(PSTR(buffer.as_mut_ptr() as *mut u8), &mut amount) };

	// unsafe { buffer.set_len(amount as usize); }
	// let username = String::from_utf8(buffer).unwrap();

	// let mut buffer = [0 as u8; 100];
	// GetClassNameA(handle, &mut buffer);
	// let name = String::from_utf8(buffer.to_vec()).unwrap();

	let mut rng = rand::thread_rng();
	let window_range: i32 = ((20.0*start_time.elapsed().as_secs_f64()/100.0).ceil() as i32).min(20);

	if is_normal_window(handle) {
		// info!("Window handle: {}", name);
		if IsWindowVisible(handle).into() {
			ShowWindow(handle, SHOW_WINDOW_CMD(4));
		}
		
		let mut r = RECT::default();
		GetWindowRect(handle, &mut r);

		// info!("{}, {}", rng.gen_range(-window_range..window_range), window_range);

		SetWindowPos(handle, None, r.left+rng.gen_range(-window_range..=window_range), r.top+rng.gen_range(-window_range..=window_range), 5, 5, SET_WINDOW_POS_FLAGS(1));
	}

	return BOOL(true as i32);
	
}

unsafe extern "system" fn move_window_callback(handle: HWND, _: LPARAM) -> BOOL {

	if is_normal_window(handle) {
		if IsWindowVisible(handle).into() {
		

			let mut pos = POINT::default();
			GetCursorPos(&mut pos);

			let mut r = RECT::default();
			GetWindowRect(handle, &mut r);

			if pos.x > r.left && pos.y > r.top && pos.x < r.right && pos.y < r.bottom {
				// let mut buffer = [0 as u8; 100];
				// GetClassNameA(handle, &mut buffer);
	
				// let name = String::from_utf8(buffer.to_vec()).unwrap();
				// println!("{:?}", name);
				// return BOOL(false as i32);

				let mut rng = rand::thread_rng();

				let window_range: (i32, i32) = ((r.right-r.left)/4, (r.bottom-r.top)/4);

				loop {
					SetWindowPos(handle, None, r.left+rng.gen_range(-window_range.0..=window_range.0), r.top+rng.gen_range(-window_range.1..=window_range.1), 1, 1, SET_WINDOW_POS_FLAGS(1));
				
					GetWindowRect(handle, &mut r);
	
					let monitor = MonitorFromPoint(POINT { x: r.left, y: r.top }, MONITOR_FROM_FLAGS(0));
					if monitor.is_invalid() {
						continue;
					} else {
						break;
					}
				}


				// println!("{}, {}", r.left, r.top);


				return BOOL(false as i32);

			}

		}
	}

	return BOOL(true as i32);

}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {

	if code < 0 {
		return CallNextHookEx(None, code, wparam, lparam);
	}

	std::thread::spawn(move || {

		if wparam.0 == 0x0201 || wparam.0 == 0x0204 {

			EnumWindows(Some(move_window_callback), LPARAM(0));

		}

	});
	return CallNextHookEx(None, code, wparam, lparam);
}

fn set_hook() {
	unsafe {
		SetWindowsHookExA(WINDOWS_HOOK_ID(14), Some(mouse_hook), None, 0).unwrap();
	}
}

fn log_error() {
	let e = unsafe { GetLastError() };

	if e.0 != 0 {
		println!("ERROR CODE: {}", e.0);
	}
}

fn main() {

	unsafe {

		set_hook();

		let thread = std::thread::spawn(|| {

			let mut rng = rand::thread_rng();
	
			loop {
				EnumWindows(Some(callback), LPARAM(0));
	
				let mut pos = POINT::default();
				GetCursorPos(&mut pos);
	
				let cursor_range: i32 = ((50.0*start_time.elapsed().as_secs_f64()/100.0).ceil() as i32).min(50);
				SetCursorPos(pos.x+rng.gen_range(-cursor_range..=cursor_range), pos.y+rng.gen_range(-cursor_range..=cursor_range));
	
				log_error();

				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		});

		let mut msg = MSG::default();
		loop {

			let ret: i32 = GetMessageA(&mut msg, None, 0, 0).0;

			if ret == 0 || ret == -1 {
				break;
			}

			TranslateMessage(&msg);
			DispatchMessageA(&msg);

			log_error();

		}

		thread.join().unwrap();


	}




}
