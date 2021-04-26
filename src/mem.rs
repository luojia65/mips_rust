// the implementation of memcpy, memcmp, memset 

#[inline(always)]
unsafe fn copy_forward(dest: *mut u8, src: *const u8, n: usize) {
	let mut i = 0;
	while i < n {
		*dest.add(i) = *src.add(i);
		i += 1;
	}
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) ->*mut u8 {
	copy_forward(dest, src, n);
	dest 
} 

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) ->i32 {
	let mut i = 0;
	while i < n {
		let a = *s1.add(i);
		let b = *s2.add(i);
		if a != b {
			return a as i32 - b as i32;
		}
		i += 1;
	}

	0
}

#[inline(always)]
pub unsafe fn set_bytes(s: *mut u8, c: u8, n: usize) {
	let mut i = 0;
	while i < n {
		*s.add(i) = c;
		i += i;
	}
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) ->*mut u8 {
	set_bytes(s, c as u8, n);
	s 
}
