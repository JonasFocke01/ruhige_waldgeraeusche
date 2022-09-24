#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"
#include "dmx.h"

uint16_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_animation;

unsigned long time_between_beats = 1000;
bool pressed = false;

int potentiometer;

uint8_t active_save;
uint8_t write_to_save;
bool active_lights[NUM_LIGHTS], display_beat;

bool button_click_states            [BUTTON_ISLANDS][BUTTON_ROWS];
bool button_click_prevent_ghosting  [BUTTON_ISLANDS][BUTTON_ROWS];

bool solo_button_click_states            [4];
bool solo_button_click_prevent_ghosting  [4];

unsigned long beat_timestamp, debug_mode_timestamp, last_beat_timestamp, last_animation_two_timestamp, loop_timestamp;


void change_values_in_write_to_save_for_each_active_light(int r, int g, int b, int animation) {
  for (int i = 0; i < NUM_LIGHTS - 1; i++ ) {
    if ( active_lights[i] ) {
      if ( r != 256 )         saves[write_to_save][i][0] = r;
      if ( g != 256 )         saves[write_to_save][i][1] = b;
      if ( b != 256 )         saves[write_to_save][i][2] = g;
      if ( animation != 256 ) saves[write_to_save][i][3] = animation;
    }
  }
}

void setup() {

  Serial.begin(2000000);

  Serial.print("booting...");

  randomSeed(analogRead(9)); // 9 is an unconnected analog pin

  write_to_save = 1;
  active_save = 1;
  memset(active_lights, false, sizeof(active_lights) / sizeof(active_lights[0]));

  for (int i = 0; i < NUM_LIGHTS; i++) {
    for (int j = 0; j < LIGHT_SAVE_SPACE; j++) {
      saves[0][i][j] = 256;
    }
  }

  debug_mode_timestamp = 1000;

  dmx_channels_init();

  led_setup();

  pinMode(LED_BUILTIN, OUTPUT);


  Serial.print(" ok\n");

  Serial.print("setting default color_themes...");

  for ( int i = 1; i < NUM_SAVES; i++ ) {
    for ( int j = 0; j < NUM_LIGHTS; j++ ) {
      saves[i][j][0] = 10;
      saves[i][j][1] = 250;
      saves[i][j][2] = 10;
    }
  }

  Serial.print("ok\n");

  Serial.print("preparing animations...");

  memset(active_lights, true, sizeof(active_lights) / sizeof(active_lights[0]));

  for ( int i = 1; i < NUM_SAVES; i++ ) {
    for ( int j = 0; j < NUM_LIGHTS - 1; j++ ) {
      saves[i][j][3] = DROP_2;
    }
  }

  for ( int i = 0; i < NUM_LIGHTS - 1; i++ ) {
    saves[6][i][3] = FLASH;
  }

  for ( int i = 0; i < NUM_LIGHTS - 1; i++ ) {
    saves[7][i][3] = OFF;
  }

  for ( int i = 0; i < NUM_LIGHTS - 1; i++ ) {
    saves[8][i][3] = BRIZZLE;
  }

  for ( int i = 0; i < NUM_LIGHTS - 1; i++ ) {
    saves[9][i][3] = STROBE;
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

  pinMode(SOLO_BUTTON_ONE,   INPUT_PULLUP);
  pinMode(SOLO_BUTTON_TWO,   INPUT_PULLUP);
  pinMode(SOLO_BUTTON_THREE, INPUT_PULLUP);
  pinMode(SOLO_BUTTON_FOUR,  INPUT_PULLUP);

  pinMode(AUDIO_JACK, INPUT);

  Serial.print("ok\n");

  Serial.print("\nbooting complete in ");
  Serial.print(millis());
  Serial.print("ms\n");
  Serial.print("----------------\n");

  beat_timestamp = millis();
  loop_timestamp = millis();
}

void loop() {

  // Serial.println( millis() - loop_timestamp );
  loop_timestamp = millis();

  handle_inputs();

  detect_beat();

  write_feedback(FEEDBACK_MODE_OFF);

  led_loop( saves[active_save] );

  dmx_loop( saves[active_save] );
}

void read_buttons() {

  solo_button_click_states[0] = !digitalRead(SOLO_BUTTON_ONE);
  solo_button_click_states[1] = !digitalRead(SOLO_BUTTON_TWO);
  solo_button_click_states[2] = !digitalRead(SOLO_BUTTON_THREE);
  solo_button_click_states[3] = !digitalRead(SOLO_BUTTON_FOUR);
  for (int i = 0; i < BUTTON_ISLANDS; i++) {
    if (i > 0) {
      digitalWrite(ISLAND_ONE + i - 1, HIGH);
    }
    digitalWrite(ISLAND_ONE + i, LOW);
    for (int j = 0; j < BUTTON_ROWS; j++) {
      if (!digitalRead(j + ROW_ONE)) {
        button_click_states[i][j] = true;
      } else {
        button_click_states[i][j] = false;
      }
      if ( button_click_states[i][j] ) {
        // Serial.print("    ");
        // Serial.print(i);
        // Serial.print("/");
        // Serial.print(j);
        // Serial.print("\n");
      }
    }
  }
}

int detect_beat_state = true;
void detect_beat() {
  if ( detect_beat_state && millis() - last_beat_timestamp > map( analogRead( POTENTIOMETER ), 0, 1024, 50, 800) ) {
    digitalWrite(LED_BUILTIN, HIGH);
    if ( active_animation == DROP_2 ) {
      spawn_fade_sector(1);
    } else {
      spawn_fade_sector(0);
    }
    spawn_snake();
    spawn_rain_drop();
    randomize_laser_animation();
    if ( active_animation == HYBRID_3 ) {
      turn_shifting_blocks_direction();
    } else {
      invert_shifting_blocks();
    }
    digitalWrite(LED_BUILTIN, LOW);
    last_beat_timestamp = millis();
  }
}

void handle_inputs() {

  // weird memory writing prevention
  if ( write_to_save == 0 ) {
    write_to_save = 1;
  }

  // read buttons into button_click_states array
  read_buttons();

  if ( solo_button_click_states[3] && solo_button_click_prevent_ghosting[3] == false ) {
    detect_beat_state = !detect_beat_state;
    solo_button_click_prevent_ghosting[3] = true;
  } else {
    solo_button_click_prevent_ghosting[3] = false;
  }

  while (solo_button_click_states[3]) { // turn detect beat off
    read_buttons();
  }

  bool erase_pixel = false;
  while (solo_button_click_states[2]) { // pitch
    set_pitch_position( map(analogRead( POTENTIOMETER ), 0, 1024, 0, PIXEL_COUNT ) );
    led_loop( saves[OFF] );
    dmx_loop( saves[OFF] );
    read_buttons();
    erase_pixel = true;
  }

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

  // strobe
  while (button_click_states[0][5]) {
    
    led_loop( saves[9] );
    dmx_loop( saves[9] );
    read_buttons();
  }

  // fill pixels
  while (solo_button_click_states[0]) {
    fill_pixels( map(analogRead( POTENTIOMETER ), 0, 1024, 0, 100 ) );
    led_loop( saves[active_save] );
    dmx_loop( saves[active_save] );
    read_buttons();
    erase_pixel = true;
  }
  if ( erase_pixel ) {
    erase_pixels();
  }

  while (solo_button_click_states[1]) {
    if ( millis() - last_animation_two_timestamp > analogRead( POTENTIOMETER ) ) {
      led_loop( saves[8] );
      dmx_loop( saves[8] );
      last_animation_two_timestamp = millis();
    }
    read_buttons();
  }

  // change laser animation
  if (button_click_states[0][7] && button_click_prevent_ghosting[0][7] == false) {
    randomize_laser_animation();
    button_click_prevent_ghosting[0][7] = true;
  } else if ( !button_click_states[0][7] ) {
    button_click_prevent_ghosting[0][7] = false;
  }

  // active lights to white
  if (button_click_states[0][6] && button_click_prevent_ghosting[0][6] == false) {
    change_values_in_write_to_save_for_each_active_light( 250, 250, 250, 256 );
    button_click_prevent_ghosting[0][6] = true;
  } else if ( !button_click_states[0][6] ) {
    button_click_prevent_ghosting[0][6] = false;
  }

  // active lights to red
  if (button_click_states[0][8] && button_click_prevent_ghosting[0][8] == false) {
    change_values_in_write_to_save_for_each_active_light( 255, 10, 10, 256 );
    button_click_prevent_ghosting[0][8] = true;
  } else if ( !button_click_states[0][8] ) {
    button_click_prevent_ghosting[0][8] = false;
  }

  // active lights to green
  if (button_click_states[1][8] && button_click_prevent_ghosting[1][8] == false) {
    change_values_in_write_to_save_for_each_active_light( 10, 255, 10, 256 );
    button_click_prevent_ghosting[1][8] = true;
  } else if ( !button_click_states[1][8] ) {
    button_click_prevent_ghosting[1][8] = false;
  }

  // active lights to blue
  if (button_click_states[2][8] && button_click_prevent_ghosting[2][8] == false) {
    change_values_in_write_to_save_for_each_active_light( 10, 10, 255, 256 );
    button_click_prevent_ghosting[2][8] = true;
  } else if ( !button_click_states[2][8] ) {
    button_click_prevent_ghosting[2][8] = false;
  }

  // select leds
  if (button_click_states[0][1] && button_click_prevent_ghosting[0][1] == false) {
    active_lights[0] = !active_lights[0];
    button_click_prevent_ghosting[0][1] = true;
  } else if ( !button_click_states[0][1] ) {
    button_click_prevent_ghosting[0][1] = false;
  }

  // select moving heads
  if (button_click_states[1][1] && button_click_prevent_ghosting[1][1] == false) {
    active_lights[1] = !active_lights[1];
    button_click_prevent_ghosting[1][1] = true;
  } else if ( !button_click_states[1][1] ) {
    button_click_prevent_ghosting[1][1] = false;
  }

  // select laser
  if (button_click_states[2][1] && button_click_prevent_ghosting[2][1] == false) {
    active_lights[2] = !active_lights[2];
    button_click_prevent_ghosting[2][1] = true;
  } else if ( !button_click_states[2][1] ) {
    button_click_prevent_ghosting[2][1] = false;
  }

  // select all lights
  if (button_click_states[1][4] && button_click_prevent_ghosting[1][4] == false) {
    for (int i = 0; i < NUM_LIGHTS; i++) {
      active_lights[i] = true;
    }
    button_click_prevent_ghosting[1][4] = true;
  } else if ( !button_click_states[1][4] ) {
    button_click_prevent_ghosting[1][4] = false;
  }

  // deselect all lights
  if (button_click_states[2][4] && button_click_prevent_ghosting[2][4] == false) {
    for (int i = 0; i < NUM_LIGHTS; i++) {
      active_lights[i] = false;
    }
    button_click_prevent_ghosting[2][4] = true;
  } else if ( !button_click_states[2][4] ) {
    button_click_prevent_ghosting[2][4] = false;
  }

  // turn leds off
  if (button_click_states[0][0] && button_click_prevent_ghosting[0][0] == false) {
    saves[write_to_save][0][3] = 255;
    button_click_prevent_ghosting[0][0] = true;
  } else if ( !button_click_states[0][0] ) {
    button_click_prevent_ghosting[0][0] = false;
  }

  // turn moving heads off
  if (button_click_states[1][0] && button_click_prevent_ghosting[1][0] == false) {
    saves[write_to_save][1][3] = 255;
    button_click_prevent_ghosting[1][0] = true;
  } else if ( !button_click_states[1][0] ) {
    button_click_prevent_ghosting[1][0] = false;
  }

  // turn laser off
  if (button_click_states[2][0] && button_click_prevent_ghosting[2][0] == false) {
    saves[write_to_save][2][3] = 255;
    button_click_prevent_ghosting[2][0] = true;
  } else if ( !button_click_states[2][0] ) {
    button_click_prevent_ghosting[2][0] = false;
  }

  //animation HYBRID_1
  if (button_click_states[0][2] && button_click_prevent_ghosting[0][2] == false) {
    spawn_snake();
    active_animation = HYBRID_1;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_1);
    button_click_prevent_ghosting[0][2] = true;
  } else if ( !button_click_states[0][2] ) {
    button_click_prevent_ghosting[0][2] = false;
  }

  //animation HYBRID_2
  if (button_click_states[0][3] && button_click_prevent_ghosting[0][3] == false) {
    spawn_rain_drop();
    beat_timestamp = millis();
    active_animation = HYBRID_2;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_2);
    button_click_prevent_ghosting[0][3] = true;
  } else if ( !button_click_states[0][3] ) {
    button_click_prevent_ghosting[0][3] = false;
  }

  //animation HYBRID_3
  if (button_click_states[1][2] && button_click_prevent_ghosting[1][2] == false) {
    turn_shifting_blocks_direction();
    beat_timestamp = millis();
    active_animation = HYBRID_3;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_3);
    button_click_prevent_ghosting[1][2] = true;
  } else if ( !button_click_states[1][2] ) {
    button_click_prevent_ghosting[1][2] = false;
  }

  //animation HYBRID_4
  if (button_click_states[2][2] && button_click_prevent_ghosting[2][2] == false) {
    spawn_fade_sector(0);
    beat_timestamp = millis();
    active_animation = HYBRID_4;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, HYBRID_4);
    button_click_prevent_ghosting[2][2] = true;
  } else if ( !button_click_states[2][2] ) {
    button_click_prevent_ghosting[2][2] = false;
  }


  //animation DROP_1
  if (button_click_states[1][3] && button_click_prevent_ghosting[1][3] == false) {
    invert_shifting_blocks();
    beat_timestamp = millis();
    active_animation = DROP_1;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, DROP_1);
    button_click_prevent_ghosting[1][3] = true;
  } else if ( !button_click_states[1][3] ) {
    button_click_prevent_ghosting[1][3] = false;
  }

  //animation DROP_2
  if (button_click_states[2][3] && button_click_prevent_ghosting[2][3] == false) {
    spawn_fade_sector(1);
    beat_timestamp = millis();
    active_animation = DROP_2;
    change_values_in_write_to_save_for_each_active_light(256, 256, 256, DROP_2);
    button_click_prevent_ghosting[2][3] = true;
  } else if ( !button_click_states[2][3] ) {
    button_click_prevent_ghosting[2][3] = false;
  }

  // switch editing mode to preset 2
  if (button_click_states[1][7] && button_click_prevent_ghosting[1][7] == false) {
    write_to_save = 2;
    button_click_prevent_ghosting[1][7] = true;
  } else if ( !button_click_states[1][7] ) {
    button_click_prevent_ghosting[1][7] = false;
  }

  // switch editing mode to preset 3
  if (button_click_states[2][7] && button_click_prevent_ghosting[2][7] == false) {
    write_to_save = 3;
    button_click_prevent_ghosting[2][7] = true;
  } else if ( !button_click_states[2][7] ) {
    button_click_prevent_ghosting[2][7] = false;
  }

  // switch active save to 2
  if (button_click_states[1][6] && button_click_prevent_ghosting[1][6] == false) {
    active_save = 2;
    button_click_prevent_ghosting[1][6] = true;
  } else if ( !button_click_states[1][6] ) {
    button_click_prevent_ghosting[1][6] = false;
  }

  // switch active save to 3
  if (button_click_states[2][6] && button_click_prevent_ghosting[2][6] == false) {
    active_save = 3;
    button_click_prevent_ghosting[2][6] = true;
  } else if ( !button_click_states[2][6] ) {
    button_click_prevent_ghosting[2][6] = false;
  }
}

void write_feedback(int mode) {

  if ( mode == FEEDBACK_MODE_LED || mode == FEEDBACK_MODE_DUAL ) {
    //TODO
  } else if ( ( mode == FEEDBACK_MODE_SERIAL || mode == FEEDBACK_MODE_DUAL ) && millis() - debug_mode_timestamp > 2500 ) {
    Serial.print("Active Lights: ");
    for (int i = 0; i < NUM_LIGHTS; i++ ) {
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
