SOURCE_FILES := $(shell test -e src/ && find src -type f)

.PHONY: build
build: target/x86_64-unknown-hermit/release/redis_test

target/x86_64-unknown-hermit/release/redis_test: $(SOURCE_FILES) Cargo.*
	cargo build \
    -Zbuild-std=std,panic_abort \
    --target x86_64-unknown-hermit \
    --release

.PHONY: run_single_core_no_pool
run_single_core_no_pool: target/x86_64-unknown-hermit/release/redis_test
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-display none -serial stdio \
		-smp 1 \
		-m 100M \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-kernel rusty-loader-x86_64 \
		-append "-- -r 10.0.2.2" \
		-initrd target/x86_64-unknown-hermit/release/redis_test \
		-netdev user,id=u1,hostfwd=tcp::3000-:3000 \
		-device rtl8139,netdev=u1

.PHONY: run_single_core_enable_pool
run_single_core_enable_pool: target/x86_64-unknown-hermit/release/redis_test
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-display none -serial stdio \
		-smp 1 \
		-m 100M \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-kernel rusty-loader-x86_64 \
		-append "-- -r 10.0.2.2 --enable-connection-pool" \
		-initrd target/x86_64-unknown-hermit/release/redis_test \
		-netdev user,id=u1,hostfwd=tcp::3000-:3000 \
		-device rtl8139,netdev=u1


.PHONY: run_multi_core_enable_pool
run_multi_core_enable_pool: target/x86_64-unknown-hermit/release/redis_test
	qemu-system-x86_64 \
		-cpu qemu64,apic,fsgsbase,fxsr,rdrand,rdtscp,xsave,xsaveopt \
		-display none -serial stdio \
		-smp 4 \
		-m 100M \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-kernel rusty-loader-x86_64 \
		-append "-- -r 10.0.2.2 --enable-connection-pool" \
		-initrd target/x86_64-unknown-hermit/release/redis_test \
		-netdev user,id=u1,hostfwd=tcp::3000-:3000 \
		-device rtl8139,netdev=u1
