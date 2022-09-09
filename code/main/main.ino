
#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"
#include "dmx.h"

uint16_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_save;
uint8_t write_to_save;
bool active_lights[NUM_LIGHTS];
bool next_selection_state;

bool button_click_states[BUTTON_COUNT];

unsigned long loop_interval_length;

unsigned long debug_mode_timestamp;

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

    write_to_save = 1;
    active_save = 1;
    memset(active_lights, false, sizeof(active_lights));

    next_selection_state = false;

    debug_mode_timestamp = millis();
    
    dmx_channels_init();

    led_setup();
    
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

    memset(active_lights, true, sizeof(active_lights));

    for ( int i = 1; i < NUM_SAVES; i++ ) {
      for ( int j = 0; j < NUM_LIGHTS; j++ ) {
        saves[i][j][3] = 5;
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

    Serial.println(saves[1][0][3]);
    // Serial.print("loop took ");
    // Serial.print(millis() - loop_interval_length);
    // Serial.print("ms\n");
  
    // handle_inputs();

    write_feedback(FEEDBACK_MODE_OFF);

    led_loop( saves[active_save] );

    dmx_loop( saves[active_save] );

    loop_interval_length = millis();
}

void write_feedback(int mode) {
  if ( mode == FEEDBACK_MODE_LED || mode == FEEDBACK_MODE_DUAL ) {
    //TODO
  } else if ( ( mode == FEEDBACK_MODE_SERIAL || mode == FEEDBACK_MODE_DUAL ) && millis() - debug_mode_timestamp > 2500 ) {
    Serial.print( "SAVE 1:\n" );
    Serial.print( "    Animations:\n" );
    Serial.print( printf( "       inner_leds:         %i", saves[1][0][3] ) );
    Serial.print( printf( "       outer_leds:         %i", saves[1][1][3] ) );
    Serial.print( printf( "       stage_lights:       %i", saves[1][2][3] ) );
    Serial.print( printf( "       moving_heads_left:  %i", saves[1][3][3] ) );
    Serial.print( printf( "       moving_heads_right: %i", saves[1][4][3] ) );
    Serial.print( printf( "       blinder:            %i", saves[1][5][3] ) );
    Serial.print( printf( "       laser:              %i", saves[1][6][3] ) );
    Serial.print( printf( "       special_slot_one:   %i", saves[1][7][3] ) );
    Serial.print( printf( "       special_slot_two:   %i", saves[1][8][3] ) );
    Serial.print( printf( "       special_slot_three: %i", saves[1][9][3] ) );
    Serial.print( "    colors:\n" );
    Serial.print( printf( "       inner_leds:         %i, %i, %i", saves[0][0][0], saves[0][0][1], saves[0][0][2] ) );
    Serial.print( printf( "       outer_leds:         %i, %i, %i", saves[0][1][0], saves[0][1][1], saves[0][1][2] ) );
    Serial.print( printf( "       stage_lights:       %i, %i, %i", saves[0][2][0], saves[0][2][1], saves[0][2][2] ) );
    Serial.print( printf( "       moving_heads_left:  %i, %i, %i", saves[0][3][0], saves[0][3][1], saves[0][3][2] ) );
    Serial.print( printf( "       moving_heads_right: %i, %i, %i", saves[0][4][0], saves[0][4][1], saves[0][4][2] ) );
    Serial.print( printf( "       blinder:            %i, %i, %i", saves[0][5][0], saves[0][5][1], saves[0][5][2] ) );
    Serial.print( printf( "       laser:              %i, %i, %i", saves[0][6][0], saves[0][6][1], saves[0][6][2] ) );
    Serial.print( printf( "       special_slot_one:   %i, %i, %i", saves[0][7][0], saves[0][7][1], saves[0][7][2] ) );
    Serial.print( printf( "       special_slot_two:   %i, %i, %i", saves[0][8][0], saves[0][8][1], saves[0][8][2] ) );
    Serial.print( printf( "       special_slot_three: %i, %i, %i", saves[0][9][0], saves[0][9][1], saves[0][9][2] ) );
    Serial.print( "SAVE 1:\n" );
    Serial.print( "    Animations:\n" );
    Serial.print( printf( "       inner_leds:         %i", saves[1][0][3] ) );
    Serial.print( printf( "       outer_leds:         %i", saves[1][1][3] ) );
    Serial.print( printf( "       stage_lights:       %i", saves[1][2][3] ) );
    Serial.print( printf( "       moving_heads_left:  %i", saves[1][3][3] ) );
    Serial.print( printf( "       moving_heads_right: %i", saves[1][4][3] ) );
    Serial.print( printf( "       blinder:            %i", saves[1][5][3] ) );
    Serial.print( printf( "       laser:              %i", saves[1][6][3] ) );
    Serial.print( printf( "       special_slot_one:   %i", saves[1][7][3] ) );
    Serial.print( printf( "       special_slot_two:   %i", saves[1][8][3] ) );
    Serial.print( printf( "       special_slot_three: %i", saves[1][9][3] ) );
    Serial.print( "    colors:\n" );
    Serial.print( printf( "       inner_leds:         %i, %i, %i", saves[1][0][0], saves[1][0][1], saves[1][0][2] ) );
    Serial.print( printf( "       outer_leds:         %i, %i, %i", saves[1][1][0], saves[1][1][1], saves[1][1][2] ) );
    Serial.print( printf( "       stage_lights:       %i, %i, %i", saves[1][2][0], saves[1][2][1], saves[1][2][2] ) );
    Serial.print( printf( "       moving_heads_left:  %i, %i, %i", saves[1][3][0], saves[1][3][1], saves[1][3][2] ) );
    Serial.print( printf( "       moving_heads_right: %i, %i, %i", saves[1][4][0], saves[1][4][1], saves[1][4][2] ) );
    Serial.print( printf( "       blinder:            %i, %i, %i", saves[1][5][0], saves[1][5][1], saves[1][5][2] ) );
    Serial.print( printf( "       laser:              %i, %i, %i", saves[1][6][0], saves[1][6][1], saves[1][6][2] ) );
    Serial.print( printf( "       special_slot_one:   %i, %i, %i", saves[1][7][0], saves[1][7][1], saves[1][7][2] ) );
    Serial.print( printf( "       special_slot_two:   %i, %i, %i", saves[1][8][0], saves[1][8][1], saves[1][8][2] ) );
    Serial.print( printf( "       special_slot_three: %i, %i, %i", saves[1][9][0], saves[1][9][1], saves[1][9][2] ) );
    Serial.print( "SAVE 7:\n" );
    Serial.print( "    Animations:\n" );
    Serial.print( printf( "       inner_leds:         %i", saves[2][0][3] ) );
    Serial.print( printf( "       outer_leds:         %i", saves[2][1][3] ) );
    Serial.print( printf( "       stage_lights:       %i", saves[2][2][3] ) );
    Serial.print( printf( "       moving_heads_left:  %i", saves[2][3][3] ) );
    Serial.print( printf( "       moving_heads_right: %i", saves[2][4][3] ) );
    Serial.print( printf( "       blinder:            %i", saves[2][5][3] ) );
    Serial.print( printf( "       laser:              %i", saves[2][6][3] ) );
    Serial.print( printf( "       special_slot_one:   %i", saves[2][7][3] ) );
    Serial.print( printf( "       special_slot_two:   %i", saves[2][8][3] ) );
    Serial.print( printf( "       special_slot_three: %i", saves[2][9][3] ) );
    Serial.print( "    colors:\n" );
    Serial.print( printf( "       inner_leds:         %i, %i, %i", saves[2][0][0], saves[2][0][1], saves[2][0][2] ) );
    Serial.print( printf( "       outer_leds:         %i, %i, %i", saves[2][1][0], saves[2][1][1], saves[2][1][2] ) );
    Serial.print( printf( "       stage_lights:       %i, %i, %i", saves[2][2][0], saves[2][2][1], saves[2][2][2] ) );
    Serial.print( printf( "       moving_heads_left:  %i, %i, %i", saves[2][3][0], saves[2][3][1], saves[2][3][2] ) );
    Serial.print( printf( "       moving_heads_right: %i, %i, %i", saves[2][4][0], saves[2][4][1], saves[2][4][2] ) );
    Serial.print( printf( "       blinder:            %i, %i, %i", saves[2][5][0], saves[2][5][1], saves[2][5][2] ) );
    Serial.print( printf( "       laser:              %i, %i, %i", saves[2][6][0], saves[2][6][1], saves[2][6][2] ) );
    Serial.print( printf( "       special_slot_one:   %i, %i, %i", saves[2][7][0], saves[2][7][1], saves[2][7][2] ) );
    Serial.print( printf( "       special_slot_two:   %i, %i, %i", saves[2][8][0], saves[2][8][1], saves[2][8][2] ) );
    Serial.print( printf( "       special_slot_three: %i, %i, %i", saves[2][9][0], saves[2][9][1], saves[2][9][2] ) );

    debug_mode_timestamp = millis();
  }
}

void handle_inputs() {
  // if (digitalRead(TESTINPUT_one_in) == HIGH && button_click_states[0] == false) {
  //   button_click_states[0] = true;
  //   change_values_in_write_to_save_for_each_active_light( 256, 256, 256, 5 );
  // } else if ( digitalRead(TESTINPUT_one_in) == LOW ){
  //   button_click_states[0] = false;
  // }

  // if (digitalRead(TESTINPUT_two_in) == HIGH && button_click_states[1] == false) {
  //   button_click_states[1] = true;
  //   change_values_in_write_to_save_for_each_active_light( 256, 256, 256, 6 );
  // } else if ( digitalRead(TESTINPUT_two_in) == LOW ){
  //   button_click_states[1] = false;
  // }
  // TODO Buttons

  //this assumes a ' if pressed ' wrapping each button but is avoided for now for simplicity

  // LED colorthemes WORKS
  // rainbow
  // change_values_in_write_to_save_for_each_active_light( 10, 255, 160, 256 );
  
  // red
  // change_values_in_write_to_save_for_each_active_light( 255, 10, 10, 256 );
 
  // green
  // change_values_in_write_to_save_for_each_active_light( 10, 255, 10, 256 );
  
  // blue
  // change_values_in_write_to_save_for_each_active_light( 10, 10, 255, 256 );
  
  // random
  // change_values_in_write_to_save_for_each_active_light( 0, 0, 0, 256 );
  
  // white
  // change_values_in_write_to_save_for_each_active_light( 200, 200, 200, 256 );
  
  // purple
  // change_values_in_write_to_save_for_each_active_light( 170, 115, 225, 256 );
  
  // orange
  // change_values_in_write_to_save_for_each_active_light( 235, 180, 115, 256 );

  // Animations 20 TOTEST
  // change_values_in_write_to_save_for_each_active_light(256, 256, 256, 0); // TODO: this should be the button that is pressed

  // select each light TOTEST
  // active_lights[0] = !active_lights[0];
  // active_lights[1] = !active_lights[1];
  // active_lights[2] = !active_lights[2];
  // active_lights[3] = !active_lights[3];
  // active_lights[4] = !active_lights[4];

  // select all lights
  // for ( int i = 0; i < 5; i++ ) {
  //   active_lights[i] = next_selection_state;
  //   next_selection_state = !next_selection_state;
  // }

  // turn all lights off TOTEST
  // saves[active_save][0][0] = 0;
  // saves[active_save][0][1] = 0;
  // saves[active_save][0][2] = 0;
  // saves[active_save][1][0] = 0;
  // saves[active_save][1][1] = 0;
  // saves[active_save][1][2] = 0;
  // saves[active_save][2][0] = 0;
  // saves[active_save][2][1] = 0;
  // saves[active_save][2][2] = 0;
  // saves[active_save][3][0] = 0;
  // saves[active_save][3][1] = 0;
  // saves[active_save][3][2] = 0;
  // saves[active_save][4][0] = 0;
  // saves[active_save][4][1] = 0;
  // saves[active_save][4][2] = 0;
  // saves[active_save][6][0] = 0;
  // saves[active_save][6][1] = 0;
  // saves[active_save][6][2] = 0;

  // flash all
  //if pressed
  // if ( saves[0][5][0] == 256 ) {
    // saves[0][5][0] = saves[active_save][5][0];
    // saves[0][7][0] = saves[active_save][7][0];
    // saves[0][2][0] = saves[active_save][2][0];
    // saves[0][2][1] = saves[active_save][2][1];
    // saves[0][2][2] = saves[active_save][2][2];
    // saves[active_save][2][0] = 255;
    // saves[active_save][2][1] = 255;
    // saves[active_save][2][2] = 255;
    // saves[active_save][7][0] = 255;
    // saves[active_save][5][0] = 255;
  // }
  //else
  // if ( saves[0][5][0] != 256 ) {
    // saves[active_save][5][0] = saves[0][5][0];
    // saves[active_save][5][0] = saves[0][5][0];
    // saves[active_save][2][0] = saves[0][2][0];
    // saves[active_save][2][1] = saves[0][2][1];
    // saves[active_save][2][2] = saves[0][2][2];
    // saves[0][5][0] = 256;
    // saves[0][2][0] = 256;
    // saves[0][5][7] = 256;
  // }

  // flash blinder
  //if pressed
  // if ( saves[0][5][0] == 256 ) {
    // saves[0][5][0] = saves[active_save][5][0];
    // saves[active_save][5][0] = 255;
  // }
  //else
  // if ( saves[0][5][0] != 256 ) {
    // saves[active_save][5][0] = saves[0][5][0];
    // saves[0][5][0] = 256;
  // }

  // flash special one
  //if pressed
  // if ( saves[0][7][0] == 256 ) {
    // saves[0][7][0] = saves[active_save][7][0];
    // saves[active_save][7][0] = 255;
  // }
  //else
  // if ( saves[0][7][0] != 256 ) {
    // saves[active_save][5][0] = saves[0][7][0];
    // saves[0][5][7] = 256;
  // }
  
  // flash stage
  //if pressed
  // if ( saves[0][2][0] == 256 ) {
    // saves[0][2][0] = saves[active_save][2][0];
    // saves[0][2][1] = saves[active_save][2][1];
    // saves[0][2][2] = saves[active_save][2][2];
    // saves[active_save][2][0] = 255;
    // saves[active_save][2][1] = 255;
    // saves[active_save][2][2] = 255;
  // }
  //else
  // if ( saves[0][2][0] != 256 ) {
    // saves[active_save][2][0] = saves[0][2][0];
    // saves[active_save][2][1] = saves[0][2][1];
    // saves[active_save][2][2] = saves[0][2][2];
    // saves[0][2][0] = 256;
  // }

  // strobe 
  // set_strobe_mode( digitalRead( STROBE_TOGGLE ) );

  // strobe frequency level 
  // set_strobe_frequency( analogRead( STROBE_FREQUENCY_POTENTIOMETER ) );

  // blinder level 
  // saves[write_to_save][5][0] = analogRead(BLINDER_POTENTIOMETER);
  
  // special_lights level 
  // saves[write_to_save][7][0] = analogRead(SPECIAL_SLOT_ONE_POTENTIOMETER);
  // saves[write_to_save][8][0] = analogRead(SPECIAL_SLOT_TWO_POTENTIOMETER);
  // saves[write_to_save][9][0] = analogRead(SPECIAL_SLOT_THREE_POTENTIOMETER);

  // stage lights rgb 
  // saves[write_to_save][2][0] = analogRead(RED_POTENTIOMETER);
  // saves[write_to_save][2][1] = analogRead(GREEN_POTENTIOMETER);
  // saves[write_to_save][2][2] = analogRead(BLUE_POTENTIOMETER);

  // preset switches
  // if ( active_save == write_to_save ) {
    // write_to_save = 0;// TODO: this should be the pressed button (0-5)
  // } else {
    // write_to_save = active_save;
  // }

  // preset apply 
  // active_save = 0;// TODO: this should be the pressed button (0-5)

  // Beat: ( beat + speed[3] + switcher )
  
  // Quick animations 5
  
  // master 1 ?
}
