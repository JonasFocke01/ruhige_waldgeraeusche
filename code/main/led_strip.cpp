#include "led_strip.h"

void led_strip_init(int color) {
  pixels.begin();

  memset(color_store, 0, sizeof(color_store));
  change_theme(color);

  timestamp = millis();

  for (int i = 0; i < PIXEL_COUNT; i++) {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }
  pixels.show();
}

void led_setup_snake(int speed) {

  memset(snake, 0, sizeof(snake));

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }

  if (speed == SPEED_SLOW) {
    snake_count = SNAKE_COUNT_SLOW;
  } else if (speed == SPEED_MEDIUM) {
    snake_count = SNAKE_COUNT_MEDIUM;
  } else if (speed == SPEED_FAST) {
    snake_count = SNAKE_COUNT_FAST;
  }

  for (int i = 0; i < snake_count; i++) {
    for (int j = 0; j < tail_length; j++) {
      snake[(PIXEL_COUNT - 1 - j) - (i * (PIXEL_COUNT - (snake_count * TAIL_LENGTH_FAST + 2)))] = (255 - j * (255 / TAIL_LENGTH));
    }
  }

}

void led_loop_snake(int speed){
  if (millis() - timestamp > speed * 10)
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

    for (int i = 0; i < sizeof(snake); i++)
    {
      pixels.setPixelBrightness(i, snake[i]);
      pixels.setPixelColor(i, pixels.Color(color_store[0], color_store[1], color_store[2]));
    }
    timestamp = millis();
    pixels.show();
  }
}

void led_change_theme(int color) {
  if (color = 0) {
    color_store[0] = 10;
    color_store[1] = 255;
    color_store[2] = 10;
  } else if (color = 1) {
    color_store[0] = 255;
    color_store[1] = 255;
    color_store[2] = 10;
  } else if (color = 2) {
    color_store[0] = 255;
    color_store[1] = 10;
    color_store[2] = 10;
  } else if (color = 3) {
    color_store[0] = 255;
    color_store[1] = 10;
    color_store[2] = 255;
  } else if (color = 4) {
    color_store[0] = 10;
    color_store[1] = 10;
    color_store[2] = 255;
  } else if (color = 5) {
    color_store[0] = 10;
    color_store[1] = 255;
    color_store[2] = 255;
  } else if (color = 6) {
    color_store[0] = 255;
    color_store[1] = 255;
    color_store[2] = 255;
  } else if (color = 7) {
    color_store[0] = 0;
    color_store[1] = 0;
    color_store[2] = 0;
  }
}
