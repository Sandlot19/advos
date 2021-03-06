// trap.S
//
// This file contains relevant assembly labels for handling
// {traps, interrupts, exceptions}. When any of these traps are taken
// trap_handler will begin executing by saving the current register context
// and jumping to the Rust |handle_trap| function, which will determine how
// to handle the given cause.

.altmacro

// Allocate the necessary space in the .bss section
.global GLOBAL_CTX
.common GLOBAL_CTX, 32*4, 4

// Macro to save a register to memory
.macro sv z, jacob=t6
    sw  x\z, ((\z)*4)(\jacob)
.endm

// Macro to load a register from memory
.macro lv z, jacob=t6
    lw  x\z, ((\z)*4)(\jacob)
.endm

// This is called from Rust, so we know we're in machine mode
// Once this returns, {traps, interrupts, exceptions} are enabled
// and can be triggered at any time.
.global enable_interrupts
enable_interrupts:
  // Enable interrupts with MPIE and MIE bits in mstatus
  // MPIE => Machine Previous Interrupt Enable
  //      => bit 7
  // MIE  => Machine Interrupt Enable
  //      => bit 3
  csrr  t0, mstatus
  li    t1, 0x88
  or    t0, t0, t1
  csrw  mstatus, t0

  // Enable HW|CLINT|SW interrupts
  // (bits) 11     7  3 in mie = 0x888
  li    t1, (1 << 11) | (1 << 7) | (1 << 3)
  csrrw zero, mie, t1

  // Set RISC-V trap handler function to be the mtvec
  la    t1, trap_handler
  
  // ensure that the trap handler is aligned by 4 since the lower 2 bits of
  // mtvec _must_ be 0 for MODE to be set to direct since our trap handler
  // handles all interrupts/traps
  li    t0, ~( (1 << 1) | 1 )
  and   t1, t1, t0 
  csrw  mtvec, t1

  ret

// RISC-V trap handler function has to be aligned by 4. This function will jump
// to the rust trap_handler where the trap will _actually_ be handled. This
// label simply saves registers and restores them upon the trap being
// successfully handled.
.global trap_handler
.align 4
trap_handler:
  // t6 will be the temporary register used to hold the stack pointer as the
  // registers are all saved, so temporarily save it on the stack
  csrw mscratch, t6

  // |GLOBAL_CTX| is located in the bss section and will hold the register
  // context that was interrupted and be restored upon the completion of the
  // |handle_trap| function in rust
  la   t6, GLOBAL_CTX

  // start at 1 because we don't need to save 0 register
  .set i, 1
  .rept 30
    sv %i
    .set i, i+1
  .endr

  // restore and save t6 to |GLOBAL_CTX|
  csrr t6, mscratch
  la  t5, GLOBAL_CTX
  sv  31, t5

  // Set up arguments for |handle_trap| in rust
  // arg0 = mcause
  // arg1 = mepc
  csrr a0, mcause
  csrr a1, mepc

  call handle_trap

  // The checks to determine if the next instruction is offset 2 or 4
  // due to compressed instructions is done in Rust, save it to the stack
  // temporarily to restore register context
  csrw mepc, a0

  // restore register context that was stored to |GLOBAL_CTX|
  la  t6, GLOBAL_CTX

  // start at 1 because we don't need to load 0 register
  .set i, 1
  .rept 30
    lv %i
    .set i, i+1
  .endr

  mret
