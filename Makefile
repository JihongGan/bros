# ref. https://github.com/mit-pdos/xv6-riscv/blob/riscv/Makefile

K = kernel
U = user
ifndef KERNELPATH
KERNELPATH = target/riscv64gc-unknown-none-elf/debug/kernel
endif
TOOLPREFIX = riscv64-unknown-elf-
QEMU = qemu-system-riscv64
SBI = opensbi/build/platform/generic/firmware/fw_jump.elf
KERNELENTRY = 0x80200000
ifndef NCPU
NCPU := 1
endif

# try to generate a unique GDB port
GDBPORT = $(shell expr `id -u` % 5000 + 25000)
# QEMU's gdb stub command line changed in 0.11
QEMUGDB = $(shell if $(QEMU) -help | grep -q '^-gdb'; \
	then echo "-gdb tcp::$(GDBPORT)"; \
	else echo "-s -p $(GDBPORT)"; fi)

QEMUOPTS = -machine virt -bios $(SBI) -m 128M -smp $(NCPU) -nographic
QEMUOPTS += -device loader,addr=$(KERNELENTRY),file=$(KERNELPATH)
QEMUOPTS += -drive file=fs.img,if=none,format=raw,id=x0
QEMUOPTS += -global virtio-mmio.force-legacy=false
QEMUOPTS += -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

UPROGS =

run: build
	$(QEMU) $(QEMUOPTS)

debug: build .gdbinit
	@echo "*** Now run 'gdb' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -S $(QEMUGDB)

build: fs.img $(SBI)
	cd $(K) && cargo build

test: build
	chmod +x $(K)/runner.sh
	cd $(K) && cargo test

clean:
	cd $(K) && cargo clean
	cd opensbi && make clean
	rm -f fs.img

.gdbinit: .gdbinit.tmpl-riscv
	sed "s/:1234/:$(GDBPORT)/" < $^ > $@

mkfs/mkfs: mkfs/mkfs.c mkfs/fs.h
	gcc -Werror -Wall -I. -o mkfs/mkfs mkfs/mkfs.c

fs.img: mkfs/mkfs README.md $(UPROGS)
	mkfs/mkfs ./fs.img README.md $(UPROGS)

# "generic" means qeme/virt
$(SBI):
	cd opensbi && make PLATFORM=generic CROSS_COMPILE=$(TOOLPREFIX)
