# Ruhige-Waldgeräusche is a lightcontroller wich is, capable of

- sending via DMX
- directly adressing lightstrips
- changing colors on the fly
- changing animations on the fly
- some other stuff a lightcontroller needs

# Using

You can download this repo and use it as you please, or you can customize the behavior by changing your local copy or contribute to this repository.
At the current development state, the rust code must be run as admin: "sudo -E /home/[USER]/.cargo/bin/cargo run"

 ## Installing

 - install rust/cargo
 - enable uart port:
    - sudo nano /boot/config.txt
    - enable_uart=1
    - -> reboot

# Contributing

Any contributing is highly appreciated!
Feel free to raise pull requests or simply write ideas as an issue.