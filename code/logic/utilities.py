import time

def mapFromTo(value, inMin, inMax, outMin, outMax):
    return outMin + (outMax - outMin) * ((value - inMin) / (inMax - inMin))

def current_time_in_millis():
    return time.time() * 1000