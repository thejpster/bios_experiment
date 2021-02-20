#![no_std]

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ErrorCode {
	Success,
	BadArgument,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BiosStrSlice<'a> {
	pub ptr: *const u8,
	pub length: usize,
	_foo: core::marker::PhantomData<&'a str>,
}	

impl<'a> BiosStrSlice<'a> {
	pub fn to_str(&'a self) -> &'a str {
		let byte_slice = unsafe { core::slice::from_raw_parts(self.ptr, self.length) };
		unsafe { core::str::from_utf8_unchecked(&byte_slice) }
	}

	pub fn from_str(s: &'a str) -> BiosStrSlice<'a> {
		BiosStrSlice {
			ptr: s.as_ptr(),
			length: s.len(),
			_foo: core::marker::PhantomData,
		}
	}
}

/// Tells you about a serial port
#[repr(C)]
pub struct SerialInfo {
	name: [u8; 32],
	baud: u32,
}

/// Arguments for the BiosApi::exec call.
#[repr(C)]
pub enum BiosArgs<'a> {
	Print(BiosStrSlice<'a>),
	SerialGetInfo {
		idx: u8,
		out: &'a mut SerialInfo
	},
	Exit,
}

/// Entry point to the BIOS
#[repr(C)]
pub struct BiosApi {
	pub exec: extern "C" fn(&mut BiosArgs) -> ErrorCode,
}

