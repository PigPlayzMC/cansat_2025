import time
import math

# =========================
# CONFIGURATION
# =========================

THRESHOLD_ALTITUDE = 100      # meters
LAUNCH_VELOCITY = 5           # m/s
DESCENT_VELOCITY = -2         # m/s
LANDING_VELOCITY = 0.2        # m/s threshold
SEA_LEVEL_PRESSURE = 101325   # Pa

# =========================
# STATE VARIABLES
# =========================

lifetime = True
launched = False
parachute_deployed = False
legs_deployed = False

last_data = None              # [altitude, velocity, time]
last_velocities = []

# =========================
# SENSOR FUNCTIONS (REPLACE)
# =========================

def read_pressure():
    # Replace with real sensor reading
    return 101325

def read_temperature():
    # Replace with real sensor reading
    return 20

# =========================
# ACTUATORS (REPLACE)
# =========================

def deploy_parachute():
    print("Parachute deployed")
    # Trigger GPIO here
    return True

def deploy_legs():
    print("Legs deployed")
    # Trigger GPIO here
    return True

def transmit_data(data):
    # Replace with LoRa / Radio transmit
    print("TX:", data)

# =========================
# FLIGHT LOOP
# =========================

def calculate_altitude(pressure):
    # Barometric formula (troposphere)
    return (1 - (pressure / SEA_LEVEL_PRESSURE) ** (1 / 5.25588)) / 2.25577e-5


def main():
    global lifetime, launched, parachute_deployed
    global legs_deployed, last_data, last_velocities

    while lifetime:

        pressure = read_pressure()
        temperature = read_temperature()

        altitude = calculate_altitude(pressure)

        current_time = time.ticks_ms()

        # -------------------------
        # VELOCITY CALCULATION
        # -------------------------
        if last_data is not None:
            delta_alt = altitude - last_data[0]
            delta_time = time.ticks_diff(current_time, last_data[2]) / 1000  # seconds

            if delta_time > 0:
                velocity = delta_alt / delta_time
            else:
                velocity = 0
        else:
            velocity = 0

        # -------------------------
        # LAUNCH DETECTION
        # -------------------------
        if not launched and abs(velocity) > LAUNCH_VELOCITY:
            launched = True
            print("Launch detected")

        # -------------------------
        # PARACHUTE DEPLOYMENT
        # -------------------------
        if (launched and
            altitude > THRESHOLD_ALTITUDE and
            velocity < DESCENT_VELOCITY and
            not parachute_deployed):

            parachute_deployed = deploy_parachute()
            legs_deployed = deploy_legs()

        # Backup safety deployment
        if altitude > 300 and not parachute_deployed:
            parachute_deployed = deploy_parachute()

        # -------------------------
        # LANDING DETECTION
        # -------------------------
        if launched and abs(velocity) < LANDING_VELOCITY:
            if len(last_velocities) == 5 and sum(abs(v) for v in last_velocities) < 0.5:
                lifetime = False
                print("Landing detected")

        # -------------------------
        # TRANSMIT DATA
        # -------------------------
        transmit_data([
            pressure,
            temperature,
            altitude,
            velocity,
            current_time
        ])

        # -------------------------
        # STORE HISTORY
        # -------------------------
        if len(last_velocities) >= 5:
            last_velocities.pop()

        last_velocities.insert(0, velocity)

        last_data = [altitude, velocity, current_time]

        time.sleep(1)

    print("Mission complete")


# =========================
# START
# =========================

main()
