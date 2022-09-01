// inputs
#define DETECT_CLICKS_LENGTH_IN_MS 500

#define BUTTON_COUNT 66

#define TESTINPUT 2
// inputs

// saves
// The saves array contains 5 saves.

// Each save contians configurations for each available light
// The lights are
// 0 -> inner_leds
// 1 -> outer_leds
// 2 -> stage_lights
// 3 -> moving_heads_left
// 4 -> moving_heads_right
// 5 -> ambience_light
// 6 -> laser
// 7 -> special_slot_one
// 8 -> special_slot_two
// 9 -> special_slot_three

// A save is constructed like this
// 0 -> r 
// 1 -> g
// 2 -> b
// 3 -> animation

#define NUM_SAVES 5
#define NUM_LIGHTS 9
#define LIGHT_SAVE_SPACE 4

void change_values_in_save(int, int, int, int, int, int);
// saves

// available themes
#define DROP_1 0
#define DROP_2 1
#define DROP_3 2
#define DROP_4 3

#define HYBRID_1 4 // snake
#define HYBRID_2 5 // raindrops
#define HYBRID_3 6 // shifting blocks
#define HYBRID_4 7 // fading blocks

#define BRIGHT_1 8
#define BRIGHT_2 9
#define BRIGHT_3 10
#define BRIGHT_4 11

#define STROBE_1 12
#define STROBE_2 13
#define STROBE_3 14
#define STROBE_4 15

#define SETTLE_1 16
#define SETTLE_2 17
#define SETTLE_3 18
#define SETTLE_4 19

#define ANIMATION_COUNT 20

#define OFF 255
// available themes
