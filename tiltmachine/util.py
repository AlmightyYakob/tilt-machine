import os
import sys
import serial  # type: ignore
import subprocess
from pathlib import Path

XRANDR_HORIZONTAL_MODE = "normal"
XRANDR_VERTICAL_MODE = "left"
XRANDR_OUTPUT_DEVICE = "HDMI-0"

HORIZONTAL = 0
VERTICAL = 1

# How many times in a row the arduino has to register a tilt before rotating the screen
TILT_LENGTH_THRESHOLD = 500

# How many times in a row the arduino has to register the starting position
INIT_ORIENTATION_THRESHOLD = 200

# Extra commands, which are not checked for completion, and are run in the background
additional_rotate_commands = [
    [str(Path(os.environ["HOME"]).absolute() / ".config" / "polybar" / "launch.sh")]
]


def determine_starting_position(serialPort: serial.Serial):
    # Wait for the first data to come through
    serialPort.read()

    tiltLength = 0
    currentOrientation = None
    while tiltLength < INIT_ORIENTATION_THRESHOLD:
        orientation = None

        if serialPort.in_waiting:
            try:
                orientation = int(serialPort.readline().decode().strip())
            except UnicodeDecodeError:
                pass
            except ValueError:
                pass

            if orientation is not None:
                if orientation == currentOrientation:
                    tiltLength += 1
                else:
                    currentOrientation = orientation
                    tiltLength = 0

    return currentOrientation


def spawn_daemon(command):
    # Do the UNIX double-fork magic, see Stevens' "Advanced
    # Programming in the UNIX Environment" for details (ISBN 0201563177)
    # This is necessary so that if this program exits, child
    # processes will continue to run

    try:
        pid = os.fork()
        if pid > 0:
            # Parent process, return and keep running
            return

    except OSError as e:
        print(sys.stderr, f"fork #1 failed: {e.errno} ({e.strerror})")
        sys.exit(1)

    os.setsid()

    # Do second fork
    try:
        pid = os.fork()
        if pid > 0:
            # Exit from second parent
            sys.exit(0)
    except OSError as e:
        print(sys.stderr, f"fork #2 failed: {e.errno} ({e.strerror})")
        sys.exit(1)

    # Actually run the process now
    subprocess.Popen(
        command, close_fds=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )

    # all done
    os._exit(os.EX_OK)


def rotate(currentPosition):
    # TODO: Use python-xlib instead
    rotation = XRANDR_HORIZONTAL_MODE if currentPosition == 0 else XRANDR_VERTICAL_MODE

    command = ["xrandr", "--output", XRANDR_OUTPUT_DEVICE, "--rotate", rotation]
    subprocess.check_call(command)

    for extra_command in additional_rotate_commands:
        spawn_daemon(extra_command)
