import socket
import time

UDP_IP = "127.0.0.1"
UDP_PORT = 4242

sock = socket.socket(socket.AF_INET, # Internet
                     socket.SOCK_DGRAM) # UDP
sock.bind((UDP_IP, UDP_PORT))
start_time = time.time()

while True:
    data, addr = sock.recvfrom(1500) # buffer size is 1024 bytes
    
    print (time.time() - start_time)
    start_time = time.time()