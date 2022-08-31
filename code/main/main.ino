#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"
#include "controller.h"
#include "dmx.h"

unsigned long button_timestamp;

uint8_t saves[NUM_SAVES][NUM_LIGHTS][LIGHT_SAVE_SPACE];

uint8_t active_save = 3;

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

    button_timestamp = millis();
    
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
    
    Serial.print("\nbooting complete\n");
    Serial.print("----------------\n");
}

void loop(){
  
    // handle_buttons();

    led_loop( saves[active_save] );

    dmx_loop( saves[active_save] );
}

void handle_buttons() {
  if (digitalRead(TESTINPUT) == LOW && millis() - button_timestamp > DETECT_CLICKS_LENGTH_IN_MS) {
    active_save += 1;
    button_timestamp = millis();
  }
}
