#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let ch1 = PwmPin::new_ch1(p.PA8, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM1, Some(ch1), None, None, None, hz(50), Default::default());
    //let max_duty = pwm.get_max_duty();
    //let min_pulse: u32 = 500; // Minimum pulse width in microseconds
    //let max_pulse: u32 = 2500; // Maximum pulse width in microseconds

    pwm.enable(Channel::Ch1);

    let ch2 = PwmPin::new_ch2(p.PA9, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM1, Some(ch2), None, None, None, hz(50), Default::default());
    //let max_duty = pwm.get_max_duty();
    //let min_pulse: u32 = 500; // Minimum pulse width in microseconds
    //let max_pulse: u32 = 2500; // Maximum pulse width in microseconds
    
    pwm.enable(Channel::Ch2);

    info!("PWM initialized");

    let max_duty = pwm.get_max_duty();
    info!("Max duty cycle value: {}", max_duty);

	loop {
        // Set duty cycle for min pulse width (500µs, 2.5% of 20ms period)
        pwm.set_duty(Channel::Ch1, (max_duty as f32 * 0.025) as u16);
		Timer::after_millis(1000).await;
        info!("Duty 2.5%");

        // Set duty cycle for mid pulse width (1500µs, 7.5% of 20ms period)
        pwm.set_duty(Channel::Ch1, (max_duty as f32 * 0.075) as u16);
		Timer::after_millis(1000).await;
        info!("Duty 7.5%");

        // Set duty cycle for min pulse width (500µs, 2.5% of 20ms period)
        pwm.set_duty(Channel::Ch2, (max_duty as f32 * 0.025) as u16);
        Timer::after_millis(1000).await;
        info!("Duty 2.5%");

        // Set duty cycle for mid pulse width (1500µs, 7.5% of 20ms period)
        pwm.set_duty(Channel::Ch2, (max_duty as f32 * 0.075) as u16);
        Timer::after_millis(1000).await;
        info!("Duty 7.5%");

        // Set duty cycle for max pulse width (2500µs, 12.5% of 20ms period)
        //pwm.set_duty(Channel::Ch1, (max_duty as f32 * 0.125) as u16);
		//Timer::after_millis(1000).await;
        //info!("Duty 12.5%");
    }
}
