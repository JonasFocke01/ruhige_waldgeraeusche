# Ruhige-Waldgeräusche is a lightcontroller wich is, depending on the verison, capable of

- sending via DMX
- directly adressing lightstrips
- changing colors on the fly
- changing animations on the fly
- displaying quickanimations
- some other stuff a lightcontroller needs

# Using

You can download this repo and use it as you please, or you can customize the behavior by changing your local copy or contribute to this repository.

 ## Installing

 - install rust/cargo
 - enable uart port:
    - sudo nano /boot/config.txt
    - enable_uart=1
    - -> reboot

# Contributing

Any contributing is highly appreciated!
Feel free to raise pull requests or simply write ideas as an issue.

# Versioning

This is explained, because the versioning in this repo is handled differently to be able to work on the different verisons simultaneously.
Each Releaseversion of rw lives in its own branch, which is labeld as the version itself.
