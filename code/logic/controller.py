import json
# from input import *
# from led_strips import *
# from dmx import *

with open("config.json") as json_data_file:
    config = json.load(json_data_file)

BUTTON_COUNT = config['input']['button_count']

button_states = [0] * BUTTON_COUNT

if __name__ == '__main__':
    led_setup()
    while True:
        # button_states = read_input()
        # parse_input()
        # calculate_leds()
        # caluclate_dmx()
        # write_leds()
        # write_dmx()