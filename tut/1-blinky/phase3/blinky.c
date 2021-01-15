#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

#define BLINK_SLEEP_MS 150

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

int kmain(void) {
  /*
  Q: Now, read the documentation for GPFSELn register on pages 91 and 92. 
  We write to this register to set up a pin as an output or input. 
  Which value to which field in register GPFSEL1 must be written 
    so that GPIO pin 16 is set as an output?

  A: Write 0x1 to field FSEL16
  */

  /*
  Q: Now, read the documentation for the GPSET0 and GPCLR0 registers on page 95. 
  We write to GPSET0 to set a pin (turn it on) and write to GPCLR0 to 
    clear a pin (turn it off). 
  Which value do we write to which field in these registers to set/clear pin 16?

  A: Write 0x1 to field SET16 in register GPSET0 and 
    0x1 to field CLEAR16 in registerGPCLR0.
  */

  // FIXME: STEP 1: Set GPIO Pin 16 as output.
  *GPIO_FSEL1 = 0x1 << 18;
  // FIXME: STEP 2: Continuously set and clear GPIO 16.
  while (1) {
    *GPIO_SET0 = 0x1 << 16;
    spin_sleep_ms(BLINK_SLEEP_MS);
    *GPIO_CLR0 = 0x1 << 16;
    spin_sleep_ms(BLINK_SLEEP_MS);
  }
}
