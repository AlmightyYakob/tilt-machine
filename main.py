import serial
import subprocess

XRANDR_HORIZONTAL_MODE = "normal"
XRANDR_VERTICAL_MODE = "left"
XRANDR_OUTPUT_DEVICE = "HDMI-0"

HORIZONTAL = 0
VERTICAL = 1


def rotate(currentPosition):
    # TODO: Use python-xlib instead
    rotation = XRANDR_HORIZONTAL_MODE if currentPosition == 0 else XRANDR_VERTICAL_MODE

    command = ["xrandr", "--output", XRANDR_OUTPUT_DEVICE, "--rotate", rotation]
    subprocess.check_call(command)


def main():
    currentPosition = HORIZONTAL
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

        if tiltLength >= 500:
            currentPosition = 1 - currentPosition
            tiltLength = 0

            rotate(currentPosition)


if __name__ == "__main__":
    serialPort = serial.Serial(port="/dev/ttyACM0", baudrate=9600)
    main()
