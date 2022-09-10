#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"
#include "dmx.h"

uint16_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_save;
uint8_t write_to_save;
bool active_lights[NUM_LIGHTS];

bool button_click_states            [BUTTON_ISLANDS][BUTTON_ROWS];
bool button_click_prevent_ghosting  [BUTTON_ISLANDS][BUTTON_ROWS];

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

    for (int i = 0; i < NUM_LIGHTS; i++) {
      for (int j = 0; j < LIGHT_SAVE_SPACE; j++) {
        saves[0][i][j] = 256;
      }
    }

    debug_mode_timestamp = 1000;
    
    dmx_channels_init();

    led_setup();

    
    Serial.print(" ok\n");

    Serial.print("setting default color_themes...");

    for ( int i = 1; i < NUM_SAVES; i++ ) {
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
    
    for ( int i = 0; i < NUM_LIGHTS; i++ ) {
      saves[6][i][3] = FLASH;
    }

    for ( int i = 0; i < NUM_LIGHTS; i++ ) {
      saves[7][i][3] = OFF;
    }
    
    Serial.print("ok\n");

    Serial.print("preparing intputs...");

    pinMode(ISLAND_ONE  , OUTPUT);
    pinMode(ISLAND_TWO  , OUTPUT);
    pinMode(ISLAND_THREE, OUTPUT);
    pinMode(ISLAND_FOUR , OUTPUT);
    digitalWrite(ISLAND_ONE  , LOW);
    digitalWrite(ISLAND_TWO  , LOW);
    digitalWrite(ISLAND_THREE, LOW);
    digitalWrite(ISLAND_FOUR , LOW);
    
    pinMode(ROW_ONE  , INPUT_PULLUP);
    pinMode(ROW_TWO  , INPUT_PULLUP);
    pinMode(ROW_THREE, INPUT_PULLUP);
    pinMode(ROW_FOUR , INPUT_PULLUP);
    pinMode(ROW_FIVE , INPUT_PULLUP);
    pinMode(ROW_SIX  , INPUT_PULLUP);
    pinMode(ROW_SEVEN, INPUT_PULLUP);
    pinMode(ROW_EIGHT, INPUT_PULLUP);
    pinMode(ROW_NINE , INPUT_PULLUP);
    
    Serial.print("ok\n");
    
    Serial.print("\nbooting complete in ");
    Serial.print(millis());
    Serial.print("ms\n");
    Serial.print("----------------\n");

    loop_interval_length = millis();
}

void loop(){

    // Serial.print("loop took ");
    // Serial.print(millis() - loop_interval_length);
    // Serial.print("ms\n");
  
    handle_inputs();

    write_feedback(FEEDBACK_MODE_OFF);

    led_loop( saves[active_save] );

    dmx_loop( saves[active_save] );

    loop_interval_length = millis();
}

void read_buttons() {
  for (int i = 0; i < BUTTON_ISLANDS; i++) {
    if (i > 0) { digitalWrite(ISLAND_ONE + i - 1, HIGH); }
    digitalWrite(ISLAND_ONE + i, LOW);
    for (int j = 0; j < BUTTON_ROWS; j++) {
      if (!digitalRead(j + ROW_ONE)) { button_click_states[i][j] = true; } else { button_click_states[i][j] = false; }
      if ( button_click_states[i][j] ) {
        Serial.print("    ");
        Serial.print(i);
        Serial.print("/");
        Serial.print(j);
        Serial.print("\n");
      }
    }
  }
}

