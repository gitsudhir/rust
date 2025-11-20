use esp_idf_svc::hal::gpio::{PinDriver, Output};
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

// MAX7219 registers
const REG_DECODE_MODE: u8 = 0x09;
const REG_INTENSITY: u8 = 0x0A;
const REG_SCAN_LIMIT: u8 = 0x0B;
const REG_SHUTDOWN: u8 = 0x0C;
const REG_DISPLAY_TEST: u8 = 0x0F;

// Global static for the MAX7219 driver
static MAX7219_DRIVER: OnceCell<Mutex<Option<Max7219MatrixDriver>>> = OnceCell::new();

pub struct Max7219MatrixDriver {
    pub din_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio23, Output>,
    pub clk_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio18, Output>,
    pub cs_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio5, Output>,
}

#[derive(Serialize)]
pub struct MatrixHealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct MatrixStatusResponse {
    pub status: String,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct PatternRequest {
    pub pattern: [[u8; 8]; 8],
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Max7219MatrixDriver {
    pub fn new(
        din_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio23, Output>,
        clk_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio18, Output>,
        cs_pin: PinDriver<'static, esp_idf_svc::hal::gpio::Gpio5, Output>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut driver = Self {
            din_pin,
            clk_pin,
            cs_pin,
        };
        
        // Initialize pins
        driver.cs_pin.set_high()?;
        driver.clk_pin.set_low()?;
        driver.din_pin.set_low()?;
        
        Ok(driver)
    }
    
    fn send_bit(&mut self, bit: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.din_pin.set_level(bit.into())?;
        self.clk_pin.set_high()?;
        // Small delay for timing
        esp_idf_svc::hal::delay::Ets::delay_us(1);
        self.clk_pin.set_low()?;
        esp_idf_svc::hal::delay::Ets::delay_us(1);
        Ok(())
    }
    
    fn send_byte(&mut self, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..8 {
            let bit = (data >> (7 - i)) & 1 == 1;
            self.send_bit(bit)?;
        }
        Ok(())
    }
    
    fn send_command(&mut self, reg: u8, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.cs_pin.set_low()?;
        self.send_byte(reg)?;
        self.send_byte(data)?;
        self.cs_pin.set_high()?;
        Ok(())
    }
    
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Turn off display test mode
        self.send_command(REG_DISPLAY_TEST, 0)?;
        
        // Set decode mode to no decode (matrix mode)
        self.send_command(REG_DECODE_MODE, 0)?;
        
        // Set scan limit to 8 rows
        self.send_command(REG_SCAN_LIMIT, 7)?;
        
        // Set intensity to medium brightness
        self.send_command(REG_INTENSITY, 0x07)?;
        
        // Clear all rows
        for row in 1..=8 {
            self.send_command(row, 0)?;
        }
        
        // Turn on display
        self.send_command(REG_SHUTDOWN, 1)?;
        
        Ok(())
    }
    
    pub fn display_row(&mut self, row: u8, data: u8) -> Result<(), Box<dyn std::error::Error>> {
        if row >= 1 && row <= 8 {
            self.send_command(row, data)?;
        }
        Ok(())
    }
    
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for row in 1..=8 {
            self.send_command(row, 0)?;
        }
        Ok(())
    }
    
    pub fn display_pattern(&mut self, pattern: &[[u8; 8]; 8]) -> Result<(), Box<dyn std::error::Error>> {
        for (row_index, row_data) in pattern.iter().enumerate() {
            let mut byte_data = 0u8;
            for (col_index, &bit) in row_data.iter().enumerate() {
                if bit != 0 {
                    byte_data |= 1 << col_index;
                }
            }
            self.display_row((row_index + 1) as u8, byte_data)?;
        }
        Ok(())
    }
    
    fn get_char_pattern(ch: char) -> u8 {
        match ch {
            'H' => 0b01011101,
            'E' => 0b01111001,
            'L' => 0b01001001,
            'O' => 0b00111100,
            'W' => 0b01011101, // Same as H
            'R' => 0b01111100,
            'D' => 0b00111101,
            '!' => 0b00010000,
            '0' => 0b00111100,
            '1' => 0b00011000,
            '2' => 0b01110110,
            '3' => 0b01110011,
            '4' => 0b01011001,
            '5' => 0b01101011,
            '6' => 0b01101111,
            '7' => 0b00001110,
            '8' => 0b11111110,
            '9' => 0b11011110,
            _ => 0b00000000, // Space/default
        }
    }
    
    pub fn display_ip_address(&mut self, ip_address: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Extract last octet
        if let Some(last_octet) = ip_address.split('.').last() {
            if let Ok(number) = last_octet.parse::<u32>() {
                self.display_number(number)
            } else {
                self.display_number(999) // Display error code instead
            }
        } else {
            self.display_number(999) // Display error code instead
        }
    }
    
    pub fn display_number(&mut self, number: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Convert number to string and display first 4 digits
        let num_str = number.to_string();
        let digits: Vec<char> = num_str.chars().take(4).collect();
        
        for (i, &digit) in digits.iter().enumerate() {
            let pattern = Self::get_char_pattern(digit);
            // Reverse bit order for each pattern to correct mirroring
            let reversed_pattern = ((pattern & 0x01) << 7) |
                                 ((pattern & 0x02) << 5) |
                                 ((pattern & 0x04) << 3) |
                                 ((pattern & 0x08) << 1) |
                                 ((pattern & 0x10) >> 1) |
                                 ((pattern & 0x20) >> 3) |
                                 ((pattern & 0x40) >> 5) |
                                 ((pattern & 0x80) >> 7);
            self.display_row((i + 1) as u8, reversed_pattern)?;
        }
        
        // Clear remaining rows
        for i in digits.len()..8 {
            self.display_row((i + 1) as u8, 0)?;
        }
        
        Ok(())
    }
}

// Public functions to access the global matrix driver
pub fn init_matrix_driver(driver: Max7219MatrixDriver) -> Result<(), Box<dyn std::error::Error>> {
    if MAX7219_DRIVER.set(Mutex::new(Some(driver))).is_err() {
        return Err("Failed to set MAX7219 driver".into());
    }
    Ok(())
}

pub mod api;

pub fn get_matrix_driver() -> Option<&'static OnceCell<Mutex<Option<Max7219MatrixDriver>>>> {
    Some(&MAX7219_DRIVER)
}