#include "dmx.h"
#include "utilities.h"

unsigned long dmx_timestamp;
int dmx_color_store[3];
int dmx_snake[COUNT_RGBW_CHANNEL * 4];
int dmx_snake_count;

void dmx_channels_init(int theme) {

  DmxSimple.usePin(DMX_DATA_PIN);

  dmx_change_theme(theme);

  dmx_timestamp = millis();
}

void dmx_setup_snake(int _speed) {

    int tail_length;

    if (_speed == SPEED_SLOW) {
        dmx_snake_count = DMX_SNAKE_COUNT_SLOW;
        tail_length = DMX_TAIL_LENGTH_SLOW;
    } else if (_speed == SPEED_MEDIUM) {
        dmx_snake_count = DMX_SNAKE_COUNT_MEDIUM;
        tail_length = DMX_TAIL_LENGTH_MEDIUM;
    } else if (_speed == SPEED_FAST) {
        dmx_snake_count = DMX_SNAKE_COUNT_FAST;
        tail_length = DMX_TAIL_LENGTH_FAST;
    }

  for (int i = 0; i < dmx_snake_count; i++) {
    for (int j = tail_length; j > 0; j--) {
      dmx_snake[((COUNT_RGBW_CHANNEL * 4) - 1 - j) - (i * ((COUNT_RGBW_CHANNEL * 4) / dmx_snake_count))] = map_from_to(map_from_to(j, 0, tail_length, tail_length, 0), 0, tail_length, 0, 10);
    }
  }
}

void dmx_loop_snake(int _speed) {
  if (millis() - dmx_timestamp > _speed * 10)
  {
    for (int i = (COUNT_RGBW_CHANNEL * 4) - 1; i > -1; i--)
    {
      if (i == (COUNT_RGBW_CHANNEL * 4) - 1)
      {
        dmx_snake[0] = dmx_snake[(COUNT_RGBW_CHANNEL * 4) - 1];
      }
      else
      {
        dmx_snake[i + 1] = dmx_snake[i];
      }
    }

    for (int i = 0; i < sizeof(dmx_snake) / 2; i++)
    {
        DmxSimple.write(i, 25 * dmx_snake[i]);
    }
    dmx_timestamp = millis();
  }
}

void dmx_change_theme(int theme) {
  if (theme == 0) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 0;
  } else if (theme == 1) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 0;
  } else if (theme == 2) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 0;
  } else if (theme == 3) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 25;
  } else if (theme == 4) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 25;
  } else if (theme == 5) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 25;
  } else if (theme == 6) {
    dmx_color_store[0] = 25;
    dmx_color_store[1] = 25;
    dmx_color_store[2] = 25;
  } else if (theme == 7) {
    dmx_color_store[0] = 0;
    dmx_color_store[1] = 0;
    dmx_color_store[2] = 0;
  }  
}

void dmx_write_RGBW(int device_channel, int r, int g, int b, int w) {
    DmxSimple.write(device_channel, r);
    DmxSimple.write(device_channel + 1, g);
    DmxSimple.write(device_channel + 2, b);
    DmxSimple.write(device_channel + 3, w);
}