void handle_inputs() {

  // read buttons into button_click_states array
  read_buttons();

 // flash
  while (button_click_states[2][5]) {
    for (int i = 0; i < NUM_LIGHTS; i++) {
      saves[6][i][0] = saves[active_save][i][0];
      saves[6][i][1] = saves[active_save][i][1];
      saves[6][i][2] = saves[active_save][i][2];
    }
    led_loop( saves[6] );
    dmx_loop( saves[6] );
      read_buttons();
  }

  // mute
  while (button_click_states[1][5]) {
      led_loop( saves[7] );
      dmx_loop( saves[7] );
      read_buttons();
  }

  //strobo
  while (button_click_states[0][5]) {
      led_loop( saves[7] );
      dmx_loop( saves[7] );
      read_buttons();
  }

  // active lights to red
  if (button_click_states[0][8] && button_click_prevent_ghosting[0][8] == false) {
      change_values_in_write_to_save_for_each_active_light( 255, 10, 10, 256 );
      button_click_prevent_ghosting[0][8] = true;
  } else if ( !button_click_states[0][8] ){
    button_click_prevent_ghosting[0][8] = false;
  }

  // active lights to green
  if (button_click_states[1][8] && button_click_prevent_ghosting[1][8] == false) {
      change_values_in_write_to_save_for_each_active_light( 10, 255, 10, 256 );
      button_click_prevent_ghosting[1][8] = true;
  } else if ( !button_click_states[1][8] ){
    button_click_prevent_ghosting[1][8] = false;
  }

  // active lights to blue
  if (button_click_states[2][8] && button_click_prevent_ghosting[2][8] == false) {
      change_values_in_write_to_save_for_each_active_light( 10, 10, 255, 256 );
      button_click_prevent_ghosting[2][8] = true;
  } else if ( !button_click_states[2][8] ){
    button_click_prevent_ghosting[2][8] = false;
  }

  // select inner_leds
  if (button_click_states[0][1] && button_click_prevent_ghosting[0][1] == false) {
    active_lights[0] = !active_lights[0];
    button_click_prevent_ghosting[0][1] = true;
  } else if ( !button_click_states[0][1] ){
    button_click_prevent_ghosting[0][1] = false;
  }

  // select outer_leds
  if (button_click_states[1][1] && button_click_prevent_ghosting[1][1] == false) {
      active_lights[1] = !active_lights[1];
      button_click_prevent_ghosting[1][1] = true;
  } else if ( !button_click_states[1][1] ){
    button_click_prevent_ghosting[1][1] = false;
  }

  // select light 3
  if (button_click_states[2][1] && button_click_prevent_ghosting[2][1] == false) {
      active_lights[2] = !active_lights[2];
      button_click_prevent_ghosting[2][1] = true;
  } else if ( !button_click_states[2][1] ){
    button_click_prevent_ghosting[2][1] = false;
  }

  // select all lights
  if (button_click_states[1][4] && button_click_prevent_ghosting[1][4] == false) {
      for (int i = 0; i < NUM_LIGHTS; i++) {
        active_lights[i] = true;
      }
      button_click_prevent_ghosting[1][4] = true;
  } else if ( !button_click_states[1][4] ){
    button_click_prevent_ghosting[1][4] = false;
  }

  // deselect all lights
  if (button_click_states[2][4] && button_click_prevent_ghosting[2][4] == false) {
      for (int i = 0; i < NUM_LIGHTS; i++) {
        active_lights[i] = false;
      }
      button_click_prevent_ghosting[2][4] = true;
  } else if ( !button_click_states[2][4] ){
    button_click_prevent_ghosting[2][4] = false;
  }

  // turn inner leds off
  if (button_click_states[0][0] && button_click_prevent_ghosting[0][0] == false) {
      saves[1][0][3] = 255;
      button_click_prevent_ghosting[0][0] = true;
  } else if ( !button_click_states[0][0] ){
      button_click_prevent_ghosting[0][0] = false;
  }

  // turn outer leds off
  if (button_click_states[1][0] && button_click_prevent_ghosting[1][0] == false) {
      saves[1][1][3] = 255;
      button_click_prevent_ghosting[1][0] = true;
  } else if ( !button_click_states[1][0] ){
      button_click_prevent_ghosting[1][0] = false;
  }

  // turn light three off
  if (button_click_states[2][0] && button_click_prevent_ghosting[2][0] == false) {
      saves[1][2][3] = 255;
      button_click_prevent_ghosting[2][0] = true;
  } else if ( !button_click_states[2][0] ){
      button_click_prevent_ghosting[2][0] = false;
  }

  //animation HYBRID_1
  if (button_click_states[0][2] && button_click_prevent_ghosting[0][2] == false) {
      change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_1);
      button_click_prevent_ghosting[0][2] = true;
  } else if ( !button_click_states[0][2] ){
      button_click_prevent_ghosting[0][2] = false;
  }

  //animation HYBRID_2
  if (button_click_states[0][3] && button_click_prevent_ghosting[0][3] == false) {
      change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_2);
      button_click_prevent_ghosting[0][3] = true;
  } else if ( !button_click_states[0][3] ){
      button_click_prevent_ghosting[0][3] = false;
  }

  //animation HYBRID_3
  if (button_click_states[1][2] && button_click_prevent_ghosting[1][2] == false) {
      change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_3);
      button_click_prevent_ghosting[1][2] = true;
  } else if ( !button_click_states[1][2] ){
      button_click_prevent_ghosting[1][2] = false;
  }
  
  //animation HYBRID_4
  if (button_click_states[2][2] && button_click_prevent_ghosting[2][2] == false) {
      change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_4);
      button_click_prevent_ghosting[2][2] = true;
  } else if ( !button_click_states[2][2] ){
      button_click_prevent_ghosting[2][2] = false;
  }

  // switch editing mode to preset 1
  if (button_click_states[0][7] && button_click_prevent_ghosting[0][7] == false) {
      write_to_save = 1;
      button_click_prevent_ghosting[0][7] = true;
  } else if ( !button_click_states[0][7] ){
      button_click_prevent_ghosting[0][7] = false;
  }

  // switch editing mode to preset 2
  if (button_click_states[1][7] && button_click_prevent_ghosting[1][7] == false) {
      write_to_save = 2;
      button_click_prevent_ghosting[1][7] = true;
  } else if ( !button_click_states[1][7] ){
      button_click_prevent_ghosting[1][7] = false;
  }

  // switch editing mode to preset 3
  if (button_click_states[2][7] && button_click_prevent_ghosting[2][7] == false) {
      write_to_save = 3;
      button_click_prevent_ghosting[2][7] = true;
  } else if ( !button_click_states[2][7] ){
      button_click_prevent_ghosting[2][7] = false;
  }

  // switch active save to 1
  if (button_click_states[0][6] && button_click_prevent_ghosting[0][6] == false) {
      active_save = 1;
      button_click_prevent_ghosting[0][6] = true;
  } else if ( !button_click_states[0][6] ){
      button_click_prevent_ghosting[0][6] = false;
  }

  // switch active save to 2
  if (button_click_states[1][6] && button_click_prevent_ghosting[1][6] == false) {
      active_save = 2;
      button_click_prevent_ghosting[1][6] = true;
  } else if ( !button_click_states[1][6] ){
      button_click_prevent_ghosting[1][6] = false;
  }
  
  // switch active save to 3
  if (button_click_states[2][6] && button_click_prevent_ghosting[2][6] == false) {
      active_save = 3;
      button_click_prevent_ghosting[2][6] = true;
  } else if ( !button_click_states[2][6] ){
      button_click_prevent_ghosting[2][6] = false;
  }

  // set_strobe_mode( digitalRead(  ) );

  // Beat: ( beat + speed[3] + switcher )
  
  // Quick animations 5
}

