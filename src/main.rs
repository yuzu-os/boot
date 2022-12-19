#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;
mod yuzu;

use alloc::vec::Vec;
use core::arch::asm;
use uefi::Identify;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::text::Color;
use uefi::proto::device_path::*;
use uefi::table::boot::*;
use yuzu::runtime::*;

struct DisplayDevicePathNode<'a>(&'a DevicePathNode);

impl core::fmt::Display for DisplayDevicePathNode<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self.0.full_type() {
            (DeviceType::HARDWARE,  DeviceSubType::HARDWARE_PCI)                  => "PCI",
            (DeviceType::HARDWARE,  DeviceSubType::HARDWARE_PCCARD)               => "PCCard",
            (DeviceType::HARDWARE,  DeviceSubType::HARDWARE_MEMORY_MAPPED)        => "MemoryMapped",
            (DeviceType::HARDWARE,  DeviceSubType::HARDWARE_VENDOR)               => "(Vendor)",
            (DeviceType::HARDWARE,  DeviceSubType::HARDWARE_CONTROLLER)           => "Controller",
            (DeviceType::ACPI,      DeviceSubType::ACPI)                          => "ACPI",
            (DeviceType::ACPI,      DeviceSubType::ACPI_EXPANDED)                 => "ExpandedACPI",
            (DeviceType::ACPI,      DeviceSubType::ACPI_ADR)                      => "ADR",
            (DeviceType::ACPI,      DeviceSubType::ACPI_NVDIMM)                   => "NVDIMM",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_ATAPI)               => "ATAPI",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SCSI)                => "SCSI",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_FIBRE_CHANNEL)       => "FibreChannel",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_1394)                => "FireWire",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB)                 => "USB",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_I2O)                 => "I2O",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_INFINIBAND)          => "InfiniBand",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_VENDOR)              => "(Vendor)",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_MAC_ADDRESS)         => "MACAddress",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_IPV4)                => "IPv4",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_IPV6)                => "IPv6",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_UART)                => "UART",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB_CLASS)           => "USBClass",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB_WWID)            => "USBWWID",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_DEVICE_LOGICAL_UNIT) => "LogicalUnit",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SATA)                => "SATA",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_ISCSI)               => "iSCSI",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_VLAN)                => "VLAN",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_FIBRE_CHANNEL_EX)    => "FibreChannelEx",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SCSI_SAS_EX)         => "SCSISASEx",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_NVME_NAMESPACE)      => "NVMENamespace",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_URI)                 => "URI",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_UFS)                 => "UFS",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SD)                  => "SD",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_BLUETOOTH)           => "Bluetooth",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_WIFI)                => "WiFi",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_EMMC)                => "eMMC",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_BLUETOOTH_LE)        => "BluetoothLE",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_DNS)                 => "DNS",
            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_NVDIMM_NAMESPACE)    => "NVDIMMNamespace",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_HARD_DRIVE)              => "HardDrive",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_CD_ROM)                  => "CDROM",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_VENDOR)                  => "(Vendor)",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_FILE_PATH)               => "FilePath",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_PROTOCOL)                => "MediaProtocol",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_PIWG_FIRMWARE_FILE)      => "PIWGFWFile",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_PIWG_FIRMWARE_VOLUME)    => "PIWGFWVolume",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_RELATIVE_OFFSET_RANGE)   => "RelativeOffsetRange",
            (DeviceType::MEDIA,     DeviceSubType::MEDIA_RAM_DISK)                => "RAMDisk",
            (DeviceType::BIOS_BOOT_SPEC, DeviceSubType::BIOS_BOOT_SPECIFICATION)  => "BIOSBootSpec",
            _ => "Unknown"
        })
    }
}

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
        let search = SearchType::ByProtocol(&DevicePath::GUID);
        let result = boot_services.locate_handle_buffer(search);
        match result {
            Ok(handle_buffer) => {
                let handles = handle_buffer.handles();

                for handle in handles {
                    let params = OpenProtocolParams {
                        handle: *handle,
                        agent: image,
                        controller: None
                    };
                    let attribs = unsafe { core::mem::transmute(1) };
                    let path = unsafe {
                        boot_services.open_protocol::<DevicePath>(params, attribs)?
                    };

                    for instance in path.instance_iter() {
                        uefi_services::print!("Device path: ");
                        for node in instance.node_iter() {
                            uefi_services::print!("::{}", DisplayDevicePathNode(node));
                        }
                        uefi_services::println!("");
                    }
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
                let cell_size = 64;
                unsafe {
                    *fb.pixel_offset(x, y).offset(2) = (x / cell_size * cell_size * 255 / width) as u8;
                    *fb.pixel_offset(x, y).offset(0) = (y / cell_size * cell_size * 255 / height) as u8;
                }
            }
        }
    }
    loop {
        unsafe {
            asm! {
                "hlt"
            }
        }
    }
}
