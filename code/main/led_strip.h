#include <Adafruit_NeoPixel.h>
#include "main.h"

//setup
#define PIXEL_COUNT 150
#define LED_1_PIN 4

unsigned long timestamp;
void setup_leds(int color);
Adafruit_NeoPixel pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_1_PIN, NEO_GRB + NEO_KHZ800);
//setup

//utility functions
void led_change_theme(int color);
//utility functions