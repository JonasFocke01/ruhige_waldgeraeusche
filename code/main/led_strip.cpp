#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"

unsigned long led_timestamp;

int snake[PIXEL_COUNT], blocks[PIXEL_COUNT], blocks_position = 0, rain_drops[PIXEL_COUNT][2], fade_sectors[PIXEL_COUNT], likelyhood;
bool shift_direction_up = true;
Adafruit_NeoPixel inner_pixels;
Adafruit_NeoPixel outer_pixels;

void led_setup() {

  randomSeed(analogRead(9)); // 9 is an unconnected analog pin

  // OUTER_LED_STRIP_PIN
  inner_pixels = Adafruit_NeoPixel(PIXEL_COUNT, INNER_LED_STRIP_PIN, NEO_GRB + NEO_KHZ800);
  outer_pixels = Adafruit_NeoPixel(PIXEL_COUNT, OUTER_LED_STRIP_PIN, NEO_GRB + NEO_KHZ800);

  led_timestamp = millis();


  // hybrid_1
  for (int i = 0; i < SNAKE_COUNT; i++) {
    for (int j = TAIL_LENGTH; j > 0; j--) {
      snake[(PIXEL_COUNT - 1 - j) - (i * (PIXEL_COUNT / SNAKE_COUNT))] = map(map(j, 0, TAIL_LENGTH, TAIL_LENGTH, 0), 0, TAIL_LENGTH, 0, 10);
    }
  }

  // hybrid 2 - nothing to do here

  // hybrid 3
  likelyhood = 100;

  for (int i = EDGE_SPACING; i < sizeof(blocks) / 2 - EDGE_SPACING; i++) {
    if (random(0, 100) < likelyhood) {
      blocks[i] = 10;
      likelyhood = 90;
    } else {
      likelyhood = 10;
    }
  }

  // hybrid 4 - nothing to do here

  inner_pixels.begin();
  outer_pixels.begin();

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    inner_pixels.setPixelColor(i, inner_pixels.Color(0, 0, 0));
    outer_pixels.setPixelColor(i, outer_pixels.Color(0, 0, 0));
  }
}

