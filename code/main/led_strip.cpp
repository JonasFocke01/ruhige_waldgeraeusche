#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"
#include "controller.h"

unsigned long led_timestamp;

int snake[PIXEL_COUNT], blocks[PIXEL_COUNT], blocks_position = 0, rain_drops[PIXEL_COUNT][2], fade_sectors[PIXEL_COUNT][3], likelyhood;
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
      snake[(PIXEL_COUNT - 1 - j) - (i * (PIXEL_COUNT / SNAKE_COUNT))] = map_from_to(map_from_to(j, 0, TAIL_LENGTH, TAIL_LENGTH, 0), 0, TAIL_LENGTH, 0, 10);
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

void led_loop(uint8_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]) {

  if ( save[0][3] == HYBRID_1 || save[1][3] == HYBRID_1 ) { // snake
    int i_wrapped;
    bool wrapped = false;
    if (millis() - led_timestamp > 10)
    {
      for (int i = PIXEL_COUNT - 1; i > -1; i--)
      {
        i_wrapped = i + 5;
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
        if ( save[0][3] == HYBRID_1 ) {
          inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * snake[i], (save[0][1] / 10) * snake[i], (save[0][2] / 10) * snake[i]));
        } 
        if ( save[1][3] == HYBRID_1 ){
          outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * snake[i], (save[1][1] / 10) * snake[i], (save[1][2] / 10) * snake[i]));        
        }
      }
      led_timestamp = millis();
    }
  } 
  if ( save[0][3] == HYBRID_2 || save[1][3] == HYBRID_2 ) { // raindrops
    int random_spot;
  
    if (millis() - led_timestamp > 10){
  
      //progress waves
      rain_drops[0][0] = 0;
      rain_drops[PIXEL_COUNT - 1][0] = 0;
      for (int i = 1; i < PIXEL_COUNT - 1; i++) {
        if (rain_drops[i][0] > 0) {
          if (rain_drops[i][1] == 2) {
            rain_drops[i + 1][0] = 10;
            rain_drops[i + 1][1] = 0;
            rain_drops[i - 1][0] = 10;
            rain_drops[i - 1][1] = 1;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
          } else if (rain_drops[i][1] == 0) {
            rain_drops[i + 1][0] = 10;
            rain_drops[i + 1][1] = 0;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
            i++;
          } 
          else if (rain_drops[i][1] == 1) {
            rain_drops[i - 1][0] = 10;
            rain_drops[i - 1][1] = 1;
            rain_drops[i][0] = 0;
            rain_drops[i][1] = 0;
          }
        }
      }
  
      //spawn new raindrop
      if (random(0, 100) > 80){
        random_spot = random(EDGE_SPACING, PIXEL_COUNT - EDGE_SPACING);
        rain_drops[random_spot][0] = 10;
        rain_drops[random_spot][1] =  2;
      }
  
      //draw waves
      for ( int i = 0; i < PIXEL_COUNT; i++) {
        if ( save[0][3] == HYBRID_2 ) {
          inner_pixels.setPixelColor(i, inner_pixels.Color((save[0][0] / 10) * rain_drops[i][0], (save[0][1] / 10) * rain_drops[i][0], (save[0][2] / 10) * rain_drops[i][0]));
        } 
        if ( save[1][3] == HYBRID_2 ) {
          outer_pixels.setPixelColor(i, outer_pixels.Color((save[1][0] / 10) * rain_drops[i][0], (save[1][1] / 10) * rain_drops[i][0], (save[1][2] / 10) * rain_drops[i][0]));
        }
      }
      led_timestamp = millis();
    }
  }
  if ( save[0][3] == HYBRID_3 || save[1][3] == HYBRID_3 ) { // shifting blocks
    if (millis() - led_timestamp > 10){
      if ( shift_direction_up && blocks_position == EDGE_SPACING ) {
        shift_direction_up = false;
      } else if ( shift_direction_up == false && blocks_position == -8  ){
        shift_direction_up = true;
      }
    
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
      led_timestamp = millis();
    }
  } 
  if ( save[0][3] == HYBRID_4 || save[1][3] == HYBRID_4 ) { // fading sectors
     for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
      if (fade_sectors[i][0] > 0) { fade_sectors[i][0] -= 1; }
      if (fade_sectors[i][1] > 0) { fade_sectors[i][1] -= 1; }
      if (fade_sectors[i][2] > 0) { fade_sectors[i][2] -= 1; }

      if ( save[0][3] == HYBRID_4 ) {
        inner_pixels.setPixelColor(i, inner_pixels.Color(fade_sectors[i][0], fade_sectors[i][1], fade_sectors[i][2]));
      } 
      if ( save[1][3] == HYBRID_4 ) {
        outer_pixels.setPixelColor(i, outer_pixels.Color(fade_sectors[i][0], fade_sectors[i][1], fade_sectors[i][2]));
      }
    }
    if (random(0, 100) > 90){
      int random_spot = random(0, PIXEL_COUNT);
      int random_length = random(5, 35);
      for ( int i = random_spot; i < random_spot + random_length; i++ ) {
          fade_sectors[i][0] = save[0][0];
          fade_sectors[i][1] = save[0][1];
          fade_sectors[i][2] = save[0][2];
          
          fade_sectors[i][0] = save[1][0];
          fade_sectors[i][1] = save[2][1];
          fade_sectors[i][2] = save[3][2];
        }
      }
    }
  if ( save[0][3] == OFF || save[1][3] == OFF ) { // this is not what off means
    if ( millis() - led_timestamp > 2500 ) {
      int random_red =   random(0, 100);
      int random_green = random(0, 100);
      int random_blue =  random(0, 100);
      for ( int i = 0; i < PIXEL_COUNT; i++ ) {
          if (  save[0][3] == OFF ) {
            inner_pixels.setPixelColor(i, inner_pixels.Color(random_red, random_blue, random_green));
          }
          if ( save[1][3] == OFF ) {
            outer_pixels.setPixelColor(i, inner_pixels.Color(random_red, random_blue, random_green));
          }
      }
      led_timestamp = millis();
    }
  }

  inner_pixels.show();
  outer_pixels.show();
}
