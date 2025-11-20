use esp_idf_svc::hal::gpio::{PinDriver, Input, Output};
use esp_idf_svc::hal::delay::Ets;
use std::time::Duration;

/// Main struct for ultrasonic sensor and TM1637 display
pub struct UltrasonicDisplay<'a> {
    trigger_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
    echo_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyInputPin, Input>,
    clk_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
    dio_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
}

impl<'a> UltrasonicDisplay<'a> {
    /// Create a new UltrasonicDisplay instance
    pub fn new(
        trigger_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
        echo_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyInputPin, Input>,
        clk_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
        dio_pin: PinDriver<'a, esp_idf_svc::hal::gpio::AnyOutputPin, Output>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            trigger_pin,
            echo_pin,
            clk_pin,
            dio_pin,
        })
    }
    
    /// Measure distance using the HC-SR04 ultrasonic sensor
    pub fn measure_distance(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        // Send a 10us pulse to the trigger pin
        self.trigger_pin.set_high()?;
        Ets::delay_us(10);
        self.trigger_pin.set_low()?;
        
        // Wait for the echo pin to go high (start of echo pulse)
        let start_time = std::time::Instant::now();
        while self.echo_pin.is_low() {
            if start_time.elapsed() > Duration::from_millis(100) {
                return Err("Timeout waiting for echo pin to go high".into());
            }
        }
        
        let start = start_time.elapsed().as_micros() as u64;
        
        // Wait for the echo pin to go low (end of echo pulse)
        let start_time = std::time::Instant::now();
        while self.echo_pin.is_high() {
            if start_time.elapsed() > Duration::from_millis(100) {
                return Err("Timeout waiting for echo pin to go low".into());
            }
        }
        
        let end = start_time.elapsed().as_micros() as u64;
        
        // Calculate distance in centimeters
        // Distance = (time / 2) * speed of sound (343 m/s)
        // Distance in cm = (time in microseconds / 2) * 0.0343
        // Simplified: Distance in cm = time in microseconds / 58
        let duration = end - start;
        let distance = (duration / 58) as u32;
        
        Ok(distance)
    }
    
    /// Send a bit to the TM1637 display
    fn tm1637_send_bit(&mut self, bit: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.dio_pin.set_level(bit.into())?;
        self.clk_pin.set_high()?;
        Ets::delay_us(5);
        self.clk_pin.set_low()?;
        Ets::delay_us(5);
        Ok(())
    }
    
    /// Send a byte to the TM1637 display
    fn tm1637_send_byte(&mut self, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..8 {
            let bit = (data >> i) & 1 == 1;
            self.tm1637_send_bit(bit)?;
        }
        
        // Wait for ACK
        self.clk_pin.set_low()?;
        self.dio_pin.set_high()?;
        Ets::delay_us(5);
        self.clk_pin.set_high()?;
        Ets::delay_us(5);
        
        // Set DIO as input to read ACK
        // For simplicity, we'll skip ACK checking
        
        self.clk_pin.set_low()?;
        Ets::delay_us(5);
        
        Ok(())
    }
    
    /// Start communication with TM1637
    fn tm1637_start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.clk_pin.set_high()?;
        self.dio_pin.set_high()?;
        Ets::delay_us(5);
        self.dio_pin.set_low()?;
        Ets::delay_us(5);
        self.clk_pin.set_low()?;
        Ets::delay_us(5);
        Ok(())
    }
    
    /// Stop communication with TM1637
    fn tm1637_stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.clk_pin.set_low()?;
        self.dio_pin.set_low()?;
        Ets::delay_us(5);
        self.clk_pin.set_high()?;
        Ets::delay_us(5);
        self.dio_pin.set_high()?;
        Ets::delay_us(5);
        Ok(())
    }
    
    /// Initialize the TM1637 display
    pub fn init_tm1637(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Command to turn on display
        self.tm1637_start()?;
        self.tm1637_send_byte(0x8f)?; // Display on command
        self.tm1637_stop()?;
        Ok(())
    }
    
    /// Display a number on the TM1637 display
    pub fn display_number(&mut self, number: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Convert number to digits
        let mut digits = [0u8; 4];
        let mut num = number;
        
        // Handle special case for 0
        if number == 0 {
            digits[0] = 0;
        } else {
            // Extract digits from right to left
            for i in (0..4).rev() {
                if num > 0 {
                    digits[i] = (num % 10) as u8;
                    num /= 10;
                } else {
                    digits[i] = 0x7f; // Blank digit
                }
            }
        }
        
        // Send data command
        self.tm1637_start()?;
        self.tm1637_send_byte(0x40)?; // Data command
        self.tm1637_stop()?;
        
        // Send digit data
        self.tm1637_start()?;
        self.tm1637_send_byte(0xc0)?; // Address command for first digit
        
        // Send each digit with 7-segment encoding
        for &digit in &digits {
            let segment_data = match digit {
                0 => 0x3f, // 0
                1 => 0x06, // 1
                2 => 0x5b, // 2
                3 => 0x4f, // 3
                4 => 0x66, // 4
                5 => 0x6d, // 5
                6 => 0x7d, // 6
                7 => 0x07, // 7
                8 => 0x7f, // 8
                9 => 0x6f, // 9
                _ => 0x00, // Blank
            };
            self.tm1637_send_byte(segment_data)?;
        }
        
        self.tm1637_stop()?;
        
        // Turn on display
        self.tm1637_start()?;
        self.tm1637_send_byte(0x8f)?; // Display on command
        self.tm1637_stop()?;
        
        Ok(())
    }
    
    /// Update display with current distance measurement
    pub fn update_display(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let distance = self.measure_distance()?;
        self.display_number(distance)?;
        Ok(())
    }
}