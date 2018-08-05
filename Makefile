# Basic Info
OS_NAME := SawayakanaOS
CUSTOM_TARGET := i386-sawayakanaos

# Commands
MAKE := make

LD_FLAG := -m elf_i386 -n

# Directories
BOOT_DIR := boot
KERNEL_DIR := kernel
KERNEL_BUILD_DIR := $(KERNEL_DIR)/target/$(CUSTOM_TARGET)/debug
LD_DIR := ldscript
BUILD_DIR := build
ISO_FILES := $(BUILD_DIR)/isofiles

# Files
LOADER_SRCS := $(wildcard $(BOOT_DIR)/*.S)
LOADER_OBJS := $(patsubst %.S, %.o, $(LOADER_SRCS))
KERNEL_SRCS := $(KERNEL_DIR)/src/lib.rs
KERNEL_OBJS := $(KERNEL_BUILD_DIR)/libkernel.a
LD_SCRIPT := $(LD_DIR)/linker.ld
GRUB_CFG := grub.cfg
OS_OBJ := $(BUILD_DIR)/boot/sawayakanaos.bin
ISO := $(BUILD_DIR)/$(OS_NAME).iso

.PHONY: build clean

build: $(ISO)

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

$(ISO): $(OS_OBJ) $(GRUB_CFG)
	mkdir -p $(BUILD_DIR)/boot/grub
	cp $(GRUB_CFG) $(BUILD_DIR)/boot/grub/
	grub-mkrescue --verbose -o $(ISO) $(BUILD_DIR) 2> /dev/null

$(OS_OBJ): $(LOADER_OBJS) $(KERNEL_OBJS) $(LD_SCRIPT)
	mkdir -p $(BUILD_DIR)/boot
	ld $(LD_FLAG) -T $(LD_SCRIPT) $(LOADER_OBJS) $(KERNEL_OBJS) -o $(OS_OBJ)

$(KERNEL_OBJS): $(KERNEL_SRCS)
	cd $(KERNEL_DIR); $(MAKE)

$(LOADER_OBJS): $(LOADER_SRCS)
	cd $(BOOT_DIR); $(MAKE)

clean:
	cd $(BOOT_DIR); $(MAKE) clean
	cd $(KERNEL_DIR); $(MAKE) clean
	rm -rf $(BUILD_DIR)
