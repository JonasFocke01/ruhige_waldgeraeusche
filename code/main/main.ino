#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"
#include "controller.h"
#include "dmx.h"

int _speed, strobe_mode, color_theme[3];
bool strobe;
unsigned long button_timestamp;

void setup() {
    
    Serial.begin(9600);

    Serial.print("booting...");

    randomSeed(analogRead(9)); // 9 is an unconnected analog pin
    
    _speed = SPEED_SLOW;

    memset(color_theme, 0, sizeof(color_theme));
    strobe = false;
    strobe_mode = STROBE_MODE_DEDICATED;
    
    led_strip_init(color_theme[0], color_theme[1], color_theme[2]);
    dmx_channels_init();

    srand(1);

    button_timestamp = millis();
    pinMode(TESTINPUT, INPUT_PULLUP);
    
    Serial.print(" ok\n");

    Serial.print("setting default themes...");
    
    change_color_theme(150, 150, 150);
    
    Serial.print("ok\n");

    Serial.print("setting default speed...");
    
    // led_setup_snake(_speed);
    //led_setup_shifting_blocks();
    led_setup_rain_drops();
    
    Serial.print("ok\n");
    
    Serial.print("booting complete\n");
    Serial.print("----------------\n");
}

void loop(){
    
    handle_buttons();
    
    // led_loop_snake(_speed, color_theme[0], color_theme[1], color_theme[2]);
    //led_loop_shifting_blocks(_speed, color_theme[0], color_theme[1], color_theme[2]);
    led_loop_rain_drops(_speed, color_theme[0], color_theme[1], color_theme[2]);
    
    dmx_main_loop(color_theme[0], color_theme[1], color_theme[2], false);
}

void handle_buttons() {
  if (digitalRead(TESTINPUT) == LOW && millis() - button_timestamp > DETECT_CLICKS_LENGTH_IN_MS) {
    if (_speed == SPEED_SLOW) {
      _speed = SPEED_MEDIUM;
    } else if (_speed == SPEED_MEDIUM) {
      _speed = SPEED_FAST;
    } else {
      _speed = SPEED_SLOW;
    }    
    change_color_theme(random(0, 255), random(0, 255), random(0, 255));
    led_setup_snake(_speed);
    button_timestamp = millis();
  }
}

void change_color_theme(int r, int g, int b) {
  color_theme[0] = r;
  color_theme[1] = g;
  color_theme[2] = b;
}
