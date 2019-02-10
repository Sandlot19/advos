.option norvc
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
	la	gp, __global_pointer$
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

	//Go into machine mode, and enable MPIE and MIE
  // MPIE => Machine Previous Interrupt Enable
  //      => bit 7
  // MIE  => Machine Interrupt Enable
  //      => bit 3
	li	t5, 0x1888
	csrw	mstatus, t5
	//Put mepc into main and return to jump to it
	//Main is written in Rust (lib.rs)
	la	t1, main
	csrw	mepc, t1

  // Enable HW|CLINT|SW interrupts
  // (bits) 11     7  3 in mie = 0x888
  li    t1, (1 << 11) | (1 << 7) | (1 << 3)
  csrrw zero, mie, t1

  // Set RISC-V trap handler function to be the mtvec
  la    t1, trap_handler
  
  // ensure that the trap handler is aligned by 4 since the lower 2 bits of
  // mtvec _must_ be 0 to for MODE to be set to direct since our trap handler
  // handles all interrupts/traps
  li    t0, (1 << 3) | (1 << 2)
  and   t1, t1, t0 
  csrw  mtvec, t1

	mret

park:
	//All HARTs that are != 0 will go here and spin in a wait loop
	wfi
	j	park

// RISC-V trap handler function has to be aligned by 4. This function will jump
// to the rust trap_handler where the trap will _actually_ be handled. This
// label simply saves registers and restores them upon the trap being
// successfully handled.
.global trap_handler
.align 4
trap_handler:
  // save mepc - the instruction that was interrupted by the trap
  // and mstatus. The csrrs pseudo-instruction will not execute the
  // set operation if the src register is x0 (zero).
  csrr s1, mepc
  csrr s2, mstatus

  // set up arguments for rust function
  // arg0 = mcause
  csrr a0, mcause

  // TODO: add code to jump to the rust trap handler
  j handle_trap

  // restore the mepc so we jump to the code that was running before the trap
  csrw  mepc, s1
  csrw  mstatus, s2
  mret
