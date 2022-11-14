use dmx::{self, DmxTransmitter};
use std::{thread, time};

fn main(){

    let mut dmx_port = dmx::open_serial("/dev/ttyS0").unwrap();
    
    // a typical 4-channel device, expecting RGBV values. this will set a
    // fairly bright yellow.
    let data_1 = &[0xe4, 0xe4];
    let data_2 = &[0x00, 0xe4];
    let mut index = 0;
    
    loop {
        index = index + 1;
        print!("index: {}\n", index);
        if index > 1000 {
            dmx_port.send_dmx_packet(data_2).unwrap();
        } else {
            dmx_port.send_dmx_packet(data_1).unwrap();
        }
        
        // repeat about every 51 ms. for more accurate frame timings,
        // consider using the ticktock crate.
        thread::sleep(time::Duration::new(0, 50_000_000));
    }
}