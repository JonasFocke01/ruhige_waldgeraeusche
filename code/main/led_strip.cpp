#include <stdlib.h>
#include "led_strip.h"
#include "controller.h"

unsigned long led_timestamp, countdown_timestamp;

int snake[PIXEL_COUNT], blocks[PIXEL_COUNT], blocks_position = 0, rain_drops[PIXEL_COUNT][2], fade_sectors[PIXEL_COUNT], likelyhood, countdown_state = 0, pitch_position = 0;
bool shift_direction_up = true, erase_countdown_bool = false;
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
      for (int i = 2; i < PIXEL_COUNT - 3; i++) {
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
      for ( int i = 0; i < PIXEL_COUNT - 1; i++) {
        if ( save[0][3] == HYBRID_3 ) {
          inner_pixels.setPixelColor(i + blocks_position, inner_pixels.Color((save[0][0] / 10) * blocks[j], (save[0][1] / 10) * blocks[j], (save[0][2] / 10) * blocks[j]));
        }
        if ( save[1][3] == HYBRID_3 ) {
          outer_pixels.setPixelColor(i + blocks_position, outer_pixels.Color((save[1][0] / 10) * blocks[j], (save[1][1] / 10) * blocks[j], (save[1][2] / 10) * blocks[j]));
        }
        j++;
      }

      // toggle shift direction if neccesarry
      if ( blocks_position - 2 * EDGE_SPACING > 0 ) shift_direction_up = false;
      if ( blocks_position + EDGE_SPACING < 0 ) shift_direction_up = true;

      led_timestamp = millis();
    }
  }
  if ( save[0][3] == DROP_1 || save[1][3] == DROP_1 ) { // fixed blocks
    //draw blocks
    int j = 0;
    for ( int i = 0; i < PIXEL_COUNT; i++) {
      if ( save[0][3] == DROP_1 ) {
        inner_pixels.setPixelColor(i + blocks_position, inner_pixels.Color((save[0][0] / 10) * blocks[j], (save[0][1] / 10) * blocks[j], (save[0][2] / 10) * blocks[j]));
      }
      if ( save[1][3] == DROP_1 ) {
        outer_pixels.setPixelColor(i + blocks_position, outer_pixels.Color((save[1][0] / 10) * blocks[j], (save[1][1] / 10) * blocks[j], (save[1][2] / 10) * blocks[j]));
      }
      j++;
    }
  }
  if ( save[0][3] == HYBRID_4 || save[1][3] == HYBRID_4 || save[0][3] == DROP_2 || save[1][3] == DROP_2 ) { // fading sectors
    if (millis() - led_timestamp > 20) {
      for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
        if (fade_sectors[i] > 0) {
          fade_sectors[i]--;
        }

        if ( save[0][3] == HYBRID_4 || save[0][3] == DROP_2 ) {
          inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * fade_sectors[i], (save[0][1] / 10) * fade_sectors[i], (save[0][2] / 10) * fade_sectors[i]));
        }
        if ( save[1][3] == HYBRID_4 || save[1][3] == DROP_2 ) {
          outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * fade_sectors[i], (save[1][1] / 10) * fade_sectors[i], (save[1][2] / 10) * fade_sectors[i]));
        }
      }
      led_timestamp = millis();
    }
  }
  if ( save[0][3] == BRIZZLE || save[1][3] == BRIZZLE ) { // brizzle
    int brightness = random(0, 255);
    for (int i = 0; i < PIXEL_COUNT - 1; i++) {
      if (  save[0][3] == BRIZZLE ) {
        inner_pixels.setPixelColor(i, inner_pixels.Color(brightness, brightness, brightness));
      }
      if ( save[1][3] == BRIZZLE ) {
        outer_pixels.setPixelColor(i, inner_pixels.Color(brightness, brightness, brightness));
      }
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

  // turn every light off
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



  // draw countdown
  if ( countdown_state > 0 ) {
    for ( int i = 0; i < countdown_state; i++ ) {
      inner_pixels.setPixelColor(i, inner_pixels.Color(save[0][0], save[0][1], save[0][2]));
      outer_pixels.setPixelColor(i, outer_pixels.Color(save[1][0], save[1][1], save[1][2]));
      if ( i > PIXEL_COUNT - 2 ) break;
    }
  }

  // draw pitch
  const uint8_t bleeding_amount = 5;
  if ( pitch_position > 0 ) {
    inner_pixels.setPixelColor(pitch_position, inner_pixels.Color(save[0][0], save[0][1], save[0][2]));
    outer_pixels.setPixelColor(pitch_position, outer_pixels.Color(save[1][0], save[1][1], save[1][2]));
    for ( int i = bleeding_amount; i > 0; i-- ) {
      if ( pitch_position - i < bleeding_amount || pitch_position + i > PIXEL_COUNT - bleeding_amount ) break;
      inner_pixels.setPixelColor(pitch_position - i, inner_pixels.Color((save[0][0] / 10) * i, (save[0][1] / 10) * i, (save[0][2] / 10) * i));
      inner_pixels.setPixelColor(pitch_position + i, inner_pixels.Color((save[0][0] / 10) * i, (save[0][1] / 10) * i, (save[0][2] / 10) * i));
      outer_pixels.setPixelColor(pitch_position - i, outer_pixels.Color((save[1][0] / 10) * i, (save[1][1] / 10) * i, (save[1][2] / 10) * i));
      outer_pixels.setPixelColor(pitch_position + i, outer_pixels.Color((save[1][0] / 10) * i, (save[1][1] / 10) * i, (save[1][2] / 10) * i));
    }
  }

  inner_pixels.show();
  outer_pixels.show();
}

void spawn_fade_sector(int mode) {
  if ( mode == 0 ) {
    int random_spot = random(0, PIXEL_COUNT);
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

void fill_pixels(int percentage) {
  countdown_state = map( percentage, 0, 100, 0, PIXEL_COUNT - 1 );
}

void erase_pixels() {
  countdown_state = 0;
  pitch_position  = 0;
}

void set_pitch_position(int pitch) {
  pitch_position = pitch;
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
