# onewire-file

To run:
1. Connect ds18b20 temperature sensors
2. Run `$ sudo dtoverlay w1-gpio gpiopin=<PIN> pullup=0` for all pins  with a sensor connected
3. Run `$ cargo run`
