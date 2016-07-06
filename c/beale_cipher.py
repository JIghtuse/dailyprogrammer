#!/usr/bin/python3


def decrypt_and_print(key_path, message_path):
    key = []
    with open(key_path) as fp:
        line = fp.readlines()[0]
        key = ''.join((s[0] for s in line.split()))

    message = open(message_path).readlines()[0]
    for i in message.split(","):
        i = int(i) % len(key)
        print(key[i - 1][0], end='')
    print()


def main():
    import sys
    if len(sys.argv) != 3:
        sys.exit("usage: {} <key.txt> <encrypted_message.txt>".format(sys.argv[0]))

    decrypt_and_print(sys.argv[1], sys.argv[2])


if __name__ == "__main__":
    main()
