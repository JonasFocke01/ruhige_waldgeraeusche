#include "led_strip.h"
#include "utilities.h"

unsigned long led_timestamp;
int snake[PIXEL_COUNT], snake_count;
Adafruit_NeoPixel pixels;

void led_strip_init(int r, int g, int b) {

  pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_1_PIN, NEO_GRB + NEO_KHZ800);
  
  pixels.begin();

  led_timestamp = millis();

  for (int i = 0; i < PIXEL_COUNT; i++) {
    pixels.setPixelColor(i, pixels.Color(r, g, b));
  }
  
  pixels.show();
}

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
  if (_speed == 1) { speed_increased = 2; }
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
