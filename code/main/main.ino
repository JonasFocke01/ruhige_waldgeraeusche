#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"
#include "controller.h"
#include "dmx.h"

uint8_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_save = 3;

unsigned long button_timestamps[BUTTON_COUNT];

unsigned long loop_interval_length;

void change_values_in_save(int save, int light, int r, int g, int b, int animation) {
  saves[save][light][0] = r;
  saves[save][light][1] = b;
  saves[save][light][2] = g;
  saves[save][light][3] = animation;
}

void applly_save_settings_to_default(int source_save) {
  memcpy( saves[0], saves[source_save], NUM_LIGHTS * LIGHT_SAVE_SPACE );
}

void setup() {
    
    Serial.begin(9600);

    Serial.print("booting...");

    randomSeed(analogRead(9)); // 9 is an unconnected analog pin
    
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
  
    // handle_buttons();

    led_loop( saves[active_save] );

    dmx_loop( saves[active_save] );

    loop_interval_length = millis();
}

void handle_buttons() {
  //if (digitalRead(TESTINPUT) == LOW && millis() - button_timestamp > DETECT_CLICKS_LENGTH_IN_MS) {
  //  active_save += 1;
  //  button_timestamp = millis();
  //}

  // TODO Buttons

  // Quick animations 5

  // LED colorthemes 8

  // Animations 20

  // Beat: ( beat + speed[3] + switcher )

  // select each light 8

  // select all lights 1

  // turn each light on/off 5

  // turn all lights on/off 2

  // min/max all lights 2

  // strobe 1

  // strobe frequency level 1

  // blinder level 1

  // special_lights level 1

  // stage lights rgb 3

  // preset switches 5

  // preset apply 5

  // master 1 ?
}
