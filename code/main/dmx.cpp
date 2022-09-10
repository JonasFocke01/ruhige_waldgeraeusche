#include "Arduino.h"
#include "dmx.h"
#include "controller.h"

unsigned long dmx_timestamp;

bool strobe_mode;

int strobe_frequency;

void dmx_channels_init() {

  DmxSimple.usePin(DMX_DATA_PIN);

  for (int i = 1; i < 512; i++) {
    DmxSimple.write(i, 0);
  }

  strobe_frequency = 0;

  strobe_mode = false;
  
  dmx_timestamp = millis();
}

void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {

  if ( strobe_mode ) {
    for (int i = 1; i < 512; i++) {
      if ( i == STROBE_CHANNEL ) {
        DmxSimple.write(i, 255);
      } else {
        DmxSimple.write(i, 0);
      }
    }
    set_strobe_mode( false );
  } else {

    trigger_animations(save);
      
    // write to stage lights
    DmxSimple.write(STAGE_LIGHTS_CHANNEL    , save[2][0]);
    DmxSimple.write(STAGE_LIGHTS_CHANNEL + 1, save[2][1]);
    DmxSimple.write(STAGE_LIGHTS_CHANNEL + 2, save[2][2]);
  
    // write to ambience
    DmxSimple.write(AMBIENCE_LIGHT_CHANNEL, save[5][0]);
    
    // write to moving heads right
    DmxSimple.write(MOVING_HEADS_RIGHT_CHANNEL    , save[4][0]);
    DmxSimple.write(MOVING_HEADS_RIGHT_CHANNEL + 1, save[4][1]);
    DmxSimple.write(MOVING_HEADS_RIGHT_CHANNEL + 2, save[4][2]);
    
    
    // write to moving heads left
    DmxSimple.write(MOVING_HEADS_LEFT_CHANNEL    , save[3][0]);
    DmxSimple.write(MOVING_HEADS_LEFT_CHANNEL + 1, save[3][1]);
    DmxSimple.write(MOVING_HEADS_LEFT_CHANNEL + 2, save[3][2]);
  
    // write to laser
    DmxSimple.write(LASER_CHANNEL, save[6][0]);
  
    // write to special channels
    DmxSimple.write(SPECIAL_SLOT_ONE_CHANNEL  , save[7][0]);
    DmxSimple.write(SPECIAL_SLOT_TWO_CHANNEL  , save[8][0]);
    DmxSimple.write(SPECIAL_SLOT_THREE_CHANNEL, save[9][0]);
  }
}

void trigger_animations(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {
  
}

void set_strobe_mode ( bool set_to ) {
  strobe_mode = set_to;
}

void set_strobe_frequency ( int set_to) {
  strobe_frequency = set_to;
}
