# Basic Info
CUSTOM_TARGET := i386-sawayakanaos

# Directories
SRC_DIR := src
TARGET_DIR := target
DEBUG_BUILD_DIR :=  $(TARGET_DIR)/$(CUSTOM_TARGET)/debug/

# Files
SRCS := $(SRC_DIR)/*.rs $(SRC_DIR)/drivers/*.rs
CUSTOM_TARGET_FILE := $(CUSTOM_TARGET).json

# COMMAND
CARGO_FLAG := --target $(CUSTOM_TARGET_FILE)

.PHONY: build clean

build: $(SRCS)
	cargo xbuild $(CARGO_FLAG)

clean:
	rm -rf $(TARGET_DIR)
