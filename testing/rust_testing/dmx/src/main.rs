use dmx::{self, DmxTransmitter};
use std::{thread, time};

fn main() {

    let mut dmx_port = dmx::open_serial("/dev/ttyS0").unwrap();
    
    // a typical 4-channel device, expecting RGBV values. this will set a
    // fairly bright yellow.
    let data = &[0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0xff];
    
    loop {
        print!("printing...");
        dmx_port.send_dmx_packet(data).unwrap();
        
        // repeat about every 51 ms. for more accurate frame timings,
        // consider using the ticktock crate.
        thread::sleep(time::Duration::new(0, 50_000_000));
    }
}