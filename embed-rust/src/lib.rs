// EASE OF UNDERSTANDING EXAMPLE OF HOW TO READ AND WRITE FROM A CHIP SET THIS IS AN EXAMPLE OF AN STM32L552KC Nucleo Board,
#![no_std]
/* Use Core As STD Library Will Not Work Here */
/* Call Pointer */
use core::ptr;
use core::panic::PanicInfo;

const PORTC_PIN7:           u32 = 7;                                        /* USER GREEN LED on GPIO A Bus, Pin 5  */
const LED_GRN:              u32 = PORTC_PIN7;                               /* USER GREEN LED on GPIO A Bus, Pin 5  */
const PORTB_PIN7:           u32 = 7;                                        /* USER BLUE LED on GPIO B Bus, Pin 7   */
const LED_BLU:              u32 = PORTB_PIN7;                               /* USER BLUE LED on GPIO B Bus, Pin 7   */
const PORTA_PIN9:           u32 = 9;                                        /* USER RED LED on GPIO 9 Bus, Pin 9    */
const LED_RED:              u32 = PORTA_PIN9;                               /* USER RED LED on GPIO 9 Bus, Pin 9    */

/* GPIO Port A REGISTERS */
const GPIOA_BASE:           u32 = 0x42020000;                               /* GPIO Port A base address */
const GPIOA_MODER:          *mut u32 = (GPIOA_BASE  + 0x00) as *mut u32;    /* Port A Mode register */
const GPIOA_OTYPER:         *mut u32 = (GPIOA_BASE  + 0x04) as *mut u32;    /* Port A Output Type Register */
const GPIOA_BSRR:           *mut u32 = (GPIOA_BASE  + 0x18) as *mut u32;    /* Output Data Set And Reset Register */

/* GPIO Port B REGISTERS */
const GPIOB_BASE:           u32 = 0x42020400;                               /* GPIO Port A base address */
const GPIOB_MODER:          *mut u32 = (GPIOB_BASE  + 0x00) as *mut u32;    /* Port A Mode register */
const GPIOB_OTYPER:         *mut u32 = (GPIOB_BASE  + 0x04) as *mut u32;    /* Port A Output Type Register */
const GPIOB_BSRR:           *mut u32 = (GPIOB_BASE  + 0x18) as *mut u32;    /* Output Data Set And Reset Register */

/* GPIO Port B REGISTERS */
const GPIOC_BASE:           u32 = 0x42020800;                               /* GPIO Port C base address */
const GPIOC_MODER:          *mut u32 = (GPIOC_BASE  + 0x00) as *mut u32;    /* Port C Mode register */
const GPIOC_OTYPER:         *mut u32 = (GPIOC_BASE  + 0x04) as *mut u32;    /* Port C Output Type Register */
const GPIOC_BSRR:           *mut u32 = (GPIOC_BASE  + 0x18) as *mut u32;    /* Output Data Set And Reset Register */

const PORTA_AHBEN:          u32 = 0;                                        /* GPIOA Enable is located on AHB2 Board Bit 0 */
const PORTB_AHBEN:          u32 = 1;                                        /* GPIOB Enable is located on AHB2 Board Bit 1 */
const PORTC_AHBEN:          u32 = 2;                                        /* GPIOC Enable is located on AHB2 Board Bit 2 */

/* Reset and Clock Control (RCC) */
const RCC_BASE:             u32 = 0x40021000;                               /* RCC base address */
const RCC_CR:               *mut u32 = (RCC_BASE + 0x00) as *mut u32;       /* Clock Control Register */
const RCC_AHB2ENR:          *mut u32 = (RCC_BASE + 0x4C) as *mut u32;       /* AHB2 Enable Register */

/* User required */                                          
const MASK_2_BIT:           u32 = 0x00000003;                               /* 2 bit mask, example 0011 = 0x03 */

#[no_mangle]
pub extern fn _system_init() {
    // RCC SHOULD ALWAYS BE IN THE SYSTEM INIT TRYING TO OPERATE THE GPIO PINS EVEN ACTIVATING WILL CAUSE ISSUES AS THERE IS NO CLOCK TO RUN
    unsafe{ptr::write_volatile(RCC_AHB2ENR, (1<<PORTC_AHBEN) | (1<<PORTB_AHBEN) | (1<<PORTA_AHBEN))};  // EN CLK FOR GPIO B and A
}

#[no_mangle]
pub extern fn _start() {
    /* Initialize the LED on L432KC board */ 
    /* Form a Pointer Via Register Address And Write Volitile To That Address */
    /* From Page 267 0x01 = General Output As The Output For The USER LED is at 3 it is x 2 */
    unsafe{ptr::write_volatile((GPIOA_MODER) as *mut u32, 1<<(LED_RED * 2))};
    unsafe{ptr::write_volatile((GPIOB_MODER) as *mut u32, 1<<(LED_BLU * 2))};
    unsafe{ptr::write_volatile((GPIOC_MODER) as *mut u32, 1<<(LED_GRN * 2))};
    /* From Page 268 0x0 = Push Pull From Board Docs, LED is Push Pull so we ensure the bit is not set by inverting
    In Practice this would set all others to open drain but since we are running only 1 output here we can get away with it */
    unsafe{ptr::write_volatile((GPIOA_OTYPER) as *mut u32, !(1<<(LED_RED)))};
    unsafe{ptr::write_volatile((GPIOB_OTYPER) as *mut u32, !(1<<(LED_BLU)))};
    unsafe{ptr::write_volatile((GPIOC_OTYPER) as *mut u32, !(1<<(LED_GRN)))};

    let mut i = 0;

    loop {
        while i <= 1200000 {
            if i == 300000 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOC_BSRR) as *mut u32, 1<<LED_GRN)};
            } else if i == 0 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOC_BSRR) as *mut u32, 1<<(LED_GRN + 16))}
            }

            if i == 600000 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<LED_BLU)};
            } else if i == 0 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOB_BSRR) as *mut u32, 1<<(LED_BLU + 16))}
            }

            if i == 900000 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOA_BSRR) as *mut u32, 1<<LED_RED)};
            } else if i == 0 {
                /* From Page 270 0x1 turns on the output and offset + 16 will reset the output when set*/
                unsafe{ptr::write_volatile((GPIOA_BSRR) as *mut u32, 1<<(LED_RED + 16))}
            }

            i += 1;
        }
        i = 0
	}
}

/* Required if an unwind of the program happens */
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {
    loop {}
}

/* Required if a panic of the program happens */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}