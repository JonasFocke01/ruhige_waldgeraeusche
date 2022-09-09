// inputs
#define DETECT_CLICKS_LENGTH_IN_MS 500

#define BUTTON_COUNT 66

#define TESTINPUT_one_out   34
#define TESTINPUT_one_in    35
#define TESTINPUT_two_out   38
#define TESTINPUT_two_in    39
#define TESTINPUT_three_out 42
#define TESTINPUT_three_in  43

//TODO: This are only dummy values. This needs to be worked out!
#define STROBE_TOGGLE 1
#define STROBE_FREQUENCY_POTENTIOMETER 1
#define BLINDER_POTENTIOMETER 1
#define SPECIAL_SLOT_ONE_POTENTIOMETER 1
#define SPECIAL_SLOT_TWO_POTENTIOMETER 1
#define SPECIAL_SLOT_THREE_POTENTIOMETER 1
#define RED_POTENTIOMETER 1
#define GREEN_POTENTIOMETER 1
#define BLUE_POTENTIOMETER 1
// inputs

// saves
// The saves array contains 7 saves
// 0 -> save used for one-time-overrides like flashing or quick animations
// 1-5 -> button saves

// Each save contians configurations for each available light
// The lights are
// 0 -> inner_leds
// 1 -> outer_leds
// 2 -> stage_lights
// 3 -> moving_heads_left
// 4 -> moving_heads_right
// 5 -> blinder
// 6 -> laser
// 7 -> special_slot_one
// 8 -> special_slot_two
// 9 -> special_slot_three

// A save is constructed like this
// 0 -> r 
// 1 -> g
// 2 -> b
// 3 -> animation

#define NUM_SAVES 7
#define NUM_LIGHTS 9
#define LIGHT_SAVE_SPACE 4

void change_values_in_write_to_save_for_each_active_light(int, int, int, int, int);
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

// feedback
#define FEEDBACK_MODE_OFF    0
#define FEEDBACK_MODE_LED    1
#define FEEDBACK_MODE_SERIAL 2
#define FEEDBACK_MODE_DUAL   3

void write_feedback(int mode);
// feedback
