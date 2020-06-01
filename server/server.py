import socket

PORT = 7878
BUF_SIZE = 1024

def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(("0.0.0.0", PORT))

    while 1:
        (data, addr) = s.recvfrom(BUF_SIZE)
        print("[{}] > [{}]".format(addr, data))

if __name__ == "__main__":
    main()