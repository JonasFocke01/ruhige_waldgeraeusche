#include <Adafruit_NeoPixel.h>
#include "main.h"

void setup() {
    Serial.begin(9600);

    Serial.println("booting leds and dmx...");

    // boot devices
    setup_dmx();

    Serial.println("booting complete...");
}

void loop(){
    
    //loop_dmx();
    
}
