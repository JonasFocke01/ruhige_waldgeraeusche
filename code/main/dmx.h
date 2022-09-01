#include <DmxSimple.h>
#include "controller.h"

// setup
#define DMX_DATA_PIN 3

#define STAGE_LIGHTS_CHANNEL        1
#define MOVING_HEADS_RIGHT_CHANNEL  4
#define MOVING_HEADS_LEFT_CHANNEL   7
#define SPECIAL_SLOT_ONE_CHANNEL   10
#define SPECIAL_SLOT_TWO_CHANNEL   11
#define SPECIAL_SLOT_THREE_CHANNEL 12
#define LASER_CHANNEL              13
#define STROBE_CHANNEL             14
#define AMBIENCE_LIGHT_CHANNEL     15
// setup

// functions
void dmx_channels_init();
void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
void set_strobe_mode(bool);
void set_strobe_frequency();
void trigger_animations(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
// functions
