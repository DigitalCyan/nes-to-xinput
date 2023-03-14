# nes-to-xinput
## About
*nes-to-xinput* is a neat little budnle consisting of two parts. An Arduno program and a bridge program that reads the data from the Arduino and translates it into XInput equivalents to be sent to a virtual XInput controller via [ViGEm](https://github.com/ViGEm/ViGEm.github.io).

## Requirements
* An Arduino or an Arduino-like development board
* Some wires
* A USB capable of connecting your Arduino to the PC
* ViGEm Bus Driver ([Download](https://vigem.org/Downloads/https://github.com/ViGEm/ViGEmBus/releases))
* Rust ([Download](https://vigem.org/Downloads/https://github.com/ViGEm/ViGEmBus/releases))
* Arduino IDE ([Download](https://www.arduino.cc/en/software))

## Setup
1. Make sure you have all the requirements in check including the ViGEm Bus Driver.
2. Clone the project.
3. Open the Arduno sketch (`source/arduino/nes.ino`) and upload it to your Arduino.
4. Connect the wires. You can find a pinout for your specific NES controller on the internet but it boils down to these pins:

    * VCC +5V
    * GND
    * Clock
    * Latch
    * Data

    Connect the pins on the arduino to the controller as follows:

    * +5V -> VCC +5V
    * GND -> GND
    * D4 -> Clock
    * D3 -> Latch
    * D2 -> Output
5. Open up a terminal and `cd` into `source/bridge` and run `cargo run -- <path_to_serial>`.

    Example: `cargo run -- COM1`

6. If no error pops up that should be it, you can test your controller [here](https://gamepad-tester.com/).

# Extra
If you want, you can build the bridge by running `cargo build` and run it as an usual binary. That way you can have the setup be standalone from the project.