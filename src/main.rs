#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::console::text::Color;
use uefi::table::boot::*;

#[entry]
fn entry(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    main(handle, system_table).status()
}

fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> uefi::Result {
    uefi_services::init(&mut system_table)?;

    let stdout = system_table.stdout();

    stdout.set_color(Color::Black, Color::Green)?;
    stdout.clear()?;

    let revision      = system_table.firmware_revision();
    let uefi_revision = system_table.uefi_revision();

    uefi_services::println!("Welcome to yuzu 0.0.1");
    uefi_services::println!("Firmware: {} {}.{} (UEFI {}.{})",
                            system_table.firmware_vendor(),
                            revision.major(), revision.minor(),
                            uefi_revision.major(), uefi_revision.minor());

    let boot_services = system_table.boot_services();

    let mut status;
    let mut buf = Vec::new();

    while {
        let buf = &mut buf;

        let MemoryMapSize { map_size, .. } = boot_services.memory_map_size();

        buf.resize(map_size, 0);

        status = boot_services.memory_map(buf).status();
        status == Status::BUFFER_TOO_SMALL
    } {}

    uefi::Result::from(status)?;

    uefi_services::println!("Successfully retrieved UEFI memory map");
    uefi_services::println!("Exiting UEFI boot services");

    system_table.exit_boot_services(handle, &mut buf)?;

    loop {}
}
