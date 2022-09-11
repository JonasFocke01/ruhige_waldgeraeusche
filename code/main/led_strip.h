#include <Adafruit_NeoPixel.h>
#include "controller.h"

//setup
#define PIXEL_COUNT 150
#define INNER_LED_STRIP_PIN 4
#define OUTER_LED_STRIP_PIN 7

void led_setup();
void led_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
//setup

//snake theme
#define SNAKE_COUNT 8
#define TAIL_LENGTH 8
//snake theme

//shifting blocks theme
#define EDGE_SPACING 8
//shifting blocks theme

// spawner
void spawn_fade_sector();
void spawn_snake();
void spawn_rain_drop();
void turn_shifting_blocks_direction();
// spawner
