use std::{thread, time::Duration};
use esp_idf_svc::hal::{
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, LedcTimer},
    prelude::*,
    units::FromValueType,
};

// Frequencies for notes (in Hz)
const NOTE_C4: u32 = 262;
const NOTE_D4: u32 = 294;
const NOTE_E4: u32 = 330;
const NOTE_F4: u32 = 349;
const NOTE_G4: u32 = 392;
const NOTE_A4: u32 = 440;
const NOTE_B4: u32 = 494;
const NOTE_C5: u32 = 523;

// Happy Birthday melody (notes + durations)
const MELODY: &[(u32, u64)] = &[
    (NOTE_C4, 400), (NOTE_C4, 400), (NOTE_D4, 800),
    (NOTE_C4, 800), (NOTE_F4, 800), (NOTE_E4, 1600),

    (NOTE_C4, 400), (NOTE_C4, 400), (NOTE_D4, 800),
    (NOTE_C4, 800), (NOTE_G4, 800), (NOTE_F4, 1600),

    (NOTE_C4, 400), (NOTE_C4, 400), (NOTE_C5, 800),
    (NOTE_A4, 800), (NOTE_F4, 800), (NOTE_E4, 800), (NOTE_D4, 1600),

    (NOTE_B4, 400), (NOTE_B4, 400), (NOTE_A4, 800),
    (NOTE_F4, 800), (NOTE_G4, 800), (NOTE_F4, 1600),
];

fn play_tone(timer: &mut LedcTimerDriver<'static, impl LedcTimer>, driver: &mut LedcDriver<'static>, freq: u32, duration_ms: u64) -> anyhow::Result<()> {
    timer.set_frequency(freq.Hz())?;
    // Use 75% duty cycle for louder sound
    driver.set_duty((driver.get_max_duty() * 3) / 4)?;
    thread::sleep(Duration::from_millis(duration_ms));
    driver.set_duty(0)?;
    thread::sleep(Duration::from_millis(50)); // small gap
    Ok(())
}

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pin = peripherals.pins.gpio15; // choose your buzzer pin

    // Configure LEDC PWM
    let mut timer = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::new().frequency(1000.Hz()),
    )?;
    
    let mut ledc = LedcDriver::new(
        peripherals.ledc.channel0,
        &timer,
        pin,
    )?;

    println!("Playing Happy Birthday...");

    for (freq, duration) in MELODY {
        play_tone(&mut timer, &mut ledc, *freq, *duration)?;
    }

    println!("Done!");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}