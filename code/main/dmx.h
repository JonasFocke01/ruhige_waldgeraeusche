#include <DmxSimple.h>

//setup
#define DMX_DATA_PIN 3

#define FIRST_RGBW_CHANNEL 0
#define COUNT_RGBW_CHANNEL 10
#define LASER_CHANNEL 100
#define FIRST_STROBO_CHANNEL 200
#define COUNT_STORBO_CHANNEL 5
#define FIRST_SINGLE_CHANNEL 300
#define COUNT_SINGEL_CHANNEL 20
//setup

//utility functions
void dmx_channels_init(int);
void dmx_change_theme(int);
//utility functions
