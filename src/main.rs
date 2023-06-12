#![allow(non_upper_case_globals)]
#![windows_subsystem = "windows"]

use std::io::Write;

use rand::Rng;
use windows::Win32::UI::WindowsAndMessaging::{LoadImageA, DrawIcon, HICON, IMAGE_FLAGS, GDI_IMAGE_TYPE, GetMessageA, TranslateMessage, DispatchMessageA, MSG, SHOW_WINDOW_CMD, EnumWindows, WINDOWS_HOOK_ID, CallNextHookEx, SetWindowsHookExA, GetWindowRect, GetCursorPos, SetCursorPos, SetWindowPos, ShowWindow, SET_WINDOW_POS_FLAGS, IsWindowVisible, GetWindowLongA, WINDOW_LONG_PTR_INDEX};
use windows::Win32::Foundation::{HMODULE, LPARAM, HWND, BOOL, RECT, POINT, WPARAM, LRESULT, GetLastError};
use windows::s;
use windows::Win32::Graphics::Gdi::{MonitorFromPoint, MONITOR_FROM_FLAGS, GetDC};
use windows::Win32::Media::Audio::{PlaySoundA, SND_FLAGS};
use windows::Win32::System::Console::AllocConsole;

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

fn beep() {

	unsafe {
		PlaySoundA(s!("DeviceDisconnect"), None, SND_FLAGS(65536) | SND_FLAGS(1));
	}
}

fn draw_random_icon() {

	std::thread::spawn(|| {
		unsafe {

			let mut rng = rand::thread_rng();

			let img = if rng.gen_ratio(1, 2) {
				LoadImageA(HMODULE::default(), s!("warning.ico"), GDI_IMAGE_TYPE(1), 0, 0, IMAGE_FLAGS(16)).unwrap()
			} else {
				LoadImageA(HMODULE::default(), s!("error.ico"), GDI_IMAGE_TYPE(1), 0, 0, IMAGE_FLAGS(16)).unwrap()
			};
			
			let dc = GetDC(HWND::default());

			let mut pos = POINT::default();
			GetCursorPos(&mut pos);

			let cursor_pos = (pos.x+rng.gen_range(-500..500), pos.y+rng.gen_range(-500..500));

			DrawIcon(dc, cursor_pos.0, cursor_pos.1, HICON(img.0));

		}
	});



}

fn main() {

	if cfg!(debug_assertions) {
		unsafe { AllocConsole(); }
	}

	let mut file = std::fs::File::create(".\\warning.ico").unwrap();
	file.write_all(include_bytes!(".\\warning.ico")).unwrap();
	drop(file);

	let mut file = std::fs::File::create(".\\error.ico").unwrap();
	file.write_all(include_bytes!(".\\error.ico")).unwrap();
	drop(file);

	let draw_thread = std::thread::spawn(|| {
		let mut rng = rand::thread_rng();
		std::thread::sleep(std::time::Duration::from_secs(1));
		loop {
			draw_random_icon();
			let mut range: (f64, f64) = (0.0, 0.0);

			range.0 = (1.0 / start_time.elapsed().as_secs_f64() * 1.5).max(0.0);
			range.1 = (2.5 / start_time.elapsed().as_secs_f64() * 1.5).max(0.15);
			// println!("{:?}, {}", range, start_time.elapsed().as_secs_f64());

			std::thread::sleep(std::time::Duration::from_secs_f64(rng.gen_range(range.0..range.1)));
		}
	});

	let mut last_beep = std::time::Instant::now();
	beep();

	unsafe {

		set_hook();

		let thread = std::thread::spawn(move || {

			let mut rng = rand::thread_rng();

			loop {
				let t = std::thread::spawn(|| {
					EnumWindows(Some(callback), LPARAM(0));
				});
	
				let mut pos = POINT::default();
				GetCursorPos(&mut pos);
	
				let cursor_range: i32 = ((50.0*start_time.elapsed().as_secs_f64()/100.0).ceil() as i32).min(50);
				SetCursorPos(pos.x+rng.gen_range(-cursor_range..=cursor_range), pos.y+rng.gen_range(-cursor_range..=cursor_range));

				log_error();

				if last_beep.elapsed().as_secs_f32() > 1.0 && rng.gen_ratio(1, 25) {
					beep();
					last_beep = std::time::Instant::now();
				}

				std::thread::sleep(std::time::Duration::from_millis(1));
				t.join().unwrap();
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
		draw_thread.join().unwrap();


	}

}
