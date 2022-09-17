#include "Arduino.h"
#include "dmx.h"
#include "controller.h"

unsigned long dmx_timestamp;

bool strobe_mode;

int strobe_frequency;

int moving_heads_position = 0;
bool rising = false;
int light = 0;
int dimmer = 255;

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
    dimmer = 255;
    for (int i = 1; i < 512; i++) {
      if ( i == STROBE_CHANNEL ) {
        DmxSimple.write(i, 255);
      } else {
        DmxSimple.write(i, 0);
      }
    }
    set_strobe_mode( false );
  } else {
    if ( save[4][3] == HYBRID_1 || save[4][3] == HYBRID_2 ) {
      dimmer = 255;
      if ( rising ) {
        moving_heads_position = 180;
        light = 8;
      } else {
        moving_heads_position = 30;
        light = 0;
      }
      if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
        rising = !rising;
        dmx_timestamp = millis();
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL    , 0 );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 2, moving_heads_position );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      }
    }
    if ( save[4][3] == HYBRID_3 || save[4][3] == HYBRID_4 ) {
      dimmer = 255;
      if ( rising ) {
        moving_heads_position = 115;
        light = 8;
      } else {
        moving_heads_position = 60;
        light = 0;
      }
      if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
        rising = !rising;
        dmx_timestamp = millis();
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL    , moving_heads_position );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 2, 180 );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      }
    }
    if ( save[4][3] == DROP_1 ) {
      dimmer = 255;
      if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
        light = 0;
        dmx_timestamp = millis();
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL    , random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 2, random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 5, random(0, 63) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      } else if ( millis() - dmx_timestamp > 150 ) {
        light = 190;
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      }
    }
    if ( save[4][3] == DROP_2 ) {
      if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
        light = 0;
        dmx_timestamp = millis();
        dimmer = 255;
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL    , random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 2, random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 5, random(0, 63) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      } else if ( millis() - dmx_timestamp > 150 ) {
        if ( dimmer > 10 ) {
          dimmer -= 10;
        }
        light = 190;
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light  );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 7, dimmer );
      }
    }
    // flash every light
    if ( save[4][3] == FLASH ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, 8 );
    }
  
    // turn every light off
    if ( save[4][3] == OFF ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, 0 );
    }

    // write to stage lights
    DmxSimple.write(STAGE_LIGHTS_CHANNEL    , save[2][0]);
    DmxSimple.write(STAGE_LIGHTS_CHANNEL + 1, save[2][1]);
    DmxSimple.write(STAGE_LIGHTS_CHANNEL + 2, save[2][2]);

    // write to ambience
    DmxSimple.write(AMBIENCE_LIGHT_CHANNEL, save[5][0]);

    // write to moving heads left
    DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 7, dimmer );

    // write to laser
    DmxSimple.write(LASER_CHANNEL, save[6][0]);

    // write to special channels
    DmxSimple.write(SPECIAL_SLOT_ONE_CHANNEL  , save[7][0]);
    DmxSimple.write(SPECIAL_SLOT_TWO_CHANNEL  , save[8][0]);
    DmxSimple.write(SPECIAL_SLOT_THREE_CHANNEL, save[9][0]);

    
  }
}

void set_strobe_mode ( bool set_to ) {
  strobe_mode = set_to;
}

void set_strobe_frequency ( int set_to) {
  strobe_frequency = set_to;
}
