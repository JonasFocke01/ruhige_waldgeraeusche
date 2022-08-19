//strobe controll
#define STROBE_MODE_LED 0
#define STROBE_MODE_DEDICATED 1
//strobe controll

//theme controll
#define THEME_GREEN 0
#define THEME_GREEN_RED 1
#define THEME_RED 2
#define THEME_RED_BLUE 3
#define THEME_BLUE 4
#define THEME_BLUE_GREEN 5
#define THEME_WHITE 6
#define THEME_OFF 7
//theme controll

//animation presets
#define SNAKE 0 //snake runns through the leds, rest is random in the color sheme
#define CYCLING 1 //cycling through different color shades when theme is < 6, else random
#define RAIN_DROPS 2 //LED: random spot lights up and produces wave to the ends. REST: Lights up to max and fades out
#define FADING 3 //fades in and out where in is cubic out, and out is cubic in, so that the lights are bright most of the time and dim only shortly. Sort of bouncing from 0.
//animation presets

//speeds
#define SPEED_SLOW 4
#define SPEED_MEDIUM 2
#define SPEED_FAST 1
//speeds

//bpm modes
#define BPM_MODE_OFF 0
#define BPM_MODE_ON 1
//bpm modes
