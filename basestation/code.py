import rfm9x

packet_count = 0

while True:
    recieved_data = rfm9x.read()

    if recieved_data is not None:
        print(f"RADIO {packet_count}: {str(radio_message, 'ascii')}")
        rssi = rfm9x.get_rssi()
        print(f"RSSI: {rssi}")

        if rssi < -90:
            print(f"WARNING: RSSI ({rssi}) below minumum value!")


