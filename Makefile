
TARGET = aarch64-unknown-none

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld

OBJCOPY = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary

QEMU_CMD = qemu-system-aarch64 -M raspi3 -kernel kernel8.img

# these are keywords and not files 
.PHONY: all qemu qemu_debug clippy clean objdump nm webdav picocom

all: clean kernel8.img

# builds the sources
target/$(TARGET)/release/kernel8: $(SOURCES)
	cargo xbuild --target=$(TARGET) --release

# concats the code into a single image
kernel8.img: target/$(TARGET)/release/kernel8
	cp $< .
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< kernel8.img

# runs in the emulator
qemu: all
	$(QEMU_CMD) -serial null -serial stdio

# runs picocom
picocom:
	picocom -b 115200 /dev/tty.Repleo-PL2303-00001014 --imap lfcrlf

# runs in the emulator
qemu_debug: all
	$(QEMU_CMD) -serial null -serial stdio -d int

# copy to webdav 2 mount
webdav: all
	cp kernel8.img /Volumes/192.168.0.1/kernel8.img

# linter
clippy:
	cargo xclippy --target=$(TARGET)

# cleans the project
clean:
	cargo clean

# prints in asm
objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex kernel8

# prints the symbols and locations in the memory
nm:
	cargo nm --target $(TARGET) -- kernel8 | sort
