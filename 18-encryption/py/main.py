"""Converts a file using a rot13 algorithm"""

import sys
import rot13

def main():
    """Run the programme"""

    if len(sys.argv) < 2:
        print("Provide the name of a file to convert")
        sys.exit()

    filename = sys.argv[1]

    try:
        file = open(filename, "r+")
        text = file.read()
        file.seek(0) # go to start of file
        file.write(rot13.convert(text))
        file.close()
        print("Converted {}".format(filename))
    except IOError:
        print("Can't open file")

if __name__ == '__main__':
    main()
