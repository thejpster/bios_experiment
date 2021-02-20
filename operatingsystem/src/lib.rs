#![no_std]

use core::panic::PanicInfo;

use bios_common::{BiosApi, BiosArgs};

#[no_mangle]
pub extern "C" fn entry_point(bios: &BiosApi)
{
	let mut m = BiosArgs::Print("Hello, world!\n");
	(bios.exec)(&mut m);

	let mut m = BiosArgs::Exit;
	(bios.exec)(&mut m);
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