void led_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {

  if ( save[0][3] == HYBRID_1 || save[1][3] == HYBRID_1 ) { // snake
    for (int i = PIXEL_COUNT - 3; i > -1; i--)
    {
      snake[i + 4] = snake[i];
    }

    snake[PIXEL_COUNT - 1] = 0;
    snake[1] = 0;
    snake[2] = 0;
    snake[3] = 0;

    for (int i = 0; i < sizeof(snake) / 2; i++)
    {
      if ( save[0][3] == HYBRID_1 ) {
        inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * snake[i], (save[0][1] / 10) * snake[i], (save[0][2] / 10) * snake[i]));
      }
      if ( save[1][3] == HYBRID_1 ) {
        outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * snake[i], (save[1][1] / 10) * snake[i], (save[1][2] / 10) * snake[i]));
      }
    }
  }
  if ( save[0][3] == HYBRID_2 || save[1][3] == HYBRID_2 ) { // raindrops
    if (millis() - led_timestamp > 1) {

      //progress waves
      rain_drops[0][0] = 0;
      rain_drops[PIXEL_COUNT - 1][0] = 0;
      for (int i = 1; i < PIXEL_COUNT - 1; i++) {
        if (rain_drops[i][0] > 0) {
          if (rain_drops[i][1] == 2) {
            rain_drops[i + 2][0] = 10;
            rain_drops[i + 2][1] = 0;
            rain_drops[i - 2][0] = 10;
            rain_drops[i - 2][1] = 1;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
          } else if (rain_drops[i][1] == 0) {
            rain_drops[i + 2][0] = 10;
            rain_drops[i + 2][1] = 0;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
            i++;
          }
          else if (rain_drops[i][1] == 1) {
            rain_drops[i - 2][0] = 10;
            rain_drops[i - 2][1] = 1;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
          }
        }
      }

      //draw waves
      for ( int i = 0; i < PIXEL_COUNT; i++) {
        if ( save[0][3] == HYBRID_2 ) {
          inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * rain_drops[i][0] + rain_drops[i][1], (save[0][1] / 10) * rain_drops[i][0], (save[0][2] / 10) * rain_drops[i][0]));
        }
        if ( save[1][3] == HYBRID_2 ) {
          outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * rain_drops[i][0], (save[1][1] / 10) * rain_drops[i][0], (save[1][2] / 10) * rain_drops[i][0]));
        }
      }
      led_timestamp = millis();
    }
  }
  if ( save[0][3] == HYBRID_3 || save[1][3] == HYBRID_3 ) { // shifting blocks
    if (millis() - led_timestamp > 5) {

      // shift blocks
      if ( shift_direction_up ) {
        blocks_position += 1;
      } else {
        blocks_position -= 1;
      }

      //draw blocks
      int j = 0;
      for ( int i = 0; i < PIXEL_COUNT; i++) {
        if ( save[0][3] == HYBRID_3 ) {
          inner_pixels.setPixelColor(i + blocks_position, inner_pixels.Color((save[0][0] / 10) * blocks[j], (save[0][1] / 10) * blocks[j], (save[0][2] / 10) * blocks[j]));
        }
        if ( save[1][3] == HYBRID_3 ) {
          outer_pixels.setPixelColor(i + blocks_position, outer_pixels.Color((save[1][0] / 10) * blocks[j], (save[1][1] / 10) * blocks[j], (save[1][2] / 10) * blocks[j]));
        }
        j++;
      }

      Serial.println(blocks_position);
      Serial.println(EDGE_SPACING);
      // toggle shift direction if neccesarry
      if ( blocks_position - 2 * EDGE_SPACING > 0 ) shift_direction_up = false;
      if ( blocks_position + EDGE_SPACING < 0 ) shift_direction_up = true;

      led_timestamp = millis();
    }
  }
  if ( save[0][3] == HYBRID_4 || save[1][3] == HYBRID_4 ) { // fading sectors
    if (millis() - led_timestamp > 20) {
      for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
        if (fade_sectors[i] > 0) {
          fade_sectors[i]--;
        }

        if ( save[0][3] == HYBRID_4 ) {
          inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * fade_sectors[i], (save[0][1] / 10) * fade_sectors[i], (save[0][2] / 10) * fade_sectors[i]));
        }
        if ( save[1][3] == HYBRID_4 ) {
          outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * fade_sectors[i], (save[1][1] / 10) * fade_sectors[i], (save[1][2] / 10) * fade_sectors[i]));
        }
      }
      led_timestamp = millis();
    }
  }

  // flash every light
  if ( save[0][3] == FLASH || save[1][3] == FLASH) {
    for ( int i = 0; i < PIXEL_COUNT; i++ ) {
      if (  save[0][3] == FLASH ) {
        inner_pixels.setPixelColor(i, inner_pixels.Color(save[0][0], save[0][1], save[0][2]));
      }
      if ( save[1][3] == FLASH ) {
        outer_pixels.setPixelColor(i, inner_pixels.Color(save[1][0], save[1][1], save[1][2]));
      }
    }
  }

  //turn every light off
  if ( save[0][3] == OFF || save[1][3] == OFF ) {
    for ( int i = 0; i < PIXEL_COUNT; i++ ) {
      if (  save[0][3] == OFF ) {
        inner_pixels.setPixelColor(i, inner_pixels.Color(0, 0, 0));
      }
      if ( save[1][3] == OFF ) {
        outer_pixels.setPixelColor(i, inner_pixels.Color(0, 0, 0));
      }
    }
  }

  inner_pixels.show();
  outer_pixels.show();
}

void spawn_fade_sector() {
  int random_spot = random(0, PIXEL_COUNT);
  int random_length = random(5, 35);
  for ( int i = random_spot; i < random_spot + random_length; i++ ) {
    fade_sectors[i] = 10;
  }
}

void spawn_snake() {
  for (int i = 10; i > 0; i--) {
    snake[i] = i;
  }
}

void spawn_rain_drop() {
  int random_spot;
  random_spot = random(EDGE_SPACING, PIXEL_COUNT - EDGE_SPACING);
  rain_drops[random_spot][0] = 10;
  rain_drops[random_spot][1] =  2;
}

void turn_shifting_blocks_direction() {
  shift_direction_up = !shift_direction_up;
}
