use bios_common::{BiosApi, BiosArgs};

fn main() {
    println!("BIOS starting. Loading OS...");
    let mut bd = BiosApi { exec };
    call_dynamic(&mut bd).expect("Failed to load OS");
}

extern "C" fn exec(args: &mut BiosArgs) -> i32 {
    match args {
        BiosArgs::Print(s) => println!("{}", s),
        BiosArgs::Exit => std::process::exit(0),
    }
    0
}

fn call_dynamic(bios_api: &BiosApi) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./liboperatingsystem.so")?;
        let func: libloading::Symbol<unsafe extern "C" fn(&BiosApi) -> u32> =
            lib.get(b"entry_point")?;
        func(bios_api);
        Ok(())
    }
}
