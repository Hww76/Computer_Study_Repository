.PHONEY: run run_gdb

run :
	@cargo build --release # 重新编译代码
	@rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin # 去除ELF文件中的元数据，生成可执行文件
	@stat target/riscv64gc-unknown-none-elf/release/os # 查看ELF文件大小
	@stat target/riscv64gc-unknown-none-elf/release/os.bin # 查看.bin可执行文件大小
	@qemu-system-riscv64 \ # 将bootlodar和内核代码放入riscv64架构虚拟系统上运行
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \

run_w_g: run # 依赖于run
	@qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S # 设置可以进行gdb调试的参数

run_gdb : # 运行make run_w_g时，新开控制台运行make run_gdb，运行gbd进行内核代码调试
	riscv64-unknown-elf-gdb \ # 运行基于riscv64架构的，使用elf文件的gdb
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234' # 监听端口1234