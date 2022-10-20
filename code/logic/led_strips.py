from rpi_ws281x import *
import time as t
import random
from utilities import *
import json
import math
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
strips_colors = {
    "red": 0,
    "green": 0,
    "blue": 0
}

def led_setup():
    physical_strip.begin()
    for i in range(STRIP_COUNT):
        strips.append([])
        for _ in range(PIXELS_PER_STRIP):
            strips[i].append([0, 0, 0])

# @parameters r, g, b: this parameters represent the colors, the pixels should be rendered in
def change_rendercolors(r, g, b):
    strips_colors["red"] = r
    strips_colors["green"] = g
    strips_colors["blue"] = b


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
                for pixel_i in range(PIXELS_PER_STRIP):
                    if strips[strip_i][pixel_i][1] > 0:
                        try:
                            temp_strips[strip_i][pixel_i + strips[strip_i][pixel_i][0]] = strips[strip_i][pixel_i]
                        except IndexError:
                            pass

            for strip_i in range(len(strips)):
                strips[strip_i] = temp_strips[strip_i]

            # drawing
            for strip_i in range(len(strips)):
                for pixel_i in range(len(strips[0])):
                    if strip_i % 2 == 0:
                        physical_strip.setPixelColor(int(mapFromTo(pixel_i, 0, PIXELS_PER_STRIP - 1, (strip_i * PIXELS_PER_STRIP), ((strip_i * PIXELS_PER_STRIP) + PIXELS_PER_STRIP) - 1)) , Color(int(strips_colors["red"] * strips[strip_i][pixel_i][1]), int(strips_colors["green"] * strips[strip_i][pixel_i][1]), int(strips_colors["blue"] * strips[strip_i][pixel_i][1])))
                    else:
                        physical_strip.setPixelColor(int(mapFromTo(pixel_i, 0, PIXELS_PER_STRIP - 1, ((strip_i * PIXELS_PER_STRIP) + PIXELS_PER_STRIP) - 1, (strip_i * PIXELS_PER_STRIP))) , Color(int(strips_colors["red"] * strips[strip_i][pixel_i][1]), int(strips_colors["green"] * strips[strip_i][pixel_i][1]), int(strips_colors["blue"] * strips[strip_i][pixel_i][1])))


            physical_strip.show()

            timestamps[0]  = current_time_in_millis()

class Renderthread (threading.Thread):
    def __init__(self):
        threading.Thread.__init__(self)
    def run(self):
        render()

# spawns a snake
# @param strip: on which strip should the snake spawn where -1 is all. default: -1
# @param speed: how fast should the snake go. default 1
def spawn_snake(strip_num = -1, speed = 1):
    if strip_num == -1:
        for strip_i in range(len(strips)):
            for j in range(30):
                strips[strip_i][j] = [speed, mapFromTo(j, 0, 30, 0, 1), 0]

      

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
    