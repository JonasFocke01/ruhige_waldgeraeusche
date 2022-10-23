from rpi_ws281x import *
import time as t
import random
from logic.utilities import *
import json
import threading

with open("config.json") as json_data_file:
    config = json.load(json_data_file)

# LED strip configuration:
LED_COUNT        = config["leds"]["led_count"]
LED_PIN          = config["leds"]["led_pin"]
LED_FREQ_HZ      = config["leds"]["led_freq_hz"]
LED_DMA          = config["leds"]["led_dma"]
LED_BRIGHTNESS   = config["leds"]["led_brightness"]
LED_INVERT       = config["leds"]["led_invert"]
LED_CHANNEL      = config["leds"]["led_channel"]
STRIP_COUNT      = config["leds"]["strip_count"]
PIXEL_OFFSET     = config["leds"]["pixel_offset"]
FRAME_TIMING     = config["leds"]["frame_timing"]
PIXELS_PER_STRIP = int(LED_COUNT / STRIP_COUNT)
physical_strip   = Adafruit_NeoPixel(LED_COUNT, LED_PIN, LED_FREQ_HZ,LED_DMA,LED_INVERT,LED_BRIGHTNESS,LED_CHANNEL)

# strips array where each strip contains
# ? velocity ( 0 -> stationary, != 0 -> how many pixels should this move on each iteration)
# ? brightness (between 0 and 1)
# ? fading (between 0 and MAX, where the number is chopped off of the brightness each iteration)
PARAMETER_COUNT = 3

# timestamps array contains the following timestamps
# ? render_timestamp
timestamps = [current_time_in_millis()] * 1

strips = []

def led_setup():
    physical_strip.begin()
    for i in range(STRIP_COUNT):
        strips.append([])
        for _ in range(PIXELS_PER_STRIP):
            strips[i].append([0, 0, 0])

inherit_color = True
strips_colors = {
    "red": 50,
    "green": 50,
    "blue": 50
}

# @parameters r, g, b: this parameters represent the colors, the pixels should be rendered in
def change_rendercolors(r, g, b):
    strips_colors["red"] = int(r)
    strips_colors["green"] = int(g)
    strips_colors["blue"] = int(b)

def color_mode_white():
    global inherit_color
    inherit_color = False

def color_mode_inherit():
    global inherit_color
    inherit_color = True

move_individualy = False
strip_to_display_on = 0

def randomize_strip_to_display_on():
    global strip_to_display_on
    strip_to_display_on = random.randrange(STRIP_COUNT)

def switch_to_individual_movement():
    global move_individualy
    move_individualy = True

def switch_to_group_movement():
    global move_individualy
    move_individualy = False

move_up = True
move_down = False
move_sideways = False

def toggle_move_up():
    global move_up
    move_up = not move_up

def toggle_move_down():
    global move_down
    move_down = not move_down

def toggle_move_sideways():
    global move_sideways
    move_sideways = not move_sideways
    
active_animation = 'off'

def animation_trigger():
    if active_animation == 'off':
        pass
    elif active_animation == 'snake':
        animation_snake()

def toggle_active_animation_off():
    global active_animation
    active_animation = 'off'

# spawns a snake
# @param strip: on which strip should the snake spawn where -1 is all. default: -1
# @param speed: how fast should the snake go. default 1
def animation_snake(strip_num = -1, speed = 1):
    global active_animation
    active_animation = 'snake'
    strip_to_display_on
    randomize_strip_to_display_on()
    if strip_num == -1:
        if move_individualy:
            if len(strips[strip_to_display_on]) > 15:
                for j in range(15):
                    strips[strip_to_display_on][j] = [speed, mapFromTo(j, 0, 15, 0, 1), 0]
        else:
            for strip_i in range(len(strips)):
                if len(strips[strip_i]) > 15:
                    for j in range(15):
                        strips[strip_i][j] = [speed, mapFromTo(j, 0, 15, 0, 1), 0]

def flash_fade():
    if move_individualy:
        for pixel_i in range(len(strips[strip_to_display_on])):
            strips[strip_to_display_on][pixel_i] = [0, 1, 0.1]
    else:
        for strip_i in range(len(strips)):
            for pixel_i in range(len(strips[strip_i])):
                strips[strip_i][pixel_i] = [0, 1, 0.1]

# this function processes strips array to print it to the physical led strips
def render():
    while True:
        if current_time_in_millis() - timestamps[0] > FRAME_TIMING:
            # fading brightness
            for strip_i in range(len(strips)):
                for pixels_i in range(len(strips[strip_i])):
                    strips[strip_i][pixels_i][1] -= strips[strip_i][pixels_i][2]

            # moving
            temp_strips = []
            for i in range(STRIP_COUNT):
                temp_strips.append([])
                for _ in range(PIXELS_PER_STRIP):
                    temp_strips[i].append([0, 0, 0])

            for strip_i in range(len(strips)):
                for pixel_i in range(len(strips[strip_i])):
                    if len(strips[strip_i][pixel_i]) > 1 and strips[strip_i][pixel_i][1] > 0:
                        try:
                            temp_strips[strip_i][pixel_i + strips[strip_i][pixel_i][0]] = strips[strip_i][pixel_i]
                        except IndexError:
                            pass

            for strip_i in range(len(strips)):
                strips[strip_i] = temp_strips[strip_i]

            # drawing
            for strip_i in range(len(strips)):
                for pixel_i in range(len(strips[0])):
                    red   = 0
                    green = 0
                    blue  = 0
                    if inherit_color:
                        red   = int(strips_colors["red"]   * strips[strip_i][pixel_i][1])
                        green = int(strips_colors["green"] * strips[strip_i][pixel_i][1])
                        blue  = int(strips_colors["blue"] * strips[strip_i][pixel_i][1])
                    else:
                        red   = int(250 * strips[strip_i][pixel_i][1])
                        green = int(250 * strips[strip_i][pixel_i][1])
                        blue  = int(250 * strips[strip_i][pixel_i][1])
                    if strip_i % 2 == 0:
                        position = int(mapFromTo(pixel_i, 0, PIXELS_PER_STRIP - 1, (strip_i * PIXELS_PER_STRIP), ((strip_i * PIXELS_PER_STRIP) + PIXELS_PER_STRIP) - 1))
                    else:
                        position = int(mapFromTo(pixel_i, 0, PIXELS_PER_STRIP - 1, ((strip_i * PIXELS_PER_STRIP) + PIXELS_PER_STRIP) - 1, (strip_i * PIXELS_PER_STRIP)))
                    physical_strip.setPixelColor(position , Color(red, green, blue))


            physical_strip.show()

            timestamps[0]  = current_time_in_millis()

class Renderthread (threading.Thread):
    def __init__(self):
        threading.Thread.__init__(self)
    def run(self):
        t.sleep(2)
        render()

threadLock = threading.Lock()



if __name__ == '__main__':
    print("starting...")
    led_setup()
    render_thread = Renderthread()
    render_thread.start()
    while True:
        spawn_snake(speed = 2)
        change_rendercolors(random.randrange(0, 255), random.randrange(0, 255), random.randrange(0, 255))
        t.sleep(2.5)
    