# Basic Info
CUSTOM_TARGET := i386-sawayakanaos

# Directories
SRC_DIR := src
TARGET_DIR := target
DEBUG_BUILD_DIR :=  $(TARGET_DIR)/$(CUSTOM_TARGET)/debug/

# Files
SRCS := $(SRC_DIR)/lib.rs

# COMMAND
XARGO_FLAG := --target $(CUSTOM_TARGET)

.PHONY: build clean

build: $(SRCS)
	xargo build $(XARGO_FLAG)

clean:
	rm -rf $(TARGET_DIR)
