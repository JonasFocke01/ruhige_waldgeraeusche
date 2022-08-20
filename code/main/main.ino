#include "led_strip.h"
#include "utilities.h"
#include "controller.h"
#include "dmx.h"

int _speed;
bool strobe;
int strobe_mode;
unsigned long button_timestamp;

void setup() {
    _speed = SPEED_SLOW;

  
    strobe = false;
    strobe_mode = STROBE_MODE_DEDICATED;
    
    Serial.begin(9600);

    Serial.print("booting...");
    led_strip_init(THEME_OFF);
    dmx_channels_init(THEME_OFF);

    button_timestamp = millis();
    pinMode(TESTINPUT, INPUT_PULLUP);
    Serial.print(" ok\n");

    Serial.print("setting default themes...");
    led_change_theme(THEME_RED);
    dmx_change_theme(THEME_RED);
    Serial.print("ok\n");

    Serial.print("setting default speed...");
    led_setup_snake(_speed);
    dmx_setup_snake(_speed);
    Serial.print("ok\n");
    
    Serial.print("booting complete\n");
    Serial.print("----------------\n");
}

void loop(){
    
    handle_buttons();
    
    //default: run snake
    led_loop_snake(_speed);
}

void handle_buttons() {
  if (digitalRead(TESTINPUT) == LOW && millis() - button_timestamp > 1000) {
    if (_speed == SPEED_SLOW) {
      _speed = SPEED_MEDIUM;
    } else if (_speed == SPEED_MEDIUM) {
      _speed = SPEED_FAST;
    } else {
      _speed = SPEED_SLOW;
    }
    led_setup_snake(_speed);
    button_timestamp = millis();
  }
}
