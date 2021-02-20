#![no_std]

/// Entry point to the BIOS
#[repr(C)]
pub struct BiosApi {
	pub exec: extern "C" fn(&mut BiosArgs) -> i32,
}

/// Arguments for the BiosApi::exec call.
#[repr(C)]
pub enum BiosArgs<'a> {
	Print(&'a str),
	Exit,
}
