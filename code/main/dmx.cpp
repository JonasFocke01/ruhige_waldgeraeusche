#include "Arduino.h"
#include "dmx.h"
#include "utilities.h"

unsigned long dmx_timestamp;
int dmx_color_store[3];

void dmx_channels_init() {

  DmxSimple.usePin(DMX_DATA_PIN);

  for (int i = 0; i < 512; i++) {
    DmxSimple.write(i, 0);
  }

  dmx_timestamp = millis();
}

void dmx_main_loop(int r, int g, int b, bool strobe_mode) {

  if ( strobe_mode ) {
    for (int i = 0; i < 512; i++) {
      DmxSimple.write(i, 0);
    }
    DmxSimple.write(STROBE_CHANNEL, 255);
  } else {
    DmxSimple.write(STROBE_CHANNEL, 0);
    
    //write to rgb channel
    DmxSimple.write(RGB_CHANNEL,     r);
    DmxSimple.write(RGB_CHANNEL + 1, g);
    DmxSimple.write(RGB_CHANNEL + 2, b);
  }
}
