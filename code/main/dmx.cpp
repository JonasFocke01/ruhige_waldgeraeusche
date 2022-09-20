#include "Arduino.h"
#include "dmx.h"
#include "controller.h"

unsigned long dmx_timestamp;

bool strobe_mode;

int strobe_frequency;

int circle_position = 0;
bool increase_circle_position = true;

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

  DmxSimple.write( LASER_CHANNEL     , 255 );
  DmxSimple.write( LASER_CHANNEL + 11, 128 );

  dmx_timestamp = millis();
}

void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {

  // moving heads
  // map rgb to single digit number
  if ( save[1][0] == save[1][1] && save[1][0] == save[1][2] ) {
    DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 4, MV_WHITE );
  } else if ( save[1][0] > save[1][1] ) {
    if ( save[1][0] > save[1][2] ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 4, MV_RED );
    } else {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 4, MV_GREEN );
    }
  } else {
    if ( save[1][1] > save[1][2] ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 4, MV_GREEN  );
    } else {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 4, MV_DARK_BLUE );
    }
  }

  // laser
  // map rgb to single digit number
  if ( save[2][0] == save[2][1] && save[2][0] == save[2][2] ) {
    DmxSimple.write( LASER_CHANNEL + 3, LS_WHITE );
  } else if ( save[2][0] > save[2][1] ) {
    if ( save[2][0] > save[2][2] ) {
      DmxSimple.write( LASER_CHANNEL + 3, LS_RED );
    } else {
      DmxSimple.write( LASER_CHANNEL + 3, LS_GREEN );
    }
  } else {
    if ( save[2][1] > save[2][2] ) {
      DmxSimple.write( LASER_CHANNEL + 3, LS_GREEN  );
    } else {
      DmxSimple.write( LASER_CHANNEL + 3, LS_BLUE );
    }
  }

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
    if ( save[1][3] == DROP_1) {
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
    if ( save[1][3] == HYBRID_3) {
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
    if ( save[1][3] == HYBRID_1 ) {
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
    if ( save[1][3] == DROP_2 ) {
      if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
        light = 0;
        dmx_timestamp = millis();
        dimmer = 255;
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL    , random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 2, random(50, 150) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 5, random(0, 63) );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light );
      } else if ( millis() - dmx_timestamp > 150 ) {
        light = 190;
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, light  );
        DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 7, dimmer );
      }
    }
    if ( save[1][3] == HYBRID_2 || save[1][3] == HYBRID_4 ) {
      DmxSimple.write(MOVING_HEADS_RIGHT_CHANNEL, (circle_position * circle_position) / 300 - 100);
      DmxSimple.write(MOVING_HEADS_RIGHT_CHANNEL + 2, circle_position);
      if ( circle_position > 100 ) {
        increase_circle_position = false;
      }
      else if (circle_position < 1) {
        increase_circle_position = true;
      }
      if ( dmx_timestamp % 5 == 0 ) {

        if ( increase_circle_position ) {
          circle_position++;
        } else {
          circle_position--;
        }
      } 
      dmx_timestamp = millis();
    }
    if ( save[1][3] == ALL_ON ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, 255 );
    }

    // flash every light
    if ( save[1][3] == FLASH ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, 8 );
    }

    // turn every light off
    if ( save[1][3] == OFF ) {
      DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 6, 0 );
    }

    // turn every light off
    if ( save[2][3] == OFF ) {
      DmxSimple.write( LASER_CHANNEL,   0 );
    } else {
      DmxSimple.write( LASER_CHANNEL, 255 );
    }

    // write to moving heads
    DmxSimple.write( MOVING_HEADS_RIGHT_CHANNEL + 7, dimmer );

    DmxSimple.write( LASER_CHANNEL + 12, map( analogRead( POTENTIOMETER ), 0, 1024, 0, 35 )  );


  }
}

void set_strobe_mode ( bool set_to ) {
  strobe_mode = set_to;
}

void set_strobe_frequency ( int set_to) {
  strobe_frequency = set_to;
}

void randomize_laser_animation() {
  DmxSimple.write( LASER_CHANNEL + 1, random(0, 100) );
  DmxSimple.write( LASER_CHANNEL + 2, random(0, 255) );
}
