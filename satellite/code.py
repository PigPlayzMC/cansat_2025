import digitalio as dg
import board as b
import time as t

# Internal imports
import bmp280
import radio

# Sanity ensuring measures
true = True
false = False

led = dg.DigitalInOut(b.GP25)
led.direction = dg.Direction.OUTPUT

print("CanSat ready!")
radio.send("CanSat ready!")

while true:
    led.value = false
    t.sleep(0.5)
    led.value = true
    t.sleep(0.5)

    temperature = bmp280.temperature()
    pressure = bmp280.pressure()
    print(f"Temperature: {temperature}˚c\nPressure: {pressure}")
