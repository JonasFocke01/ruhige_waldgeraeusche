#include "led_strip.h"
#include "utilities.h"

unsigned long timestamp;
void setup_leds(int color);
int snake[PIXEL_COUNT];
int snake_count;
int color_store[3];
Adafruit_NeoPixel pixels;

void led_strip_init(int theme) {

  pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_1_PIN, NEO_GRB + NEO_KHZ800);
  
  pixels.begin();

  memset(color_store, 0, sizeof(color_store) / 2);
  led_change_theme(theme);

  timestamp = millis();

  for (int i = 0; i < PIXEL_COUNT; i++) {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }
  
  pixels.show();
}

void led_setup_snake(int _speed) {

  memset(snake, 0, sizeof(snake) / 2);

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

void led_loop_snake(int _speed){
  if (millis() - timestamp > _speed * 10)
  {
    for (int i = PIXEL_COUNT - 1; i > -1; i--)
    {
      if (i == PIXEL_COUNT - 1)
      {
        snake[0] = snake[PIXEL_COUNT - 1];
      }
      else
      {
        snake[i + 1] = snake[i];
      }
    }

    for (int i = 0; i < sizeof(snake) / 2; i++)
    {
      pixels.setPixelColor(i, pixels.Color(color_store[0] * snake[i], color_store[1] * snake[i], color_store[2] * snake[i]));
    }
    timestamp = millis();
    pixels.show();
  }
}

void led_change_theme(int theme) {
  if (theme == 0) {
    color_store[0] = 0;
    color_store[1] = 25;
    color_store[2] = 0;
  } else if (theme == 1) {
    color_store[0] = 25;
    color_store[1] = 25;
    color_store[2] = 0;
  } else if (theme == 2) {
    color_store[0] = 25;
    color_store[1] = 0;
    color_store[2] = 0;
  } else if (theme == 3) {
    color_store[0] = 25;
    color_store[1] = 0;
    color_store[2] = 25;
  } else if (theme == 4) {
    color_store[0] = 0;
    color_store[1] = 0;
    color_store[2] = 25;
  } else if (theme == 5) {
    color_store[0] = 0;
    color_store[1] = 25;
    color_store[2] = 25;
  } else if (theme == 6) {
    color_store[0] = 25;
    color_store[1] = 25;
    color_store[2] = 25;
  } else if (theme == 7) {
    color_store[0] = 0;
    color_store[1] = 0;
    color_store[2] = 0;
  }  
}
