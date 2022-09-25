#include <stdlib.h>
#include "keyboard.h"

<<<<<<< HEAD
int saves[NUM_SAVES][BUTTON_COUNT];
int active_save = 0;

bool active_buttons    [BUTTON_ISLANDS][BUTTON_ROWS];
=======
int active_save = 0;
int write_to_save = 0;
int saves[NUM_SAVES][BUTTON_ISLANDS][BUTTON_ROWS];
>>>>>>> 085e93e (feat: 🎉 first untested implementation for the keyboard component)
bool prevent_ghosting  [BUTTON_ISLANDS][BUTTON_ROWS];

unsigned long fps_timestamp, debug_mode_timestamp;

void setup() {

  Serial.begin(2000000);

  randomSeed(analogRead(9)); // 9 is an unconnected analog pin

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

  debug_mode_timestamp = millis();
  fps_timestamp = millis();
}

void loop() {

  calculate_active_buttons();

  write_feedback();

  write_button_states_to_serial();// this should run as fast as possible to guarantee the best ux. The Framerate will be limited by the rpi
}

bool is_button_pressed(int island, int row) {
  digitalWrite(island, HIGH);

  if ( digitalRead(row) == 0 ) {
    digitalWrite(island, LOW );
    return true;
  } else {
    digitalWrite(island, LOW );
    return false;
  }
}

void calculate_active_buttons() {
  // overwritten for every button
  bool current_button_state = false;

  // buttons like flash or the tigger buttons could be colored in a way the other buttons are colored but NOT PREVENTING THINGS FROM HAPPENING

  //! EXAMPLE / BLUEPRINT
  /**
   * * Group: Scanner
   * * Type:  Animation
   * * Label: In->Out
   * Island:  2
   * Row:     5
  */
  current_button_state = is_button_pressed(2, 5);
  if (current_button_state && prevent_ghosting[2][5] == false)
  {

    // !EXAMPLELOGIC: if this button was not active previously, set it to active. 
    // !If the button for Out->In is active, set button state for random to inactive 
    if ( !saves[write_to_save][2][5] ) {
      saves[write_to_save][2][5] = true;
      if ( saves[write_to_save][2][3] ) {
        saves[write_to_save][2][8] = false;
      }
    }

    prevent_ghosting[2][5] = true;
  }
  else if (!current_button_state)
  {
    prevent_ghosting[2][5] = false;
  }
}

void write_feedback() {
  // TODO: write feedback to the keyboard leds
}

void write_button_states_to_serial() {
  // 0x69 or 105 in dec is the startbit on wich the other side should hear to start reading in the information
  Serial.write(0x69);
  // write the whole button states array
  Serial.write(active_buttons, sizeof(active_buttons));

  // an endbyte is not required

  // temp delay to verify this logic
  // delay(34);
}