import digitalio as d
import board as b
import time as t

# Local imports
import bmp280
import rfm9x

# QoL
true = True
false = False

# LED for sanity check
led = d.DigitalInOut(board.LED)
led.direction = d.Direction.OUTPUT

while true:
    led.value = false

    t.sleep(0.5)
    led.value = true

    # Collect data
    bmp280.display_environment() # TODO Remove debug
    data = bmp280.read_temp_pressure_array()

    # Transmit data
    rfm9x.send(data)

    t.sleep(0.5)
