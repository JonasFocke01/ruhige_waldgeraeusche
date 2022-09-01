#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"
#include "dmx.h"

uint16_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_save;
uint8_t write_to_save;
bool active_lights[NUM_LIGHTS];
bool next_selection_state;

unsigned long button_timestamps[BUTTON_COUNT];

unsigned long loop_interval_length;

void change_values_in_write_to_save_for_each_active_light(int r, int g, int b, int animation) {
  for (int i = 0; i < NUM_LIGHTS; i++ ) {
    if ( active_lights[i] ) {
      if ( r != 256 )         saves[write_to_save][i][0] = r;
      if ( g != 256 )         saves[write_to_save][i][1] = b;
      if ( b != 256 )         saves[write_to_save][i][2] = g;
      if ( animation != 256 ) saves[write_to_save][i][3] = animation;
    }
  }
}

void setup() {
    
    Serial.begin(9600);

    Serial.print("booting...");

    randomSeed(analogRead(9)); // 9 is an unconnected analog pin

    write_to_save = 0;
    active_save = 0;
    memset(active_lights, false, sizeof(active_lights));

    next_selection_state = false;
    
    dmx_channels_init();

    led_setup();

    for (int i=0;i<BUTTON_COUNT; i++) button_timestamps[i] = 1;

    // button_timestamp = millis();
    
    pinMode(TESTINPUT, INPUT);
    
    Serial.print(" ok\n");

    Serial.print("setting default color_themes...");

    for ( int i = 0; i < NUM_SAVES; i++ ) {
      for ( int j = 0; j < NUM_LIGHTS; j++ ) {
        saves[i][j][0] = random(0, 255);
        saves[i][j][1] = random(0, 255);
        saves[i][j][2] = random(0, 255);
      }
    }
    
    Serial.print("ok\n");

    Serial.print("preparing animations...");

    for ( int i = 0; i < NUM_SAVES; i++ ) {
      for ( int j = 0; j < NUM_LIGHTS; j++ ) {
        saves[i][j][3] = i + 4;
      }
    }

    Serial.print("ok\n");
    
    Serial.print("\nbooting complete in ");
    Serial.print(millis());
    Serial.print("ms\n");
    Serial.print("----------------\n");

    loop_interval_length = millis();
}

void loop(){

    Serial.print("loop took ");
    Serial.print(millis() - loop_interval_length);
    Serial.print("ms\n");
  
    handle_inputs();

    led_loop( saves[active_save] );

    dmx_loop( saves[active_save] );

    loop_interval_length = millis();
}

