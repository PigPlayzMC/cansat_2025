import digitalio as d
import board as b
import busio as bus
import adafruit_rfm9x as ada_rfm9x

spi = bus.SPI(b.GP2, b.GP3, b.GP4)

cs = d.DigitalInOut(b.GP6)
reset = d.DigitalInOut(b.GP7)

frequency = 433.0

rfm9x = ada_rfm9x.RFM9x(spi, cs, reset, frequency)

print("RFM9x ready!")

def send(data):
    rfm9x.send(data)
