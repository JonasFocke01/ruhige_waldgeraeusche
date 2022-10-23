# Ruhige-Waldgeräusche

## The Two main Parts of this project are divided into a fronend and a backend

### The Backend

The Backend is capable of writing to dmx and to led strips.
This task itself is divided into more parts.

- REST interface -> The input to the logic of the rpi is routet through a rest-api backend.
- DMX writing -> The rpi-python code sends an array of dmx values to an arduino which in turn translates and writes them directly into dmx.
- LED writing -> The rpi-python code writes arrays to connected led stripes.
- Audio analytics -> On the rpi itself runns a script which analyses the incomming audio stream from the onboard 3.5mm audiojack to detect the current beat and pitch.
- Logic -> The rpi translates and renders the incomming rest instructions into the dmx and led output arrays like a real lightcontroller.

### The Frontend

The Frontend consists of a website written with vuejs which itself contains a bit of logic which makes the rest-api calls to the backend

# Using

You can download this repo and use it as you please, or you can customize the behavior by changing your local copy or contribute to this repository.

# Contributing

Any contributing is highly appreciated!
Feel free to raise pull requests or simply write ideas as an issue.

# Concept

The releaseversion of ruhige_waldgeräusche will feature an rpi as a brain and several extension arduinos as eg. the dmx sending device or a custom keyboard