void write_feedback(int mode) {
  
  if ( mode == FEEDBACK_MODE_LED || mode == FEEDBACK_MODE_DUAL ) {
    //TODO
  } else if ( ( mode == FEEDBACK_MODE_SERIAL || mode == FEEDBACK_MODE_DUAL ) && millis() - debug_mode_timestamp > 2500 ) {
    Serial.print("Active Lights: ");
    for (int i=0;i<NUM_LIGHTS; i++ ){
      Serial.print(active_lights[i]);
      Serial.print("-");
    }
    Serial.print("\n");
    Serial.print("ACTIVE SAVE: ");
    Serial.print(active_save);
    Serial.print("\n");
    Serial.print("    Animations:\n");
    Serial.print("      inner_leds:          ");
    Serial.println(saves[active_save][0][3]);
    Serial.print("       outer_leds:         ");
    Serial.println(saves[active_save][1][3]);
    Serial.print("       stage_lights:       ");
    Serial.println(saves[active_save][2][3]);
    Serial.print("       moving_heads_left:  ");
    Serial.println(saves[active_save][3][3]);
    Serial.print("       moving_heads_right: ");
    Serial.println(saves[active_save][4][3]);
    Serial.print("       blinder:            ");
    Serial.println(saves[active_save][5][3]);
    Serial.print("       laser:              ");
    Serial.println(saves[active_save][6][3]);
    Serial.print("       Special_slot_one:   ");
    Serial.println(saves[active_save][7][3]);
    Serial.print("       Special_slot_two:   ");
    Serial.println(saves[active_save][8][3]);
        Serial.print( "    colors:\n" );
    Serial.print("       inner_leds:         ");
    Serial.print(saves[active_save][0][0]);
    Serial.print("-");
    Serial.print(saves[active_save][0][1]);
    Serial.print("-");
    Serial.println(saves[active_save][0][2] );
    Serial.print("       outer_leds:         ");
    Serial.print(saves[active_save][1][0]);
    Serial.print("-");
    Serial.print(saves[active_save][1][1]);
    Serial.print("-");
    Serial.println(saves[active_save][1][2] );
    Serial.print("       stage_lights:       ");
    Serial.print(saves[active_save][2][0]);
    Serial.print("-");
    Serial.print(saves[active_save][2][1]);
    Serial.print("-");
    Serial.println(saves[active_save][2][2] );
    Serial.print("       moving_heads_left:  ");
    Serial.print(saves[active_save][3][0]);
    Serial.print("-");
    Serial.print(saves[active_save][3][1]);
    Serial.print("-");
    Serial.println(saves[active_save][3][2] );
    Serial.print("       moving_heads_right: ");
    Serial.print(saves[active_save][4][0]);
    Serial.print("-");
    Serial.print(saves[active_save][4][1]);
    Serial.print("-");
    Serial.println(saves[active_save][4][2] );
    Serial.print("       blinder:            ");
    Serial.print(saves[active_save][5][0]);
    Serial.print("-");
    Serial.print(saves[active_save][5][1]);
    Serial.print("-");
    Serial.println(saves[active_save][5][2] );
    Serial.print("       laser:              ");
    Serial.print(saves[active_save][6][0]);
    Serial.print("-");
    Serial.print(saves[active_save][6][1]);
    Serial.print("-");
    Serial.println(saves[active_save][6][2] );
    Serial.print("       special_slot_one:   ");
    Serial.print(saves[active_save][7][0]);
    Serial.print("-");
    Serial.print(saves[active_save][7][1]);
    Serial.print("-");
    Serial.println(saves[active_save][7][2] );
    Serial.print("       special_slot_two:   ");
    Serial.print(saves[active_save][8][0]);
    Serial.print("-");
    Serial.print(saves[active_save][8][1]);
    Serial.print("-");
    Serial.println(saves[active_save][8][2] );
    Serial.print("       special_slot_three: ");
    Serial.print(saves[active_save][9][0]);
    Serial.print("-");
    Serial.print(saves[active_save][9][1]);
    Serial.print("-");
    Serial.println(saves[active_save][9][2] );
    Serial.print("------------------------------------------------------------\n");
    Serial.print("WRITE TO SAVE: ");
    Serial.print(write_to_save);
    Serial.print("\n");
    Serial.print("    Animations:\n");
    Serial.print("      inner_leds:          ");
    Serial.println(saves[write_to_save][0][3]);
    Serial.print("       outer_leds:         ");
    Serial.println(saves[write_to_save][1][3]);
    Serial.print("       stage_lights:       ");
    Serial.println(saves[write_to_save][2][3]);
    Serial.print("       moving_heads_left:  ");
    Serial.println(saves[write_to_save][3][3]);
    Serial.print("       moving_heads_right: ");
    Serial.println(saves[write_to_save][4][3]);
    Serial.print("       blinder:            ");
    Serial.println(saves[write_to_save][5][3]);
    Serial.print("       laser:              ");
    Serial.println(saves[write_to_save][6][3]);
    Serial.print("       Special_slot_one:   ");
    Serial.println(saves[write_to_save][7][3]);
    Serial.print("       Special_slot_two:   ");
    Serial.println(saves[write_to_save][8][3]);
        Serial.print( "    colors:\n" );
    Serial.print("       inner_leds:         ");
    Serial.print(saves[write_to_save][0][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][0][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][0][2] );
    Serial.print("       outer_leds:         ");
    Serial.print(saves[write_to_save][1][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][1][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][1][2] );
    Serial.print("       stage_lights:       ");
    Serial.print(saves[write_to_save][2][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][2][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][2][2] );
    Serial.print("       moving_heads_left:  ");
    Serial.print(saves[write_to_save][3][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][3][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][3][2] );
    Serial.print("       moving_heads_right: ");
    Serial.print(saves[write_to_save][4][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][4][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][4][2] );
    Serial.print("       blinder:            ");
    Serial.print(saves[write_to_save][5][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][5][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][5][2] );
    Serial.print("       laser:              ");
    Serial.print(saves[write_to_save][6][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][6][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][6][2] );
    Serial.print("       special_slot_one:   ");
    Serial.print(saves[write_to_save][7][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][7][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][7][2] );
    Serial.print("       special_slot_two:   ");
    Serial.print(saves[write_to_save][8][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][8][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][8][2] );
    Serial.print("       special_slot_three: ");
    Serial.print(saves[write_to_save][9][0]);
    Serial.print("-");
    Serial.print(saves[write_to_save][9][1]);
    Serial.print("-");
    Serial.println(saves[write_to_save][9][2] );
    Serial.print("==============================================================================================\n");
    debug_mode_timestamp = millis();
  }
}
