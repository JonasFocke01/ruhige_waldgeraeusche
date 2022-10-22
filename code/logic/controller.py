import json
from led_strips import *
# from input import *
# from dmx import *

render_thread = Renderthread()

def start_controller():
    with open("config.json") as json_data_file:
        config = json.load(json_data_file)

    BUTTON_COUNT = config['input']['button_count']

    button_states = [0] * BUTTON_COUNT

    # led stuff
    try:
        render_thread.start()
        led_setup()
    except RuntimeError:
        pass
    while True:
        t.sleep(1)

if __name__ == '__main__':
    start_controller()
    # led_setup()
    # while True:
        # button_states = read_input()
        # parse_input()
        # calculate_leds()
        # caluclate_dmx()
        # write_leds()
        # write_dmx()