#include <DmxSimple.h>
#include "controller.h"

// setup
#define DMX_DATA_PIN 3

#define STAGE_LIGHTS_CHANNEL        1
#define MOVING_HEADS_RIGHT_CHANNEL  4
#define MOVING_HEADS_LEFT_CHANNEL   25

// those are dummy values
#define SPECIAL_SLOT_ONE_CHANNEL   55
#define SPECIAL_SLOT_TWO_CHANNEL   56
#define SPECIAL_SLOT_THREE_CHANNEL 57
#define LASER_CHANNEL              58
#define STROBE_CHANNEL             59
#define AMBIENCE_LIGHT_CHANNEL     60
// setup

// functions
void dmx_channels_init();
void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
void set_strobe_mode(bool);
void set_strobe_frequency(int);
// functions
