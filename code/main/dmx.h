#include <DmxSimple.h>
#include "controller.h"

// setup
#define DMX_DATA_PIN 3

#define STAGE_LIGHTS_CHANNEL        1
#define SCANNER_CHANNEL             4
#define LASER_CHANNEL              30
#define STROBE_CHANNEL            100

// those are dummy values
#define SPECIAL_SLOT_ONE_CHANNEL   55
#define SPECIAL_SLOT_TWO_CHANNEL   56
#define SPECIAL_SLOT_THREE_CHANNEL 57
#define AMBIENCE_LIGHT_CHANNEL     60
// setup

// functions
void dmx_channels_init();
void dmx_loop(uint16_t save[NUM_LIGHTS][LIGHT_SAVE_SPACE]);
void set_strobe_mode(bool);
void set_strobe_frequency(int);
// functions

// moving heads

// colors
#define  MV_WHITE        0
#define  MV_YELLOW      15
#define  MV_BLUE        30
#define  MV_GREEN       45
#define  MV_RED         57
#define  MV_PURPLE      71
#define  MV_DARK_BLUE   85
#define  MV_ORANGE      99
#define  MV_PINK       113

//msl mode
#define MV_MSL_YELLOW      8
#define MV_MSL_BLUE       38
#define MV_MSL_GREEN      68
#define MV_MSL_RED        98
#define MV_MSL_PURPLE    128
#define MV_MSL_DARK_BLUE 158
#define MV_MSL_ORANGE    188
#define MV_MSL_PINK      218
// moving heads

// laser
#define  LS_WHITE       56
#define  LS_RED          8
#define  LS_BLUE        32
#define  LS_YELLOW      24
#define  LS_GREEN       16
#define  LS_PURPLE      40
// laser
