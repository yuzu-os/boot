#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;
mod yuzu;

use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::text::Color;
use uefi::proto::media::partition::PartitionInfo;
use uefi::table::boot::*;
use yuzu::runtime::*;

#[entry]
fn entry(image: Handle, system_table: SystemTable<Boot>) -> Status {
    boot(image, system_table).status()
}

fn boot(image: Handle, mut system_table: SystemTable<Boot>) -> uefi::Result {
    uefi_services::init(&mut system_table)?;

    let stdout = system_table.stdout();

    stdout.set_color(Color::Black, Color::Green)?;
    stdout.clear()?;

    uefi_services::println!("Entered UEFI boot services");
    uefi_services::println!("Welcome to yuzu 0.0.1");

    let revision      = system_table.firmware_revision();
    let uefi_revision = system_table.uefi_revision();

    uefi_services::println!("Firmware: {} {}.{} (UEFI {}.{})",
                            system_table.firmware_vendor(),
                            revision.major(), revision.minor(),
                            uefi_revision.major(), uefi_revision.minor());

    let boot_services = system_table.boot_services();

    let mut ctx = RuntimeContext::new();

    {
        let search = SearchType::from_proto::<GraphicsOutput>();
        let handle_buffer = boot_services.locate_handle_buffer(search)?;
        let handles = handle_buffer.handles();

        ctx.framebuffers.reserve_exact(handles.len());

        for handle in handles {
            let params = OpenProtocolParams {
                handle: *handle,
                agent: image,
                controller: None
            };
            let attribs = unsafe { core::mem::transmute(1) };
            let mut gop = unsafe {
                boot_services.open_protocol::<GraphicsOutput>(params, attribs)?
            };

            let modeinfo = gop.current_mode_info();
            let (width, height) = modeinfo.resolution();
            uefi_services::println!("Graphics mode: {}x{}", width, height);

            ctx.framebuffers.push(Framebuffer::from_gop(&mut gop));
        }
    }

    {
        let search = SearchType::from_proto::<PartitionInfo>();
        let result = boot_services.locate_handle_buffer(search);
        match result {
            Ok(handle_buffer) => {
                let handles = handle_buffer.handles();

                for handle in handles {
                    uefi_services::println!("found partition info");
                }
            },
            Err(ref err) => if err.status() != Status::NOT_FOUND { result?; }
        }
    }

    let mut buf = Vec::new();
    let mut status = Status::BUFFER_TOO_SMALL;

    while status == Status::BUFFER_TOO_SMALL {
        let MemoryMapSize { map_size, .. } = boot_services.memory_map_size();
        buf.resize(map_size, 0);
        status = boot_services.memory_map(&mut buf).status();
    }

    uefi::Result::from(status)?;
    uefi_services::println!("Successfully retrieved UEFI memory map");

    uefi_services::println!("Exiting UEFI boot services");
    system_table.exit_boot_services(image, &mut buf)?;

    runtime(ctx)
}

fn runtime(ctx: RuntimeContext) -> uefi::Result {
    //uefi_services::println!("Entered UEFI runtime services"); // invalid
    for fb in ctx.framebuffers {
        let (width, height) = fb.resolution;
        for y in 0..height {
            for x in 0..width {
                let index: isize = (y * width + x).try_into().unwrap();
                let cell_size = 64;
                unsafe {
                    *fb.data.offset(index * 4 + 2) = (x / cell_size * cell_size * 255 / width) as u8;
                    *fb.data.offset(index * 4 + 0) = (y / cell_size * cell_size * 255 / height) as u8;
                }
            }
        }
    }
    loop {}
}
