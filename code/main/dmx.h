#include <DmxSimple.h>

//setup
#define DMX_DATA_PIN 3

#define STAGE_LIGHTS_CHANNEL 0
#define STROBE_CHANNEL 100
//setup

//utility functions
void dmx_channels_init();
void dmx_main_loop(int, int, int, bool);
//utility functions
