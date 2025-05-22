
# Tutorial 10 - Asynchronous Programming - Brodcast Chat Application

![server](/screenshot/server.png)

![client1](/screenshot/client1.png)

![client2](/screenshot/client2.png)

![client3](/screenshot/client3.png)

The chat system consists of one server and multiple clients. I successfully compiled and launched the server using the command cargo run --bin server, which listens on port 2000. Then, I opened three separate terminals and started three instances of the client using cargo run --bin client.

Each client successfully connected to the server and received a welcome message. I tested the messaging functionality by typing text into one client, and I observed that the message was instantly broadcasted and displayed in all other client terminals. This demonstrates that the asynchronous broadcast system works as intended, allowing real-time message delivery across multiple clients.