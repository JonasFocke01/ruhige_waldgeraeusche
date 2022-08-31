#include <DmxSimple.h>
#include "controller.h"

// setup
#define DMX_DATA_PIN 3

#define STAGE_LIGHTS_CHANNEL 1
#define MOVING_HEADS_RIGHT_CHANNEL 4
#define MOVING_HEADS_LEFT_CHANNEL 7
#define SPECIAL_SLOT_ONE_CHANNEL 8
#define SPECIAL_SLOT_TWO_CHANNEL 9
#define SPECIAL_SLOT_THREE_CHANNEL 10
#define LASER_CHANNEL 11
#define STROBE_CHANNEL 12
// setup

// functions
void dmx_channels_init();
void dmx_loop(uint8_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
void set_strobe_mode(bool);
// functions
