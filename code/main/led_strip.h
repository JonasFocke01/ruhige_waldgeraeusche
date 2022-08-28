#include <Adafruit_NeoPixel.h>

//setup
#define PIXEL_COUNT 150
#define LED_1_PIN 4

void led_strip_init(int, int, int);
//setup

//snake theme
#define SNAKE_COUNT 8
#define TAIL_LENGTH 8

void led_setup_snake();
void led_loop_snake(int, int, int);
//snake theme

//shifting blocks theme
#define EDGE_SPACING 8

void led_setup_shifting_blocks();
void led_loop_shifting_blocks(int, int, int);
//shifting blocks theme

//raindrops theme
void led_setup_rain_drops();
void led_loop_rain_drops(int, int, int);
//raindrops theme

//fading sectors theme
void led_erase_existing_fade_sectors();
void led_spawn_fade_sector(int first_pixel, int sector_length, int r, int g, int b);
void led_fade_out_sectors();
//fading sectors theme
