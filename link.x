/* Memory layout of the Microbit V2 Nordic nRF52833 Microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The entry point is the reset handler */
ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{
  /* Vector table for Cortex-M processor starts at 0x0 (FLASH) */
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));

    /* The 14 Cortex-M exception handler entriesx must follow */
    KEEP(*(.vector_table.exceptions));
  } > FLASH


  /* The code sits in read only memory in the .text segment */
  .text :
  {
    *(.text .text.*);
  } > FLASH


  /* All read only static data goes into .rodata */
  .rodata :
  {
    . = ALIGN(4);
    _srodata = .;
    *(.rodata .rodata.*);
    . = ALIGN(4);
    _erodata = .;
  } > FLASH

  /* Statically allocated but uninitialized data ("Block Starting Symbol") */
  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /* Statically allocated data is writable*/
  .data : AT(_erodata) /* LMA is right after .rodata in ROM */
  {
    . = ALIGN(4); /* VMA starts on word boundary */
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4); /* VMA ends on word boundary */
    _edata = .;
  } > RAM


  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

/* Provide defaults for the 8 Cortex-M exceptions */
PROVIDE(NMI = UnhandledException);
PROVIDE(HardFault = UnhandledException);
PROVIDE(MMFault = UnhandledException);
PROVIDE(BusFault = UnhandledException);
PROVIDE(UsageFault = UnhandledException);
PROVIDE(SVCall = UnhandledException);
PROVIDE(PendSV = UnhandledException);
PROVIDE(SysTick = UnhandledException);

ASSERT(ADDR(.text) % 4 == 0, ".text should be word aligned");

ASSERT(_erodata % 4 == 0, ".rodata should end word algined");

ASSERT(_sbss == ORIGIN(RAM), ".bss should be at start of RAM");
ASSERT(_sbss % 4 == 0, ".bss should start word algined");
ASSERT(_ebss % 4 == 0, ".bss should end word algined");

ASSERT(LOADADDR(.data) == _erodata, ".data LMA should be at end of .rodata");
ASSERT(_sdata % 4 == 0, ".data VMA should start word algined");
ASSERT(_edata % 4 == 0, ".data VMA should end word algined");
