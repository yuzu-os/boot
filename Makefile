IMAGE=uefi.img
SECTORS=262144
SECTOR_SIZE=512
RELEASE?=0

QEMU_SYSTEM_X64=qemu-system-x86_64
BIOS=/usr/share/edk2/ovmf/OVMF_CODE.fd

ifeq ($(RELEASE),1)
TARGET_IMAGE:=target/x86_64-unknown-uefi/release/boot.efi
else
TARGET_IMAGE:=target/x86_64-unknown-uefi/debug/boot.efi
endif

PART_IMAGE:=$(shell mktemp)
PART_SECTORS:=$(shell expr $(SECTORS) - 2047)

.PHONY: all
all: image

.PHONY: run
run: image
	$(QEMU_SYSTEM_X64) -cpu qemu64 -bios $(BIOS) -net none -hda $(IMAGE)

.PHONY: clean
clean:
	rm -fv $(IMAGE)

.PHONY: cargo
cargo:
ifeq ($(RELEASE),1)
	cargo build --release
else
	cargo build
endif

.PHONY: part
part: cargo
	dd if=/dev/zero of=$(PART_IMAGE) bs=$(SECTOR_SIZE) count=$(PART_SECTORS)
	mformat -i $(PART_IMAGE)
	mmd -i $(PART_IMAGE) ::EFI
	mmd -i $(PART_IMAGE) ::EFI/BOOT
	mcopy -i $(PART_IMAGE) $(TARGET_IMAGE) ::EFI/BOOT/BOOTX64.EFI

.PHONY: image
image: part
	dd if=/dev/zero of=$(IMAGE) bs=$(SECTOR_SIZE) count=$(shell expr $(SECTORS) + 64)
	parted $(IMAGE) -s -a minimal mklabel gpt
	parted $(IMAGE) -s -a minimal mkpart EFI FAT16 2048s $(SECTORS)s
	parted $(IMAGE) -s -a minimal toggle 1 boot
	dd if=$(PART_IMAGE) of=$(IMAGE) bs=$(SECTOR_SIZE) count=$(PART_SECTORS) seek=2048 conv=notrunc
