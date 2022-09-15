// input pins
#define BUTTON_ISLANDS 4
#define BUTTON_ROWS 9

#define BUTTON_COUNT (BUTTON_ISLANDS * BUTTON_ROWS)

#define ISLAND_ONE   22
#define ISLAND_TWO   23
#define ISLAND_THREE 24
#define ISLAND_FOUR  25

#define ROW_ONE	   28
#define ROW_TWO	   29
#define ROW_THREE	 30
#define ROW_FOUR	 31
#define ROW_FIVE	 32
#define ROW_SIX	   33
#define ROW_SEVEN	 34
#define ROW_EIGHT	 35
#define ROW_NINE	 36

#define SOLO_BUTTON_ONE   38
#define SOLO_BUTTON_TWO   39
#define SOLO_BUTTON_THREE 40
#define SOLO_BUTTON_FOUR  41

#define AUDIO_JACK        A3

#define MICROPHONE A1

#define POTENTIOMETER A2
// input pins

// saves
// The saves array contains 8 saves
// 0 ->   save used for one-time-overrides like flashing or quick animations
// 1-5 -> button saves
// 6 ->   flash save: everything in here is flashed
// 7 ->   mute save:  everything in here is muted

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

#define NUM_SAVES 8
#define NUM_LIGHTS 9
#define LIGHT_SAVE_SPACE 4

void change_values_in_write_to_save_for_each_active_light(int, int, int, int, int);
// saves

// available animations
// normal animations
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


// quickanimations
#define FILLING_COUNTDOWN 253
#define ERASING_COUNTDOWN 252

#define FLASH 254
#define OFF   255
// available animations

// feedback
#define FEEDBACK_MODE_OFF    0
#define FEEDBACK_MODE_LED    1
#define FEEDBACK_MODE_SERIAL 2
#define FEEDBACK_MODE_DUAL   3

void write_feedback(int mode);
// feedback
