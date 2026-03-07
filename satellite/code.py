import digitalio as dg
import board as b
import time as t

# Internal imports
import bmp280
import radio

# Sanity ensuring measures
true = True
false = False

# Constants (ish)
SEA_LEVEL_PRESSURE = 101325 # Conversion from Pa to hPa

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
    altitude = calculate_altitude(pressure * 100)
    #print(f"Temperature: {temperature}˚c\nPressure: {pressure}")

    radio.send(f"{temperature} {pressure}")


def calculate_altitude(pressure):
    # Barometric formula (troposphere)
    return (1 - (pressure / SEA_LEVEL_PRESSURE) ** (1 / 5.25588)) / 2.25577e-5