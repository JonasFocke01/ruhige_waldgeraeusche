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
