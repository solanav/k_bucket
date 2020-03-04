import socket
from datetime import datetime

HOST='192.168.35.1'
PORT=9393
MAX_MSG=512

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.bind((HOST, PORT))

sock.listen(1)

while True:
    connection, client_address = sock.accept()
    data = connection.recv(MAX_MSG)
    print("[{}] [{}]: {}".format(datetime.now(), client_address[0], data.decode()))