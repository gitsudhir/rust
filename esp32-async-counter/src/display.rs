//! Async TM1637 4-digit 7-segment display driver
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;

/// TM1637 display driver
pub struct TM1637Display<'a, 'b> {
    clk: PinDriver<'a, esp_idf_svc::hal::gpio::Gpio22, esp_idf_svc::hal::gpio::Output>,
    dio: PinDriver<'b, esp_idf_svc::hal::gpio::Gpio21, esp_idf_svc::hal::gpio::Output>,
}

impl<'a, 'b> TM1637Display<'a, 'b> {
    /// Create a new TM1637 display instance
    pub fn new(
        clk: PinDriver<'a, esp_idf_svc::hal::gpio::Gpio22, esp_idf_svc::hal::gpio::Output>,
        dio: PinDriver<'b, esp_idf_svc::hal::gpio::Gpio21, esp_idf_svc::hal::gpio::Output>,
    ) -> Self {
        Self { clk, dio }
    }

    /// Start communication with TM1637
    fn start(&mut self) {
        self.dio.set_high().ok();
        self.clk.set_high().ok();
        self.dio.set_low().ok();
        self.clk.set_low().ok();
    }

    /// Stop communication with TM1637
    fn stop(&mut self) {
        self.clk.set_low().ok();
        self.dio.set_low().ok();
        self.clk.set_high().ok();
        self.dio.set_high().ok();
    }

    /// Write a byte to TM1637
    fn write_byte(&mut self, data: u8) -> bool {
        for i in 0..8 {
            self.clk.set_low().ok();
            if (data >> i) & 1 == 1 {
                self.dio.set_high().ok();
            } else {
                self.dio.set_low().ok();
            }
            self.clk.set_high().ok();
        }
        
        // Wait for ACK
        self.clk.set_low().ok();
        self.dio.set_high().ok();
        self.clk.set_high().ok();
        
        // In a real implementation, we would check for ACK here
        // For simplicity, we'll just return true
        true
    }

    /// Display a number on the 7-segment display
    pub fn display_number(&mut self, num: u16) {
        // Start communication
        self.start();
        
        // Send command: data command setting
        self.write_byte(0x40);
        
        // Stop communication
        self.stop();
        
        // Small delay
        Ets::delay_ms(1u32);
        
        // Start communication
        self.start();
        
        // Send address command: display data from address 0xC0
        self.write_byte(0xC0);
        
        // Send 4 digits
        let digits = [
            digit_to_segments((num / 1000) % 10),
            digit_to_segments((num / 100) % 10),
            digit_to_segments((num / 10) % 10),
            digit_to_segments(num % 10)
        ];
        
        for &digit in &digits {
            self.write_byte(digit);
        }
        
        // Stop communication
        self.stop();
        
        // Small delay
        Ets::delay_ms(1u32);
        
        // Start communication
        self.start();
        
        // Send command: display control (turn on display with max brightness)
        self.write_byte(0x88 | 0x07); // 0x88 = display on, 0x07 = max brightness
        
        // Stop communication
        self.stop();
    }
}

/// Convert a digit to its 7-segment representation
fn digit_to_segments(digit: u16) -> u8 {
    // 7-segment encoding: dp g f e d c b a
    match digit {
        0 => 0x3F, // 0
        1 => 0x06, // 1
        2 => 0x5B, // 2
        3 => 0x4F, // 3
        4 => 0x66, // 4
        5 => 0x6D, // 5
        6 => 0x7D, // 6
        7 => 0x07, // 7
        8 => 0x7F, // 8
        9 => 0x6F, // 9
        _ => 0x00, // blank
    }
}