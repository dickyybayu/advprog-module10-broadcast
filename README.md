
# Tutorial 10 - Asynchronous Programming - Brodcast Chat Application

## Original code, and how it run

![server](/screenshot/server.png)

![client1](/screenshot/client1.png)

![client2](/screenshot/client2.png)

![client3](/screenshot/client3.png)

The chat system consists of one server and multiple clients. I successfully compiled and launched the server using the command cargo run --bin server, which listens on port 2000. Then, I opened three separate terminals and started three instances of the client using cargo run --bin client.

Each client successfully connected to the server and received a welcome message. I tested the messaging functionality by typing text into one client, and I observed that the message was instantly broadcasted and displayed in all other client terminals. This demonstrates that the asynchronous broadcast system works as intended, allowing real-time message delivery across multiple clients.

## Modifying Port

### Server port 2000 while Client port 8080 (Mismatch)

**Server port 2000**
![Image](/screenshot/server2000.png)

**Client port 8080**
![Image](/screenshot/clienterror.png)

In the first case, I deliberately introduced a mismatch between the server and client ports to observe how WebSocket handles incorrect configurations. The server was configured to listen on port 2000, while the client attempted to connect to port 8080.

As a result, the connection failed. The client threw a ConnectionRefused error, indicating that it was unable to establish a WebSocket handshake since no server was actively listening on the expected port. This behavior demonstrates that even if the application compiles and runs correctly, a mismatched port setup will prevent communication from being established.

This highlights the importance of ensuring that both the server and client agree on the same port for any TCP-based protocol like WebSocket to function properly.

### Server port 8080 while Client port 8080 (Match)

**Server port 8080**
![Image](/screenshot/server8080.png)

**Client port 8080**
![Image](/screenshot/clientsuccess.png)

In the second case, I corrected the configuration by ensuring that both the server and the client used the same port — 8080. The server was updated to listen on 127.0.0.1:8080, and the client was also configured to connect to ws://127.0.0.1:8080.

With matching ports, the client successfully established a connection. Upon connecting, it received a welcome message from the server, confirming that the WebSocket handshake completed properly and the asynchronous communication channel was functional.

This successful connection confirms that matching ports are essential for establishing reliable WebSocket communication. It also reinforces the idea that network-level details, like ports and IP addresses, must be aligned between parties to enable proper protocol negotiation.