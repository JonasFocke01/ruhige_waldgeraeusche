#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"

unsigned long led_timestamp;
int snake[PIXEL_COUNT], snake_count, blocks[PIXEL_COUNT], blocks_position = 0;
bool shift_direction_up = true;
Adafruit_NeoPixel pixels;

void led_strip_init(int r, int g, int b) {

  pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_1_PIN, NEO_GRB + NEO_KHZ800);
  
  pixels.begin();

  led_timestamp = millis();

  for (int i = 0; i < PIXEL_COUNT; i++) {
    pixels.setPixelColor(i, pixels.Color(r, g, b));
  }

  randomSeed(analogRead(9)); // 9 is an unconnected analog pin
  
  pixels.show();
}

//snake theme
void led_setup_snake(int _speed) {

  memset(snake, 0, sizeof(snake));

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }

  int tail_length;

  if (_speed == SPEED_SLOW) {
    snake_count = SNAKE_COUNT_SLOW;
    tail_length = TAIL_LENGTH_SLOW;
  } else if (_speed == SPEED_MEDIUM) {
    snake_count = SNAKE_COUNT_MEDIUM;
    tail_length = TAIL_LENGTH_MEDIUM;
  } else if (_speed == SPEED_FAST) {
    snake_count = SNAKE_COUNT_FAST;
    tail_length = TAIL_LENGTH_FAST;
  }

  for (int i = 0; i < snake_count; i++) {
    for (int j = tail_length; j > 0; j--) {
      snake[(PIXEL_COUNT - 1 - j) - (i * (PIXEL_COUNT / snake_count))] = map_from_to(map_from_to(j, 0, tail_length, tail_length, 0), 0, tail_length, 0, 10);
    }
  }
}

void led_loop_snake(int _speed, int r, int g, int b){
  int speed_increased = 1, i_wrapped;
  bool wrapped = false;
  if (_speed == 0) { speed_increased = 1; }
  if (millis() - led_timestamp > _speed * 10)
  {
    for (int i = PIXEL_COUNT - 1; i > -1; i--)
    {
      i_wrapped = i + speed_increased;
      if ( i_wrapped >= PIXEL_COUNT ) {
        i_wrapped -= PIXEL_COUNT;
        wrapped = true;
      }
      if (wrapped)
      {
        snake[i_wrapped] = snake[i];
      }
      else
      {
        snake[i_wrapped + 1] = snake[i];
      }
    }

    for (int i = 0; i < sizeof(snake) / 2; i++)
    {
      pixels.setPixelColor(i, pixels.Color((r / 10) * snake[i], (g / 10) * snake[i], (b / 10) * snake[i]));
    }
    led_timestamp = millis();
    pixels.show();
  }
}
//snake theme

//shifting blocks theme
void led_setup_shifting_blocks() {

  int likelyhood = 100;
  
  memset(blocks, 0, sizeof(blocks));

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }
  
  for (int i = EDGE_SPACING; i < sizeof(blocks) / 2 - EDGE_SPACING; i++) {
    if (random(0, 100) < likelyhood) {
      blocks[i] = 10;
      likelyhood = 90;
    } else {
      likelyhood = 10;
    }
  }
}

void led_loop_shifting_blocks(int _speed, int r, int g, int b) {

  int shift_by = 0;

  if (millis() - led_timestamp > _speed * 10){
    led_timestamp = millis();
  
    if (_speed == SPEED_SLOW) {
      shift_by = 1;
    } else if (_speed == SPEED_MEDIUM) {
      shift_by = 1;
    } else if (_speed == SPEED_FAST) {
      shift_by = 1;
    }
    
    if ( shift_direction_up && blocks_position == EDGE_SPACING ) {
      shift_direction_up = false;
    } else if ( shift_direction_up == false && blocks_position == -8  ){
      shift_direction_up = true;
    }
  
    if ( shift_direction_up ) {
      blocks_position += shift_by;
    } else {
      blocks_position -= shift_by;
    }

    //draw blocks
    int j = 0;
    for ( int i = 0; i < PIXEL_COUNT; i++) {
      pixels.setPixelColor(i + blocks_position, pixels.Color((r / 10) * blocks[j], (g / 10) * blocks[j], (b / 10) * blocks[j]));
      j++;
    }
    pixels.show();
  }
}
//shifting blocks theme
