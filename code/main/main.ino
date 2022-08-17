#include <Adafruit_NeoPixel.h>
#include "led_strip.h"
#include "main.h"

void setup() {
    Serial.begin(9600);

    Serial.println("booting leds and dmx...");

    // boot devices
    led_strip_init(THEME_OFF);
    setup_dmx();

    // run default themes
    led_change_theme(THEME_RED);
    led_setup_snake(SPEED_SLOW);

    Serial.println("booting complete...");
}

void loop(){

    led_loop_snake(SPEED_SLOW);

    //loop_dmx();
    
}
