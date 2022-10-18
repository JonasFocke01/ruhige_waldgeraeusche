from rpi_ws281x import *
import time as t
import random

# led array
rising = True
testindex = 0
led_array = []
for i in range(0, 150):
    led_array.append(i)

# LED strip configuration:
LED_COUNT      = 150      # Number of LED pixels.

#LED_PIN        = 18      # GPIO pin connected to the pixels (18 uses PWM!).
#LED_PIN        = 10      # GPIO pin connected to the pixels (10 uses SPI /dev/spidev0.0).
LED_PIN        = 21      # custom läuft mit channel 0
#meine idee ist, dass die led strips nicht an verschiedene pinne des rpi angeschlossen sein müssen.
#Die können auch in Reihe geschaltet und dann entsprechend angesprochen werden. So läuft alles über nur einen Pin am rpi
#Das ist aber noch nicht getestet!

LED_FREQ_HZ    = 800000  # LED signal frequency in hertz (usually 800khz)
LED_DMA        = 10      # DMA channel to use for generating signal (try 10)
LED_BRIGHTNESS = 255     # Set to 0 for darkest and 255 for brightest
LED_INVERT     = False   # True to invert the signal (when using NPN transistor level shift)
LED_CHANNEL    = 0       # set to '1' for GPIOs 13, 19, 41, 45 or 53

strip = Adafruit_NeoPixel(LED_COUNT, LED_PIN, LED_FREQ_HZ,LED_DMA,LED_INVERT,LED_BRIGHTNESS,LED_CHANNEL)
strip.begin()
while True:
    testindex += 1
    r = random.randrange(100, 250)
    for x in range(0,LED_COUNT - 1):
        strip.setPixelColor(x, Color(led_array[x], 0, 0))
        
    if testindex % 100 > 90:
        rising = not rising
    for index in range(0, LED_COUNT - 1):
        if rising:
            if led_array[index] < 240:
                led_array[index] += 10
        else:
            if led_array[index] > 10:
                led_array[index] -= 10

    strip.show()
    
    t.sleep(0.03)

# t.sleep(2)

# for x in range(0,LED_COUNT):
#     strip.setPixelColor(x,Color(0,0,0))

# strip.show()
