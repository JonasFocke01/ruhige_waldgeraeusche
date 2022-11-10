#include "Arduino.h"
#include "dmx.h"
#include "controller.h"

unsigned long dmx_timestamp;

bool strobe_mode;

int strobe_frequency;

int circle_position = 0;
bool increase_circle_position = true;

int scanner_position = 0;
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
  DmxSimple.write( LASER_CHANNEL + 5,  128 );
  DmxSimple.write( LASER_CHANNEL + 6,   30 );
  DmxSimple.write( LASER_CHANNEL + 7,  128 );
  DmxSimple.write( LASER_CHANNEL + 8,  100 );
  DmxSimple.write( LASER_CHANNEL + 1 ,  60 );
  DmxSimple.write( LASER_CHANNEL + 2 ,  80 );

  DmxSimple.write(STROBE_CHANNEL + 21, 255);

  // DmxSimple.write(SCANNER_CHANNEL + 2,   8);
  // DmxSimple.write(SCANNER_CHANNEL + 5, 255);

  dmx_timestamp = millis();
}

void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {


  if ( save[1][3] == STROBE  ) {
    for ( int i = 1; i < 300; i++ ) {
      if ( i < STROBE_CHANNEL ) {
        DmxSimple.write( i, 0 );
      }
    }
    DmxSimple.write( STROBE_CHANNEL + 3,  255);
    DmxSimple.write( STROBE_CHANNEL    ,  255);
    DmxSimple.write( STROBE_CHANNEL + 1, map( analogRead( POTENTIOMETER ), 1024, 0, 9, 255 ) );
    DmxSimple.write( STROBE_CHANNEL + 2, 10 );
    DmxSimple.write(STROBE_CHANNEL + 21, 255);

    DmxSimple.write( LASER_CHANNEL     , 0 );
    DmxSimple.write( LASER_CHANNEL + 11, 0 );
    return;
  } else {
    
    DmxSimple.write( LASER_CHANNEL     , 255 );
    DmxSimple.write( LASER_CHANNEL + 5,  128 );
    DmxSimple.write( LASER_CHANNEL + 6,   30 );
    DmxSimple.write( LASER_CHANNEL + 7,  128 );
    DmxSimple.write( LASER_CHANNEL + 8,  100 );
    DmxSimple.write( LASER_CHANNEL + 1 ,  60 );
    DmxSimple.write( LASER_CHANNEL + 2 ,  80 );
  
    DmxSimple.write(STROBE_CHANNEL + 21, 255);
  
    // DmxSimple.write(SCANNER_CHANNEL + 2,   8);
    // DmxSimple.write(SCANNER_CHANNEL + 5, 255);
  
    DmxSimple.write( LASER_CHANNEL     , 255 );
    DmxSimple.write( LASER_CHANNEL + 11, 128 );

    DmxSimple.write( STROBE_CHANNEL + 3,  0);
    DmxSimple.write( STROBE_CHANNEL    ,  0);
    DmxSimple.write( STROBE_CHANNEL + 1, 0 );
    DmxSimple.write( STROBE_CHANNEL + 2, 0 );
  }


  // scanner
  // map rgb to single digit number
  if ( save[1][0] == save[1][1] && save[1][0] == save[1][2] ) {
    // DmxSimple.write( SCANNER_CHANNEL + 2, MV_WHITE );
    DmxSimple.write( SCANNER_CHANNEL + 2, MV_MSL_YELLOW );
  } else if ( save[1][0] > save[1][1] ) {
    if ( save[1][0] > save[1][2] ) {
      // DmxSimple.write( SCANNER_CHANNEL + 2, MV_RED );
      DmxSimple.write( SCANNER_CHANNEL + 2, MV_MSL_RED );
    } else {
      // DmxSimple.write( SCANNER_CHANNEL + 2, MV_GREEN );
      DmxSimple.write( SCANNER_CHANNEL + 2, MV_MSL_GREEN );
    }
  } else {
    if ( save[1][1] > save[1][2] ) {
      // DmxSimple.write( SCANNER_CHANNEL + 2, MV_GREEN  );
      DmxSimple.write( SCANNER_CHANNEL + 2, MV_MSL_GREEN  );
    } else {
      // DmxSimple.write( SCANNER_CHANNEL + 2, MV_DARK_BLUE );
      DmxSimple.write( SCANNER_CHANNEL + 2, MV_MSL_DARK_BLUE );
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

  if ( save[1][3] == DROP_1) {
    dimmer = 255;
    if ( rising ) {
      scanner_position = 230;
      light = 255;
    } else {
      scanner_position = 25;
      light = 0;
    }
    if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
      rising = !rising;
      dmx_timestamp = millis();
      // DmxSimple.write( SCANNER_CHANNEL    , 0 );
      // DmxSimple.write( SCANNER_CHANNEL + 1 , scanner_position );
      // DmxSimple.write( SCANNER_CHANNEL + 5, light );
    }
  }
  if ( save[1][3] == HYBRID_3) {
    dimmer = 255;
    if ( rising ) {
      scanner_position = 230;
      light = 255;
    } else {
      scanner_position = 30;
      light = 0;
    }
    if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
      rising = !rising;
      dmx_timestamp = millis();
      // DmxSimple.write( SCANNER_CHANNEL    , scanner_position );
      // DmxSimple.write( SCANNER_CHANNEL + 1, 180 );
      // DmxSimple.write( SCANNER_CHANNEL + 6, light );
    }
  }
  if ( save[1][3] == HYBRID_1 ) {
    dimmer = 255;
    if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
      light = 0;
      dmx_timestamp = millis();
      // DmxSimple.write( SCANNER_CHANNEL    , random(30, 230) );
      // DmxSimple.write( SCANNER_CHANNEL + 1, random(30, 230) );
      // DmxSimple.write( SCANNER_CHANNEL + 5, light );
    } else if ( millis() - dmx_timestamp > 150 ) {
      light = 255;
      // DmxSimple.write( SCANNER_CHANNEL + 5, light );
    }
  }
  if ( save[1][3] == DROP_2 ) {
    if ( millis() - dmx_timestamp > analogRead( POTENTIOMETER ) ) {
      light = 0;
      dmx_timestamp = millis();
      dimmer = 255;
      // DmxSimple.write( SCANNER_CHANNEL    , random(30, 230) );
      // DmxSimple.write( SCANNER_CHANNEL + 1, random(30, 230) );
      // DmxSimple.write( SCANNER_CHANNEL + 5, light );
    } else if ( millis() - dmx_timestamp > 150 ) {
      light = 255;
  }
    // DmxSimple.write(SCANNER_CHANNEL, (circle_position * circle_position) / 300 - 100);
    // DmxSimple.write(SCANNER_CHANNEL + 1, circle_position);
    // DmxSimple.write( SCANNER_CHANNEL + 5, 255 );
    light = 255;
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
  if ( save[2][3] == BRIZZLE || save[2][3] == OFF ) { // brizzle / off

    DmxSimple.write( LASER_CHANNEL, 0 );
    DmxSimple.write( STROBE_CHANNEL    ,  255);
    DmxSimple.write( STROBE_CHANNEL + 1, map( analogRead( POTENTIOMETER ), 1024, 0, 9, 255 ) );
    DmxSimple.write( STROBE_CHANNEL + 2, 10 );
    return;
  } else {
    DmxSimple.write( STROBE_CHANNEL    ,   0);
  }
   if ( light == 8 ) {
    DmxSimple.write( LASER_CHANNEL     ,   0 );
  } else {
    DmxSimple.write( LASER_CHANNEL     , 255 );

  }
  
  if ( save[1][3] == ALL_ON ) {
    DmxSimple.write( SCANNER_CHANNEL + 5, 255 );
  }

  // flash every light
  if ( save[1][3] == FLASH ) {
    // DmxSimple.write( SCANNER_CHANNEL + 5, 255 );
  }

  // turn every light off
  if ( save[1][3] == BRIZZLE || save[1][3] == OFF ) { // brizzle /off
    DmxSimple.write( SCANNER_CHANNEL, 0 );
  } else {
    DmxSimple.write( SCANNER_CHANNEL, 255 );
  }

  // write to moving heads
  //DmxSimple.write( SCANNER_CHANNEL + 5, dimmer );

  DmxSimple.write( LASER_CHANNEL + 12, map( analogRead( POTENTIOMETER ), 0, 1024, 0, 35 )  );


}

void set_strobe_mode ( bool set_to ) {
  strobe_mode = set_to;
}

void set_strobe_frequency ( int set_to) {
  strobe_frequency = set_to;
}
