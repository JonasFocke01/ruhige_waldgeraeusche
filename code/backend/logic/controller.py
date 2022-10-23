import json
from logic.led_strips import *

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
        while True:
            t.sleep(1)
    except RuntimeError:
        pass

if __name__ == '__main__':
    start_controller()