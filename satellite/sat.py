from satellite import bmp280

temperature = bmp280.read_temperature()

print(temperature)