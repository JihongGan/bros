# ref. https://github.com/mit-pdos/xv6-riscv/blob/riscv/Makefile

K = kernel
U = user
KERNELPATH = target/riscv64gc-unknown-none-elf/debug/kernel
TOOLPREFIX = riscv64-unknown-elf-
QEMU = qemu-system-riscv64
ifndef NCPU
NCPU := 1
endif

# try to generate a unique GDB port
GDBPORT = $(shell expr `id -u` % 5000 + 25000)
# QEMU's gdb stub command line changed in 0.11
QEMUGDB = $(shell if $(QEMU) -help | grep -q '^-gdb'; \
	then echo "-gdb tcp::$(GDBPORT)"; \
	else echo "-s -p $(GDBPORT)"; fi)

QEMUOPTS = -machine virt -bios none -kernel $(KERNELPATH) -m 128M -smp $(NCPU) -nographic
QEMUOPTS += -global virtio-mmio.force-legacy=false
QEMUOPTS += -drive file=fs.img,if=none,format=raw,id=x0
QEMUOPTS += -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

UPROGS =

run: fs.img build
	$(QEMU) $(QEMUOPTS)
# cd $(K) && cargo run

debug: build .gdbinit fs.img
	@echo "*** Now run 'gdb' in another window." 1>&2
	$(QEMU) $(QEMUOPTS) -S $(QEMUGDB)

build:
	cd $(K) && cargo build

clean:
	cd $(K) && cargo clean
	rm -f fs.img

.gdbinit: .gdbinit.tmpl-riscv
	sed "s/:1234/:$(GDBPORT)/" < $^ > $@

mkfs/mkfs: mkfs/mkfs.c mkfs/fs.h mkfs/params.h
	gcc -Werror -Wall -I. -o mkfs/mkfs mkfs/mkfs.c

fs.img: mkfs/mkfs README.md $(UPROGS)
	mkfs/mkfs ./fs.img README.md $(UPROGS)
