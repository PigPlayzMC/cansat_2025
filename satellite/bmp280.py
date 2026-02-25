# BMP280 Readings program
import board as b
import busio as bus
import adafruit_bmp280 as bmp280

addr = 0x77

i2c = bus.I2C(b.GP15, b.GP14)

bmp280_sensor = bmp280.Adafruit_BMP280_I2C(i2c, address=addr)

print("BMP280 ready!")

def temperature(): # INT
    return bmp280_sensor.temperature

def pressure(): # INT
    return bmp280_sensor.pressure
