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
SEA_LEVEL_PRESSURE = 101325

led = dg.DigitalInOut(b.GP25)
led.direction = dg.Direction.OUTPUT

print("CanSat ready!")
radio.send("CanSat ready!")

while true:
    led.value = false
    t.sleep(0.5)
    led.value = true
    t.sleep(0.5)

    current_time = t.ticks_ms()

    temperature = bmp280.temperature()
    pressure = bmp280.pressure()
    altitude = calculate_altitude(pressure * 100) # BMP280 outputs in hPa

    if last_data is not None:
        delta_alt = altitude - last_data[2]
        delta_time = time.ticks_diff(current_time, last_data[3] / 1000) # Should be 1 in an ideal world

        velocity = delta_alt / delta_time
    else:
        velocity = 0

    #print(f"Temperature: {temperature}˚c\nPressure: {pressure}\nAltitude: {altitude}")

    # CSV wants: time, speed, temp, alt, pressure
    radio.send(f"{current_time}, {velocity}, {temperature}, {altitude}, {pressure}")

    last_data = [temperature, pressure, altitude, current_time, velocity]


def calculate_altitude(pressure):
    # Barometric formula (troposphere)
    return (1 - (pressure / SEA_LEVEL_PRESSURE) ** (1 / 5.25588)) / 2.25577e-5