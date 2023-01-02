import time as t
from rpi_ws281x import *
import json
import sys

print(sys.argv)

config_file_path = ""

if (len(sys.argv) > 1):
    config_file_path = "../config.json"
else:
    config_file_path = "config.json"

with open(config_file_path) as json_data_file:
    config = json.load(json_data_file)

LED_COUNT_PER_STRIP = config["leds"]["led_count_per_strip"]
STRIP_COUNT         = config["leds"]["strip_count"]
LED_COUNT           = LED_COUNT_PER_STRIP * STRIP_COUNT
LED_PIN             = config["leds"]["led_pin"]
LED_FREQ_HZ         = config["leds"]["led_freq_hz"]
LED_DMA             = config["leds"]["led_dma"]
LED_BRIGHTNESS      = config["leds"]["led_brightness"]
LED_INVERT          = config["leds"]["led_invert"]
LED_CHANNEL         = config["leds"]["led_channel"]
PIXEL_OFFSET        = config["leds"]["pixel_offset"]
FRAME_TIMING        = config["leds"]["frame_timing"]

physical_strip      = Adafruit_NeoPixel(LED_COUNT, LED_PIN, LED_FREQ_HZ,LED_DMA,LED_INVERT,LED_BRIGHTNESS,LED_CHANNEL)

strips = []

physical_strip.begin()

while 1:
    try:
        std_input = input()
        start = t.time()
        std_input = std_input.split(" ")
        print(std_input)
        assert len(std_input) == LED_COUNT * 3
        for strip_i in range(STRIP_COUNT):
            for pixel_i in range(LED_COUNT_PER_STRIP):
                physical_strip.setPixelColor(pixel_i , Color(int(std_input[strip_i * pixel_i]), int(std_input[strip_i * pixel_i + 1]), int(std_input[strip_i * pixel_i + 2])))
        physical_strip.show()
        print(t.time() - start)
    except AssertionError:
        print("Wrong input size!")
    except KeyboardInterrupt:
        print("See you soon!")
        exit(0)
    except:
        print("Unknown exception occured")