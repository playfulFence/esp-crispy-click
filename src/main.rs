#![no_std]
#![no_main]

use esp32_hal::{
    clock::ClockControl,
    pac::Peripherals,
    gpio_types::*,
    gpio::*,
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

/* Display and graphics */
use ili9341::{DisplaySize240x320, Ili9341, Orientation};

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_graphics::image::Image;
use embedded_graphics::geometry::*;
use embedded_graphics::draw_target::DrawTarget;
use embedded_hal;


use profont::{PROFONT_24_POINT, PROFONT_18_POINT};

use xtensa_lx_rt::entry;

use esp_println::println;
use esp_backtrace as _;

const textStyle : TextStyle = TextStyleBuilder::new()
    .alignment(embedded_graphics::text::Alignment::Center)
    .baseline(embedded_graphics::text::Baseline::Middle)
    .build();


/* Some stuff for correct orientation and color on ILI9341 */
pub enum KalugaOrientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeVericallyFlipped,
    LandscapeFlipped,
}

impl ili9341::Mode for KalugaOrientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0,
            Self::LandscapeVericallyFlipped => 0x20,
            Self::Landscape => 0x20 | 0x40,
            Self::PortraitFlipped => 0x80 | 0x40,
            Self::LandscapeFlipped => 0x80 | 0x20 | 0x08,
        }
    }

    fn is_landscape(&self) -> bool {
        matches!(self, Self::Landscape | Self::LandscapeFlipped | Self::LandscapeVericallyFlipped)
    }
}


#[derive(Copy, Clone, PartialEq)]
pub enum Event {
    Pressed,
    Released,
    Nothing,
}
pub struct Button<T> {
    button: T,
    pressed: bool,
}
impl<T: ::embedded_hal::digital::v2::InputPin<Error = core::convert::Infallible>> Button<T> {
    pub fn new(button: T) -> Self {
        Button {
            button,
            pressed: true,
        }
    }
    pub fn check(&mut self){
        self.pressed = !self.button.is_low().unwrap();
    }

    pub fn poll(&mut self, delay :&mut Delay) -> Event {
        let pressed_now = !self.button.is_low().unwrap();
        if !self.pressed  &&  pressed_now
        {
            delay.delay_ms(30 as u32);
            self.check();
            if !self.button.is_low().unwrap() {
                Event::Pressed
            }
            else {
                Event::Nothing
            }
        }
        else if self.pressed && !pressed_now{
            delay.delay_ms(30 as u32);
            self.check();
            if self.button.is_low().unwrap()
            {
                Event::Released
            }
            else {
                Event::Nothing
            }
        }
        else{
            Event::Nothing
        }
        
    }
}

 


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut system = peripherals.DPORT.split();

    let mut clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();


    println!("About to initialize the SPI LED driver ILI9341");
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    

    /* Set corresponding pins */
 
    let mosi = io.pins.gpio23;
    let cs = io.pins.gpio15;
    let rst = io.pins.gpio4;
    let dc = io.pins.gpio2;
    let sck = io.pins.gpio18;
    let miso = io.pins.gpio8;
    let backlight = io.pins.gpio17;

    
    /* Then set backlight (set_low() - display lights up when signal is in 0, set_high() - opposite case(for example.)) */
    let mut backlight = backlight.into_push_pull_output();
    backlight.set_high().unwrap();

    let spi = spi::Spi::new(
        peripherals.SPI3,
        sck,
        mosi,
        miso,
        cs,
        400u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &mut clocks,
    );

    let di = SPIInterfaceNoCS::new(spi, dc.into_push_pull_output());
    let reset = rst.into_push_pull_output();
    let mut delay = Delay::new(&clocks);

    let mut display = Ili9341::new(di, reset, &mut delay, KalugaOrientation::LandscapeFlipped, DisplaySize240x320).unwrap();
    
    println!("Display initialized");

    display.clear(Rgb565::WHITE).unwrap(); 

    let mut button_green = Button::new(io.pins.gpio21.into_pull_up_input());
    let mut button_blue  = Button::new(io.pins.gpio13.into_pull_up_input());


 
    Text::with_text_style("Press GREEN button",
            display.bounding_box().center() - Size::new(0, 35), 
            MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
            textStyle,
    )
    .draw(&mut display)
    .unwrap();


    Text::with_text_style("Press BLUE button",
            display.bounding_box().center() + Size::new(0, 25), 
            MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
            textStyle,
    )
    .draw(&mut display)
    .unwrap();

    let mut green_cnt = 0;
    let mut blue_cnt = 0;

    let mut last_pressed_blue : bool = false;

    loop {
        if let Event::Pressed = button_green.poll(&mut delay)
        {
            green_cnt += 1;

            
            Rectangle::with_center(display.bounding_box().center() - Size::new(0, 30), Size::new(270, 30))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::WHITE)
                    .stroke_color(Rgb565::WHITE)
                    .stroke_width(1)
                    .build(),
            )
            .draw(&mut display);

                Text::with_text_style("Green button pressed!",
                    display.bounding_box().center() - Size::new(0, 35), 
                    MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::CSS_GREEN),
                    textStyle,
                )
                .draw(&mut display)
                .unwrap();

            println!("Green! (x{})", green_cnt);

            if last_pressed_blue {
                Rectangle::with_center(display.bounding_box().center() + Size::new(0, 30), Size::new(270, 30))
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .fill_color(Rgb565::WHITE)
                            .stroke_color(Rgb565::WHITE)
                            .stroke_width(1)
                            .build(),
                    )
                    .draw(&mut display);
                

                Text::with_text_style("Press BLUE button",
                    display.bounding_box().center() + Size::new(0, 25), 
                    MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
                    textStyle,
                )
                .draw(&mut display)
                .unwrap();
            }
            last_pressed_blue = false;
        }
        if let Event::Pressed = button_blue.poll(&mut delay)
        {
            blue_cnt += 1;

                Rectangle::with_center(display.bounding_box().center() + Size::new(0, 30), Size::new(270, 30))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb565::WHITE)
                        .stroke_color(Rgb565::WHITE)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut display);
            
                Text::with_text_style("Blue button pressed!",
                            display.bounding_box().center() + Size::new(0, 25), 
                            MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLUE),
                            textStyle,
                )
                .draw(&mut display)
                .unwrap();
            
            println!("Blue! (x{})", blue_cnt);

            if !last_pressed_blue {

                Rectangle::with_center(display.bounding_box().center() - Size::new(0, 30), Size::new(270, 30))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb565::WHITE)
                        .stroke_color(Rgb565::WHITE)
                        .stroke_width(1)
                        .build(),
                )
                .draw(&mut display);

                Text::with_text_style("Press GREEN button",
                    display.bounding_box().center() - Size::new(0, 35), 
                    MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::BLACK),
                    textStyle,
                )
                .draw(&mut display)
                .unwrap();
            }
            last_pressed_blue = true;
        }
    }
}