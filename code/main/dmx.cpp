#include "dmx.h"
#include "utilities.h"
#include "Arduino.h"

unsigned long dmx_timestamp;
int dmx_color_store[3];

void dmx_channels_init(int theme) {

  DmxSimple.usePin(DMX_DATA_PIN);

  dmx_change_theme(theme);

  dmx_timestamp = millis();
}

void dmx_change_theme(int theme) {
  if (theme == 0) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 0;
  } else if (theme == 1) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 0;
  } else if (theme == 2) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 0;
  } else if (theme == 3) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 25;
  } else if (theme == 4) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 25;
  } else if (theme == 5) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 25;
  } else if (theme == 6) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 25;
  } else if (theme == 7) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 0;
  }  
}

void dmx_write_RGBW(int device_channel, int r, int g, int b, int w) {
    DmxSimple.write(device_channel, r);
    DmxSimple.write(device_channel + 1, g);
    DmxSimple.write(device_channel + 2, b);
    DmxSimple.write(device_channel + 3, w);
}
