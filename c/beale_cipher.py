#!/usr/bin/python3


def decrypt_and_print(key_path, message_path):
    words = open(key_path).read().split()
    positions = open(message_path).read().split(",")
    positions = (int(i) % len(words) for i in positions)
    message = ''.join(words[i - 1][0] for i in positions)
    print(message)


def main():
    import sys
    if len(sys.argv) != 3:
        sys.exit("usage: {} <key.txt> <encrypted_message.txt>".format(sys.argv[0]))

    decrypt_and_print(sys.argv[1], sys.argv[2])


if __name__ == "__main__":
    main()
