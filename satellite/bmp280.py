import board # type: ignore
import busio # type: ignore
import adafruit_bmp280 # type: ignore

i2c = busio.I2c(scl = board.GP15, sda = board.GP14)

bmp280_sensor = adafruit_bmp280.Adafruit_BMP280_I2c(i2c)

def read_temperature():
    return bmp280_sensor.temperature

def read_pressure():
    return bmp280_sensor.pressure