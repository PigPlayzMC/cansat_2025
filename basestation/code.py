import rfm9x

packet_count = 0

while True:
    #print("Reading")
    recieved_data = rfm9x.read()

    if recieved_data is not None:
        #print(f"RADIO {packet_count}: {str(recieved_data, 'ascii')}")
        #rssi = rfm9x.get_rssi()
        #print(f"RSSI: {rssi}")

        # This is how we serial send data
        print(f"{packet_count}, {str(recieved_data, 'ascii')}")

        packet_count += 1;
