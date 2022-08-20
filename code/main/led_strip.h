#include <Adafruit_NeoPixel.h>

//setup
#define PIXEL_COUNT 150
#define LED_1_PIN 4

void led_strip_init(int, int, int);
//setup


// snake theme
#define SNAKE_COUNT_SLOW 1
#define TAIL_LENGTH_SLOW 50
#define SNAKE_COUNT_MEDIUM 3
#define TAIL_LENGTH_MEDIUM 25
#define SNAKE_COUNT_FAST 9
#define TAIL_LENGTH_FAST 10

void led_setup_snake(int);
void led_loop_snake(int, int, int, int);
// snake theme
