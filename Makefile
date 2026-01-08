PACKAGE_NAME := $(shell cargo metadata --format-version 1 | jq -r .packages[0].name)

TARGET_ROOT := $(shell cargo metadata --format-version 1 | jq -r .target_directory)
DEBUG_TARGET := $(TARGET_ROOT)/target/debug/$(PACKAGE_NAME)
RELEASE_TARGET := $(TARGET_ROOT)/target/release/$(PACKAGE_NAME)
TARGET := $(DEBUG_TARGET)

QEMU_FLAGS := -machine type=pc-i440fx-3.1 -m 100M
CARGO_FLAGS :=

ifeq ($(RELEASE), 1)
	TARGET := $(RELEASE_TARGET)
	CARGO_FLAGS := $(CARGO_FLAGS) --release
endif

.PHONY: help
help:
	@echo "available commands:"
	@echo "  make help          print this message"
	@echo "  make build         build the kernel"
	@echo "  make run           run the kernel with QEMU"
	@echo "  make print-size    print the size of the kernel"
	@echo "  make clean         remove intermediate files"
	@echo "  make re            clean then build the kernel again"

.PHONY: build
build:
	cargo build $(CARGO_FLAGS)

.PHONY: run
run:
	cargo build $(CARGO_FLAGS)
	qemu-system-i386 -kernel $(TARGET) $(QEMU_FLAGS)

.PHONY: print-size
print-size:
	@cargo build -q $(CARGO_FLAGS)
	@du -h $(TARGET)

.PHONY: clean
clean:
	cargo clean

.PHONY: re
re:
	@make --no-print-directory clean
	@make --no-print-directory build
