#include <Adafruit_NeoPixel.h>
#include "main.h"

//setup
#define PIXEL_COUNT 150
#define LED_1_PIN 4

unsigned long timestamp;
void setup_leds(int color);
Adafruit_NeoPixel pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_1_PIN, NEO_GRB + NEO_KHZ800);
//setup


// snake theme
#define SNAKE_COUNT_SLOW 1
#define TAIL_LENGTH_SLOW 50
#define SNAKE_COUNT_MEDIUM 3
#define TAIL_LENGTH_MEDIUM 25
#define SNAKE_COUNT_FAST 9
#define TAIL_LENGTH_FAST 10

int snake[PIXEL_COUNT];
int snake_count;
int color_store[3];

void led_loop_snake(int speed);
void led_setup_snake(int speed);
// snake theme

//utility functions
void led_change_theme(int color);
//utility functions