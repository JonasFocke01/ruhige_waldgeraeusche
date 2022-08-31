#include "Arduino.h"
#include "dmx.h"
#include "utilities.h"
#include "controller.h"

unsigned long dmx_timestamp;

bool strobe_mode = false;

void dmx_channels_init() {

  DmxSimple.usePin(DMX_DATA_PIN);

  for (int i = 1; i < 512; i++) {
    DmxSimple.write(i, 0);
  }

  dmx_timestamp = millis();
}

void dmx_loop(uint8_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {

  if ( strobe_mode ) {
    for (int i = 1; i < 512; i++) {
      if ( i = STROBE_CHANNEL ) {
        DmxSimple.write(i, 255);
      } else {
        DmxSimple.write(i, 0);
      }
    }
    set_strobe_mode( false );
  } else {
      
    // write to stage lights
  
    // write to ambience
  
    // write to moving heads right

    // write to moving heads left
  
    // write to laser
  
    // write to special channels
  
  }
}

void set_strobe_mode ( bool set_to ) {
  strobe_mode = set_to;
}
