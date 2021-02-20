use bios_common;

fn main() {
    println!("BIOS starting. Loading OS...");
    let mut bd = bios_common::BiosApi { exec };
    call_dynamic(&mut bd).expect("Failed to load OS");
}

extern "C" fn exec(args: &mut bios_common::BiosArgs) -> bios_common::ErrorCode {
    match args {
        bios_common::BiosArgs::Print(s) => println!("{}", s.to_str()),
        bios_common::BiosArgs::Exit => std::process::exit(0),
        _ => {
            println!("Bad API call!")
        }
    }
    bios_common::ErrorCode::Success
}

fn call_dynamic(bios_api: &bios_common::BiosApi) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./liboperatingsystem.so")?;
        let func: libloading::Symbol<unsafe extern "C" fn(&bios_common::BiosApi) -> u32> =
            lib.get(b"entry_point")?;
        func(bios_api);
        Ok(())
    }
}
