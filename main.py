import serial  # type: ignore

from util import HORIZONTAL, TILT_LENGTH_THRESHOLD, determine_starting_position, rotate


def main(starting_position=HORIZONTAL):
    currentPosition = starting_position
    tiltLength = 0

    while True:
        if serialPort.in_waiting:
            tilting = None

            try:
                string = serialPort.readline().decode().strip()
                tilting = int(string) != currentPosition
            except UnicodeDecodeError:
                pass
            except ValueError:
                pass

            if tilting:
                tiltLength += 1
            else:
                tiltLength = 0

        if tiltLength >= TILT_LENGTH_THRESHOLD:
            currentPosition = 1 - currentPosition
            tiltLength = 0

            rotate(currentPosition)


if __name__ == "__main__":
    serialPort = serial.Serial(port="/dev/ttyACM0", baudrate=9600)

    print("Determining initial monitor orientation...")
    starting_position = determine_starting_position(serialPort)

    print("Starting tilt monitoring service...")
    main(starting_position)
