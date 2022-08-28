#include <stdlib.h>
#include "led_strip.h"
#include "utilities.h"

unsigned long led_timestamp;
int snake[PIXEL_COUNT], blocks[PIXEL_COUNT], blocks_position = 0, rain_drops[PIXEL_COUNT][2], fade_sectors[PIXEL_COUNT][3];
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
void led_setup_snake() {

  memset(snake, 0, sizeof(snake));

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }

  for (int i = 0; i < SNAKE_COUNT; i++) {
    for (int j = TAIL_LENGTH; j > 0; j--) {
      snake[(PIXEL_COUNT - 1 - j) - (i * (PIXEL_COUNT / SNAKE_COUNT))] = map_from_to(map_from_to(j, 0, TAIL_LENGTH, TAIL_LENGTH, 0), 0, TAIL_LENGTH, 0, 10);
    }
  }
}

void led_loop_snake(int r, int g, int b){
  int i_wrapped;
  bool wrapped = false;
  if (millis() - led_timestamp > 10)
  {
    for (int i = PIXEL_COUNT - 1; i > -1; i--)
    {
      i_wrapped = i + 10;
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

void led_loop_shifting_blocks(int r, int g, int b) {
  if (millis() - led_timestamp > 10){
    led_timestamp = millis();
    
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
      pixels.setPixelColor(i + blocks_position, pixels.Color((r / 10) * blocks[j], (g / 10) * blocks[j], (b / 10) * blocks[j]));
      j++;
    }
    pixels.show();
  }
}
//shifting blocks theme

//rain drops
void led_setup_rain_drops() {
  
  memset(rain_drops, 0, sizeof(rain_drops));

  for (int i = 0; i < PIXEL_COUNT; i++)
  {
    pixels.setPixelColor(i, pixels.Color(0, 0, 0));
  }
}

void led_loop_rain_drops(int r, int g, int b) {

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
    led_timestamp = millis();

    //draw waves
    for ( int i = 0; i < PIXEL_COUNT; i++) {
      pixels.setPixelColor(i, pixels.Color((r / 10) * rain_drops[i][0], (g / 10) * rain_drops[i][0], (b / 10) * rain_drops[i][0]));
    }
    pixels.show();
  }
}
//rain drops

//spawn fade sectors
void led_erase_existing_fade_sectors() {
  memset(fade_sectors, 0, sizeof(fade_sectors));
}

void led_spawn_fade_sector(int first_pixel, int sector_length, int r, int g, int b) {
  for ( int i = first_pixel; i < first_pixel + sector_length; i++ ) {
    fade_sectors[i][0] = r;
    fade_sectors[i][1] = g;
    fade_sectors[i][2] = b;
  }
}

void led_fade_out_sectors() {   
  for ( int i = 0; i < PIXEL_COUNT - 1; i++ ) {
    if (fade_sectors[i][0] > 0) { fade_sectors[i][0] -= 1; }
    if (fade_sectors[i][1] > 0) { fade_sectors[i][1] -= 1; }
    if (fade_sectors[i][2] > 0) { fade_sectors[i][2] -= 1; }

    //draw sectors
    pixels.setPixelColor(i, pixels.Color(fade_sectors[i][0], fade_sectors[i][1], fade_sectors[i][2]));
  } 
  pixels.show();
}
//spawn fade sectors
