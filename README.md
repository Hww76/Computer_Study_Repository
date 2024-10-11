# Computer Study Repository
学习计算机科学与技术中个人编写的各种程序的保存仓库

# TODO List
1. [Learn Git Branching](https://learngitbranching.js.org/?locale=zh_CN) 第一次通关(24/9/25),以后每个月通关一次，直至熟练，在下方打卡。
2. 学习Rust语言 完成时间： 24/10/10
3. 学完OS导论部分 完成时间： (Option)
4. RISC-V处理器的指令集和部分特权操作
5. RISC-V汇编语言
6. [什么是virt设备平台](#todo1)
7. 复习Rust语法，计划用时3天。完成时间：
8. 分析Rust编程代码110题，计划用时3天。完成时间：

# 2024年
## [9月](./log/2024-9.md)

## 10月
10/1
* 完成rust课程48-57题

10/2
* Qemu模拟器
    * qemu-system-riscv64虚拟化的是基于riscv64指令集架构的主机，下面是它的配置。
    * -machine virt：设置设备平台为virt<a id = "todo1"><font color = red>(待探究)</font></a>
    * -nographic：无图形化界面
    * -bios：选择引导加载程序(bootloader)
    * -device：loader属性可以把宿主机的一个文件放入Qemu模拟的主机中的物理内存，file指出文件路径，addr指出存放的位置。
    ```
    qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
    ```
* Qemu启动流程
    1. 第一个阶段由固化在 Qemu 内的一小段汇编程序负责。
    主要工作：将必要的文件载入到 Qemu 物理内存，Qemu CPU 的程序计数器（PC）会被初始化为 0x1000 ，执行的第一条指令位于物理地址 0x1000 ，接下来它将执行数条指令并跳转到物理地址 0x80000000 对应的指令处并进入第二阶段。
    2. 第二个阶段由 bootloader 负责。
    主要工作：bootloader 负责对计算机进行一些初始化工作，并跳转到下一阶段软件的入口，在 Qemu 上即可实现将计算机控制权移交给我们的内核镜像 os.bin
    3. 第三个阶段则由内核镜像负责。
* 程序内存布局
    1. .text：存放程序的所有汇编代码。
    2. .rodata：存放只读的全局数据。
    3. .data：存放可修改的全局数据。
    4. .bss：保存程序中那些未初始化的全局数据，通常由程序的加载者代为进行零初始化，即将这块区域逐字节清零。
    5. heap：存放程序运行时动态分配的数据。
    6. stack：函数调用上下文的保存与恢复，每个函数作用域内的局部变量，它向低地址增长。
* 编译流程
    1. 编译器：源文件从某门高级编程语言转化为汇编语言。
    2. 汇编器：源文件中的文本格式的指令转化为机器码，得到二进制的目标文件。
    3. 链接器：所有目标文件以及一些可能的外部目标文件链接在一起形成一个完整的可执行文件。第一步：将来自不同目标文件的段在目标内存布局中重新排布。第二件事情是将符号替换为具体地址。
* 在Qemu上正确运行脚本流程：
    1. 通过链接脚本调整内核可执行文件的内存布局，使得被执行的第一条指令位于地址 0x80200000 处。
    2. 使得代码段地址低于其他段
    3. 丢掉可执行文件中的元数据，得到内核镜像(只有代码段和数据段)。

* 去除ELF的元数据，得到可执行文件
    ```
    rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
    ```
* 比较内核可执行文件和内核镜像的大小
    ```
    stat target/riscv64gc-unknown-none-elf/release/os
    stat target/riscv64gc-unknown-none-elf/release/os.bin
    ```
* 基于 GDB 验证启动流程,当前目录为os
    ```
    qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
    ```
    -s：可以使 Qemu 监听本地 TCP 端口 1234 等待 GDB 客户端连接。
    -S：可以使 Qemu 在收到 GDB 的请求后再开始运行。因此，Qemu 暂时没有任何输出。
    注意，如果不想通过 GDB 对于 Qemu 进行调试而是直接运行 Qemu 的话，则要删掉最后一行的 -s -S 。

* GDB客户端连接到Qemu
    ```
    riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
    ```

* gdb指令
    从PC值位置开始，在内存中反汇编10条指令
    ```
    x/10i $pc
    ```
    单步执行
    ```
    si
    ```
    在物理地址设置断点
    ```
    b *0x地址值
    ```
    显示寄存器值
    ```
    p/x $sp/t0
    p/d $x1
    ```

* Qemu模拟，反汇编的源码
    ```
    0x1000:     auipc   t0,0x0
    0x1004:     addi    a1,t0,32
    0x1008:     csrr    a0,mhartid
    0x100c:     ld      t0,24(t0)
    0x1010:     jr      t0
    0x1014:     unimp
    0x1016:     unimp
    0x1018:     unimp
    0x101a:     0x8000
    0x101c:     unimp

    0x80000000:         auipc   sp,0x28
    0x80000004: mv      sp,sp
    0x80000008: lui     t0,0x4
    0x8000000a: addi    t1,a0,1
    0x8000000e: add     sp,sp,t0
    0x80000010: addi    t1,t1,-1
    0x80000012: bnez    t1,0x8000000e
    0x80000016: j       0x8001125a
    0x8000001a: unimp
    0x8000001c: addi    sp,sp,-48

    0x80200004: unimp
    0x80200006: unimp
    0x80200008: unimp
    0x8020000a: unimp
    ```

* 我生成的Qemu模拟，反汇编的源码
    ```
    0x1000:      auipc   t0,0x0
    0x1004:      addi    a2,t0,40
    0x1008:      csrr    a0,mhartid
    0x100c:      ld      a1,32(t0)
    0x1010:      ld      t0,24(t0)
    0x1014:      jr      t0
    0x1018:      unimp
    0x101a:      .insn   2, 0x8000
    0x101c:      unimp
    0x101e:      unimp

    0x80000000:  auipc   ra,0x2
    0x80000004:  jalr    834(ra)
    0x80000008:  auipc   ra,0x0
    0x8000000c:  jalr    116(ra)
    0x80000010:  j       0x80001690
    0x80000014:  unimp
    0x80000016:  addi    sp,sp,-80
    0x80000018:  sd      ra,72(sp)
    0x8000001a:  ld      a1,40(a0)
    0x8000001c:  ld      a2,32(a0)

    0x80200000:  li      ra,100
    0x80200004:  unimp
    0x80200006:  unimp
    0x80200008:  unimp
    0x8020000a:  unimp
    ```

10/10
* 完成第1阶段rust编程题

10/11  
* 加入第2阶段学习群
    * 学习规划 
        完成实验：3、4、5、6、8
        学习时长：20天
        规划：前7天打基础，对实验整体结构进行把握以及学习基础知识。之后每2天完成一个实验，1天了解实验要求内容，1天编写代码。余下3天动态调整。
    * 参考资料
        1.《计算机组成与设计（RISC-V版）》第一、二章，可以在整体结构上对 RISC-V 体系建立基本认知
        2. RISC-V手册：一本开源指令集的指南第第一、二、三、十章
    * 学习目标
        掌握RUST编程，理解RISC-V与OS相关的硬件特性（中断，异常，系统调用，寄存器，特权级，MMU...）。

