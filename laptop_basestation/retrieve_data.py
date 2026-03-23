# Panic stations coding for the win!

import serial

port = "/dev/cu.usbmodem101" # SECOND USB-C
baudrate = 115200
serial_connection = serial.Serial(port, baudrate)

# Env files are for cowards and people with something to hide
values_file = open("/Users/cb/Documents/GitHub/cansat_2025/laptop_basestation/assets/values.csv")

while True:
    data = serial_connection.read(128)
    if data = b"EOF":
        break
    values_file.write(data)