void handle_inputs() {
  //if (digitalRead(TESTINPUT) == LOW && millis() - button_timestamp > DETECT_CLICKS_LENGTH_IN_MS) {
  //  active_save += 1;
  //  button_timestamp = millis();
  //}

  // TODO Buttons

  //this assumes a ' if pressed ' wrapping each button but is avoided for now for simplicity

  // LED colorthemes 8
  // rainbow
  change_values_in_write_to_save_for_each_active_light( 10, 255, 160, 256 );
  
  // red
  change_values_in_write_to_save_for_each_active_light( 255, 10, 10, 256 );
 
  // green
  change_values_in_write_to_save_for_each_active_light( 10, 255, 10, 256 );
  
  // blue
  change_values_in_write_to_save_for_each_active_light( 10, 10, 255, 256 );
  
  // random
  change_values_in_write_to_save_for_each_active_light( 0, 0, 0, 256 );
  
  // white
  change_values_in_write_to_save_for_each_active_light( 200, 200, 200, 256 );
  
  // purple
  change_values_in_write_to_save_for_each_active_light( 170, 115, 225, 256 );
  
  // orange
  change_values_in_write_to_save_for_each_active_light( 235, 180, 115, 256 );

  // Animations 20

  // select each light 5
  for ( int i = 0; i < 5; i++ ) {
    if ( false ) { // button i is pressed
      active_lights[i] = !active_lights[i];
    }
  }

  // select all lights 1

  // turn all lights on
  saves[active_save][0][0] = 0;
  saves[active_save][0][1] = 0;
  saves[active_save][0][2] = 0;
  saves[active_save][1][0] = 0;
  saves[active_save][1][1] = 0;
  saves[active_save][1][2] = 0;
  saves[active_save][2][0] = 0;
  saves[active_save][2][1] = 0;
  saves[active_save][2][2] = 0;
  saves[active_save][3][0] = 0;
  saves[active_save][3][1] = 0;
  saves[active_save][3][2] = 0;
  saves[active_save][4][0] = 0;
  saves[active_save][4][1] = 0;
  saves[active_save][4][2] = 0;
  saves[active_save][6][0] = 0;
  saves[active_save][6][1] = 0;
  saves[active_save][6][2] = 0;

  // flash all
  //if pressed
  if ( saves[6][5][0] == 256 ) {
    saves[6][5][0] = saves[active_save][5][0];
    saves[6][7][0] = saves[active_save][7][0];
    saves[6][2][0] = saves[active_save][2][0];
    saves[6][2][1] = saves[active_save][2][1];
    saves[6][2][2] = saves[active_save][2][2];
    saves[active_save][2][0] = 255;
    saves[active_save][2][1] = 255;
    saves[active_save][2][2] = 255;
    saves[active_save][7][0] = 255;
    saves[active_save][5][0] = 255;
  }
  //else
  if ( saves[6][5][0] != 256 ) {
    saves[active_save][5][0] = saves[6][5][0];
    saves[active_save][5][0] = saves[6][5][0];
    saves[active_save][2][0] = saves[6][2][0];
    saves[active_save][2][1] = saves[6][2][1];
    saves[active_save][2][2] = saves[6][2][2];
    saves[6][5][0] = 256;
    saves[6][2][0] = 256;
    saves[6][5][7] = 256;
  }

  // flash blinder
  //if pressed
  if ( saves[6][5][0] == 256 ) {
    saves[6][5][0] = saves[active_save][5][0];
    saves[active_save][5][0] = 255;
  }
  //else
  if ( saves[6][5][0] != 256 ) {
    saves[active_save][5][0] = saves[6][5][0];
    saves[6][5][0] = 256;
  }

  // flash special one
  //if pressed
  if ( saves[6][7][0] == 256 ) {
    saves[6][7][0] = saves[active_save][7][0];
    saves[active_save][7][0] = 255;
  }
  //else
  if ( saves[6][7][0] != 256 ) {
    saves[active_save][5][0] = saves[6][7][0];
    saves[6][5][7] = 256;
  }
  
  // flash stage
  //if pressed
  if ( saves[6][2][0] == 256 ) {
    saves[6][2][0] = saves[active_save][2][0];
    saves[6][2][1] = saves[active_save][2][1];
    saves[6][2][2] = saves[active_save][2][2];
    saves[active_save][2][0] = 255;
    saves[active_save][2][1] = 255;
    saves[active_save][2][2] = 255;
  }
  //else
  if ( saves[6][2][0] != 256 ) {
    saves[active_save][2][0] = saves[6][2][0];
    saves[active_save][2][1] = saves[6][2][1];
    saves[active_save][2][2] = saves[6][2][2];
    saves[6][2][0] = 256;
  }

  // strobe 
  set_strobe_mode( digitalRead( STROBE_TOGGLE ) ); // TODO: STROBE_TOGGLE pin

  // strobe frequency level 
  set_strobe_frequency( analogRead( STROBE_FREQUENCY_POTENTIOMETER ) ); // TODO: STROBE_FREQUENCY_POTENTIOMETER pin

  // blinder level 
  saves[write_to_save][5][0] = analogRead(BLINDER_POTENTIOMETER);     // TODO: BLINDER_POTENTIOMETER pin
  
  // special_lights level 
  saves[write_to_save][7][0] = analogRead(SPECIAL_SLOT_ONE_POTENTIOMETER);     // TODO: SPECIAL_SLOT_ONE_POTENTIOMETER pin
  saves[write_to_save][8][0] = analogRead(SPECIAL_SLOT_TWO_POTENTIOMETER);     // TODO: SPECIAL_SLOT_TWO_POTENTIOMETER pin
  saves[write_to_save][9][0] = analogRead(SPECIAL_SLOT_THREE_POTENTIOMETER);   // TODO: SPECIAL_SLOT_THREE_POTENTIOMETER pin

  // stage lights rgb 
  saves[write_to_save][2][0] = analogRead(RED_POTENTIOMETER);   // TODO: RED_POTENTIOMETER pin
  saves[write_to_save][2][1] = analogRead(GREEN_POTENTIOMETER); // TODO: GREEN_POTENTIOMETER pin
  saves[write_to_save][2][2] = analogRead(BLUE_POTENTIOMETER);  // TODO: BLUE_POTENTIOMETER pin

  // preset switches
  if ( active_save == write_to_save ) {
    write_to_save = 0;// TODO: this should be the pressed button (0-5)
  } else {
    write_to_save = active_save;
  }

  // preset apply 
  active_save = 0;// TODO: this should be the pressed button (0-5)

  // Beat: ( beat + speed[3] + switcher )
  
  // Quick animations 5
  
  // master 1 ?
}
