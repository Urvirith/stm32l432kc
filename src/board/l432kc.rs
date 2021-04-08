use crate::hal::common;

/* Register Base */

/* Reset and Clock Control (RCC) */
pub const RCC_BASE:                 u32 = 0x40021000;

/* General Purpose I/O */
pub const GPIOA_BASE:               u32 = 0x48000000;  
pub const GPIOB_BASE:               u32 = 0x48000400; 
pub const GPIOC_BASE:               u32 = 0x48000800;

/* Timers */
pub const TIMER1_BASE:              u32 = 0x40012C00;
pub const TIMER2_BASE:              u32 = 0x40000000;
pub const TIMER3_BASE:              u32 = 0x40000400;
pub const TIMER6_BASE:              u32 = 0x40001000;
pub const TIMER7_BASE:              u32 = 0x40001400;
pub const TIMER15_BASE:             u32 = 0x40014000;
pub const TIMER16_BASE:             u32 = 0x40014400;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART1_BASE:              u32 = 0x40013800;
pub const USART2_BASE:              u32 = 0x40004400;      
pub const USART3_BASE:              u32 = 0x40004800;

/* Inter-Integrated Circuit (I2C) */
pub const I2C1_BASE:                u32 = 0x40005400; 
//pub const I2C2_BASE:                u32 = 0x40005800; Does not exist in the 432KC
pub const I2C3_BASE:                u32 = 0x40005C00;
      
/* Reset and Clock Control (RCC) */
pub const GPIOA_RCC_AHB2_ENABLE:    u32 = common::BIT_0;
pub const GPIOB_RCC_AHB2_ENABLE:    u32 = common::BIT_1;

/* General Purpose I/O */
pub const USER_LED:                 u32 = 3;
pub const USER_LED_BIT:             u32 = common::BIT_3;

/* Timer */
pub const TIMER2_RCC_APB1R1_ENABLE: u32 = common::BIT_0;

/* USART (Universal Synchronous and Asynchronous Receiver Transmitter) */
pub const USART2_RCC_APB1R1_ENABLE: u32 = common::BIT_17;
pub const PORTA_PIN2:               u32 = 2;    //A7    TX
pub const PORTA_PIN3:               u32 = 3;    //A2    RX
pub const USART2_TX:                u32 = PORTA_PIN2;
pub const USART2_RX:                u32 = PORTA_PIN3;

/* I2C 1*/
/* RCC */
pub const I2C1_RCC_APB1R1_ENABLE:   u32 = common::BIT_21;

/* PINS */
pub const PORTB_PIN6:               u32 = 6;    //D5    SCL
pub const PORTB_PIN7:               u32 = 7;   //D4    SDA
pub const I2C1_SCL:                 u32 = PORTB_PIN6;
pub const I2C1_SDA:                 u32 = PORTB_PIN7;


