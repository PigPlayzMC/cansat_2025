# Panic stations coding for the win!

import serial

port = "/dev/cu.usbmodem101" # SECOND USB-C
baudrate = 115200
serial_connection = serial.Serial(port, baudrate)

# Env files are for cowards and people with something to hide
values_file = open("/Users/cb/Documents/GitHub/cansat_2025/laptop_basestation/assets/values.csv", 'w')
values_file.write("time_sec,speed_m_s,temperature_C,altitude_m,pressure_hPa\r\n")
values_file.close()

while True:
    data = serial_connection.read(60)
    if data == b"EOF":
        break

    print(str(data, 'ascii'))
    values_file = open("/Users/cb/Documents/GitHub/cansat_2025/laptop_basestation/assets/values.csv", 'a')
    values_file.write(str(data, 'ascii'))
    values_file.close()