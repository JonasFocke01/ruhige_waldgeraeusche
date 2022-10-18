// use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
// use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
// use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

use ws2812_blocking_spi::Ws2812BlockingWriter;

pub fn write_leds(debug: bool) -> [u8; 40] {
    print!("WRITING LEDS");
    

    let spi: embedded_hal::blocking::spi::Write<u8>;

// setup some data to write
let mut data = [RGB8::default(); 3];
data[0] = [0xFF_u8, 0_u8, 0_u8].into();  // Full RED
data[1] = [0_u8, 0xFF_u8, 0_u8].into();  // Full GREEN
data[2] = [0_u8, 0_u8, 0xFF_u8].into();  // Full BLUE

// Create a writer
let mut leds = Ws2812BlockingWriter::new(spi);

// does the data write
leds.write(data.iter().cloned());

    // let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    // let mut rgb_values = vec![];
    //     // set first three pixels to bright red, bright green and bright blue
    //     rgb_values.push((255, 0, 0));
    //     rgb_values.push((0, 255, 0));
    //     rgb_values.push((0, 0, 0));
    //     rgb_values.push((0, 0, 0));
    //     rgb_values.push((0, 0, 0));
    //     rgb_values.push((0, 0, 255));
    //     adapter.write_rgb(&rgb_values);
    


    let test: [u8; 40] = [0; 40];
    test  
}