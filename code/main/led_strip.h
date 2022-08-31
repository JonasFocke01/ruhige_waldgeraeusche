#include <Adafruit_NeoPixel.h>
#include "controller.h"

//setup
#define PIXEL_COUNT 150
#define INNER_LED_STRIP_PIN 4
#define OUTER_LED_STRIP_PIN 7

void led_setup();
void led_loop(uint8_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
//setup

//snake theme
#define SNAKE_COUNT 8
#define TAIL_LENGTH 8
//snake theme

//shifting blocks theme
#define EDGE_SPACING 8
//shifting blocks theme
