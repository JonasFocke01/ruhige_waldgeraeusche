# Ruhige-Waldgeräusche is a lightcontroller wich is capable of

- sending via DMX
- directly adressing lightstrips

- running in predefined color sets
- running predefined light-action sets

# Using

You can download this repo and use it on your arduino as you please, or you can customize the behavior by changing your local copy or contribute to this repository.

 ## Installing

 - install rust/cargo
 - enable uart port:
    - sudo nano /boot/config.txt
    - enable_uart=1
    - -> reboot

# Contributing

Any contributing is highly appreciated!
Feel free to raise pull requests or simply write ideas as an issue.

# Concept

The releaseversion of ruhige_waldgeräusche ( 1.0 ) will feature an arduino as keyboard, an rpi running rust for the logic bits and another arduino to connect to dmx and to led.