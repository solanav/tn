import socket

PORT = 7878
BUF_SIZE = 1024

def main():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(("127.0.0.1", PORT))

    while 1:
        try:
            (data, addr) = s.recvfrom(BUF_SIZE)
            print("[{}] > [{}]".format(addr, data))
        except Exception as e:
            print(e)

if __name__ == "__main__":
    main()