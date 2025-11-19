import machine; # type: ignore
# ^ Perfectly fine, Python checker just dislikes it.
import time;

# === Pins ===
temperature_pin = machine.ADC(4) # Internal temperature sensor
pressure_pin = machine.ADC(0) # Placeholder
transmission_pin = machine.ADC(0) # Placeholder
led_pin = Pin("LED", Pin.OUT) # type: ignore

# === Conversions ===
# Not all sensors output in a sensible format...
temperature_conversion = 3.3 / (65535) # From Vbe voltage to degrees C (with later offset)

# === Dependent variables ===
lifetime = True

# === Thresholds ===
threshhold_altitude = 100 # Placeholder

# === Definitions ===
def get_temperature():
    temperature = temperature_pin.read_u16()
    temperature = temperature * temperature_conversion

    temperature = 27 - (temperature - 0.706) / 0.001721

    return temperature
# END

def transmit_data():
    led_pin.toggle() # On - Allows for debugging. Remove if annoying.

    return_data = [] # Placeholder values

    transmission_pin.value(return_data) # Possible implementation

    led_pin.toggle() # Off
# END

while lifetime:
    temperature = get_temperature()
    
    transmit_data() # Placeheld
# END



print("Satellite landed.")