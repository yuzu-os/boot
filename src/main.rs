#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use core::fmt::Write;
use uefi::prelude::*;
use uefi::proto::console::text::Color;

#[entry]
fn entry(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    main(handle, &mut system_table).status()
}

fn main(_handle: Handle, system_table: &mut SystemTable<Boot>) -> uefi::Result {
    uefi_services::init(system_table)?;

    let stdout = system_table.stdout();

    stdout.set_color(Color::Black, Color::Green)?;
    stdout.clear()?;
    write!(stdout, "Welcome to yuzu 0.0.1\n").unwrap();
    write!(stdout, "Firmware vendor: ").unwrap();

    uefi_services::println!("Firmware vendor: {}", system_table.firmware_vendor());
    //uefi_services::println!("Firmware version: {}.{}", system_table.firmware_revision());

    /*{
        let vendor = system_table.firmware_vendor();
        stdout.output_string(vendor)?;
    }*/

    loop {}
}
