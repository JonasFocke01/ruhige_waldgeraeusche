#include "led_strip.h"

int _speed;
bool strobe;
int strobe_mode;

void setup() {
    _speed = SPEED_FAST;

  
    strobe = false;
    strobe_mode = STROBE_MODE_DEDICATED;
    
    Serial.begin(9600);

    Serial.print("booting leds and dmx...");
    led_strip_init(THEME_OFF);
    //setup_dmx();
    Serial.print(" ok\n");

    Serial.print("setting default themes...");
    led_change_theme(THEME_RED);
    Serial.print("ok\n");

    Serial.print("setting default speed...");
    led_setup_snake(_speed);
    Serial.print("ok\n");
    
    Serial.print("booting complete\n");
    Serial.print("----------------\n");
}

void loop(){
    //default: run snake
    led_loop_snake(_speed);

    //loop_dmx();
    
}
