#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO, i2c::I2C};
use esp_backtrace as _;
use esp_println::println;
use axp20x::Axpxx;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // SET GPIO4 as an output and set its state high initially
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    println!("Configure AXP202");
    
    let _i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        1u32.MHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    
    let mut pmu = Axpxx::new(_i2c);
    pmu.init().expect("Cannot init AXP202");

    println!("AXP202 should be connected");

    let axp_battery_percentage = pmu.get_battery_percentage();
    let axp_battery_voltage = pmu.get_battery_voltage();
    
    println!("AXP202 has battery: {:?}% and {:?}mV", axp_battery_percentage, axp_battery_voltage);


    println!("Init Motor");
    let mut motor = io.pins.gpio4.into_push_pull_output();

    //motor.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    loop {
        //motor.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}
