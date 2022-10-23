import serial
import serial.tools.list_ports
import time

ARDUINO_PID = 29987



def get_arduino_port():
    ports = list(serial.tools.list_ports.comports())
    for entry in ports:
        if ARDUINO_PID == entry.pid:
            return entry.device
    return False

if __name__ == '__main__':
    #while True:
    ser = False
    while not ser:
        ser = get_arduino_port()
    ser = serial.Serial(ser, 9600, rtscts=True)
    print("connected")
    while True:
        print(ser)
        print("writing")
        # ser.reset_input_buffer()
        testsees = 69
        ser.write(testsees.to_bytes(1, 'little'))
        testsees = 67
        ser.write(testsees.to_bytes(1, 'little'))
        testsees = 125
        ser.write(testsees.to_bytes(1, 'little'))
        testsees = 254
        ser.write(testsees.to_bytes(1, 'little'))
        testsees = 123
        ser.write(testsees.to_bytes(1, 'little'))
        time.sleep(5)
        print("waiting for data")
        # while ser.in_waiting == 0:
        #     pass
        print(ser.read(1))
        print(ser.in_waiting)
    # ser.reset_input_buffer()
    # ser.write(bytearray([4, 7, 3, 250, 137]))