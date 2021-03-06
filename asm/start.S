.option norvc

.altmacro

.section .init
.global _start
_start:
	//Clear interrupts
	li 	t1, 0xb
	csrrc	zero, mstatus, t1
	//Now, check to see what CPU is running.
	//We check the mhartid (machine hardware thread id)
	csrr	t0, mhartid
	//We are going to use CPU #0, everything else will go into sleep loop
	bnez	t0, park
	//Set the global and stack memory pointers
	la	gp, __advos_global_pointer$
	la	sp, _sp
	//Copy data section from flash memory into RAM
	//_data_lma - This is the memory address in the FLASH memory you need to copy FROM.
	//_data - The start of the data section in RAM.
	//_edata - The end of the data section in RAM.
	la	a0, _data_lma
	la	a1, _data
	la	a2, _edata
	bgeu	a1, a2, 2f
1:
	// t0 = *a0
	lw	t0, 0(a0)
	// *a1 = t0
	sw	t0, 0(a1)
	addi	a0, a0, 4
	addi	a1, a1, 4
	bltu	a1, a2, 1b
2:
	//Clear the BSS. The BSS section must be all 0s
	//otherwise global unintialized or set to 0
	//will not get a proper value
	la	a0, __bss_start
	la	a1, __bss_end
	bgeu	a0, a1, 2f
1:
	sw	zero, (a0)
	addi	a0, a0, 4
	bltu	a0, a1, 1b
2:
	auipc	ra, 0
	addi	sp, sp, -16
	sw	ra, 8(sp)
	li	a0, 0
	li	a1, 0

	//Go into machine mode
	li	t5, 0x1800
	csrw	mstatus, t5

	//Put main into mepc and return to jump to it
	//Main is written in Rust (lib.rs)
	la	t1, main
	csrw	mepc, t1

	mret

park:
	//All HARTs that are != 0 will go here and spin in a wait loop
	wfi
	j	park
