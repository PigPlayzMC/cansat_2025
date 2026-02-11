import board as b
import busio as bus
import adafruit_bmp280 as ada_bmp280

i2c = bus.I2C(b.GP15, b.GP14)
bmp280 = ada_bmp280.Adafruit_BMP280_I2C(i2c)

def read_temperature():
    return bmp280.temperature

def read_pressure():
    return bmp280.pressure

def read_temp_pressure_array():
    return [bmp280.temperature, bmp280.pressure]

def display_environment():
    print(f"Temperature: {bmp280.temperature}˚c\nPressure: {bmp280.pressure}")
