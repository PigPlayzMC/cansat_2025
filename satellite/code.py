import digitalio as dg
import board as b
import time as t
import adafruit_ticks as tk
import math as m

# Internal imports
import bmp280
import radio

# Sanity ensuring measures
true = True
false = False

# Constants (ish)
SEA_LEVEL_PRESSURE = 101325

led = dg.DigitalInOut(b.GP25)
led.direction = dg.Direction.OUTPUT

last_data = []

print("CanSat ready!")
radio.send("CanSat ready!")

def calculate_altitude(pressure):
    # Barometric formula (troposphere)
    return (1 - (pressure / SEA_LEVEL_PRESSURE) ** (1 / 5.25588)) / 2.25577e-5

while true:
    led.value = false
    t.sleep(0.5)
    led.value = true
    t.sleep(0.5)

    current_time = tk.ticks_ms() / 1000 - 536788

    temperature = bmp280.temperature()
    pressure = bmp280.pressure()
    altitude = calculate_altitude(pressure * 100) # BMP280 outputs in hPa

    if last_data != []:
        delta_alt = altitude - last_data[2]
        
        print(f"{current_time}, {last_data[4]}")
        
        delta_time = tk.ticks_diff(m.floor(current_time), m.floor(last_data[4])) # Should be 1 in an ideal world

        velocity = delta_alt / delta_time
    else:
        velocity = 0

    #print(f"Temperature: {temperature}˚c\nPressure: {pressure}\nAltitude: {altitude}")

    # CSV wants: time, speed, temp, alt, pressure
    radio.send(f"{current_time}, {velocity}, {temperature}, {altitude}, {pressure}")

    last_data = [temperature, pressure, altitude, current_time, velocity]
    
    print("Updated last_data[]")