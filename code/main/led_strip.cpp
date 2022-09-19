#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"

unsigned long led_timestamp, countdown_timestamp;

int snake[PIXEL_COUNT], blocks[PIXEL_COUNT], blocks_position = 0, rain_drops[PIXEL_COUNT][2], fade_sectors[PIXEL_COUNT], likelyhood, countdown_state = 0, pitch_position = 0;
bool shift_direction_up = true, erase_countdown_bool = false;
int j = 0, brightness = 0;
Adafruit_NeoPixel pixels;

void led_setup() {

  randomSeed(analogRead(9)); // 9 is an unconnected analog pin

  pixels = Adafruit_NeoPixel(PIXEL_COUNT, LED_STRIP_PIN, NEO_GRB + NEO_KHZ800);

  led_timestamp = millis();

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

  pixels.begin();

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }
}

void led_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {
  j = 0;

  if ( save[1][3] == ALL_ON ) {
    for ( int i = PIXEL_OFFSET; i < PIXEL_COUNT - 1; i++ ) {
      pixels.setPixelColor(i, pixels.Color(save[0][0], save[0][1], save[0][2]));
    }
  }
  if ( save[0][3] == HYBRID_1 ) { // snake
    for ( int i = 0; i < 5; i++ ) {
      for (int j = PIXEL_COUNT - 2; j > -1; j--)
      {
        snake[j + 1] = snake[j];
      }
    }

    snake[PIXEL_COUNT - 1] = 0;
    snake[1] = 0;
    snake[2] = 0;
    snake[3] = 0;

    for (int i = PIXEL_OFFSET; i < sizeof(snake) / 2; i++)
    {
      if ( save[0][3] == HYBRID_1 ) {
        pixels.setPixelColor(i, pixels.Color((save[0][0] / 10) * snake[i], (save[0][1] / 10) * snake[i], (save[0][2] / 10) * snake[i]));
      }
    }
  }
  if ( save[0][3] == HYBRID_2 || save[1][3] == HYBRID_2 ) { // raindrops
    if (millis() - led_timestamp > 1) {

      //progress waves
      rain_drops[0][0] = 0;
      rain_drops[PIXEL_COUNT - 1][0] = 0;
      for (int i = PIXEL_OFFSET; i < PIXEL_COUNT - 3; i++) {
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
      for ( int i = PIXEL_OFFSET; i < PIXEL_COUNT - 1; i++) {
        if ( save[0][3] == HYBRID_2 ) {
          pixels.setPixelColor(i, pixels.Color((save[0][0] / 10) * rain_drops[i][0], (save[0][1] / 10) * rain_drops[i][0], (save[0][2] / 10) * rain_drops[i][0]));
        }
      }
      led_timestamp = millis();
    }
  }
  if ( save[0][3] == HYBRID_3 ) { // shifting blocks
    if (millis() - led_timestamp > 5) {

      // shift blocks
      if ( shift_direction_up ) {
        blocks_position += 1;
      } else {
        blocks_position -= 1;
      }

      //draw blocks
      for ( int i = 0; i < PIXEL_COUNT - 1; i++) {
        if ( save[0][3] == HYBRID_3 ) {
          pixels.setPixelColor(i + blocks_position, pixels.Color((save[0][0] / 10) * blocks[j], (save[0][1] / 10) * blocks[j], (save[0][2] / 10) * blocks[j]));
        }
        j++;
      }

      // toggle shift direction if neccesarry
      if ( blocks_position - 2 * EDGE_SPACING > 0 ) shift_direction_up = false;
      if ( blocks_position + EDGE_SPACING < 0 ) shift_direction_up = true;

      led_timestamp = millis();
    }
  }
  if ( save[0][3] == DROP_1 ) { // fixed blocks
    //draw blocks
    for ( int i = 0; i < PIXEL_COUNT; i++) {
      if ( save[0][3] == DROP_1 ) {
        pixels.setPixelColor(i + blocks_position, pixels.Color((save[0][0] / 10) * blocks[j], (save[0][1] / 10) * blocks[j], (save[0][2] / 10) * blocks[j]));
      }
      j++;
    }
  }
  if ( save[0][3] == HYBRID_4 || save[0][3] == DROP_2 ) { // fading sectors
    if (millis() - led_timestamp > 20) {
      for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
        if (fade_sectors[i] > 0) {
          fade_sectors[i]--;
        }
        pixels.setPixelColor(i, pixels.Color((save[0][0] / 10) * fade_sectors[i], (save[0][1] / 10) * fade_sectors[i], (save[0][2] / 10) * fade_sectors[i]));
      }
      led_timestamp = millis();
    }
  }
  if ( save[0][3] == BRIZZLE ) { // brizzle
    if ( brightness > 0 ) {
      brightness = 0;
    } else {
      brightness = 255;
    }
    for (int i = 0; i < PIXEL_COUNT - 1; i++) {
      if (  save[0][3] == BRIZZLE ) {
        pixels.setPixelColor(i, pixels.Color(brightness, brightness, brightness));
      }
    }
  }

  // flash every light
  if ( save[0][3] == FLASH ) {
    for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
      if ( save[0][3] == FLASH ) {
        pixels.setPixelColor(i, pixels.Color(save[0][0], save[0][1], save[0][2]));
      }
    }
  }

  // turn every light off
  if ( save[0][3] == OFF ) {
    for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
      if ( save[0][3] == OFF ) {
        pixels.setPixelColor(i, pixels.Color(0, 0, 0));
      }
    }
  }

  // draw countdown
  if ( countdown_state > 0 ) {
    for ( int i = 0; i < countdown_state; i++ ) {
      pixels.setPixelColor(i, pixels.Color(save[0][0], save[0][1], save[0][2]));
      if ( i > PIXEL_COUNT - 2 ) break;
    }
  }

  pixels.show();
}

void spawn_fade_sector(int mode) {
  if ( mode == 0 ) {
    int random_spot = random(PIXEL_OFFSET, PIXEL_COUNT - 36);
    int random_length = random(5, 35);
    for ( int i = random_spot; i < random_spot + random_length; i++ ) {
      fade_sectors[i] = 10;
    }
  } else {
    for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
      fade_sectors[i] = 10;
    }
  }
}

void spawn_snake() {
  for (int i = 10; i > 0; i--) {
    snake[i + PIXEL_OFFSET] = i;
  }
}

void spawn_rain_drop() {
  int random_spot;
  random_spot = random(EDGE_SPACING + PIXEL_OFFSET, PIXEL_COUNT - EDGE_SPACING);
  rain_drops[random_spot][0] = 10;
  rain_drops[random_spot][1] =  2;
}

void turn_shifting_blocks_direction() {
  shift_direction_up = !shift_direction_up;
}

void fill_pixels(int percentage) {
  countdown_state = map( percentage, 0, 100, PIXEL_OFFSET, PIXEL_COUNT - 1 );
}

void erase_pixels() {
  countdown_state = 0;
  pitch_position  = 0;
}

void set_pitch_position(int pitch) {
  pitch_position = map( pitch, 0, PIXEL_COUNT, PIXEL_OFFSET, PIXEL_COUNT );
}

void invert_shifting_blocks() {
  for (int i = 0; i < PIXEL_COUNT - 1; i++) {
    if ( blocks[i] > 0 ) {
      blocks[i] = 0;
    } else {
      blocks[i] = 10;
    }
  }
}
