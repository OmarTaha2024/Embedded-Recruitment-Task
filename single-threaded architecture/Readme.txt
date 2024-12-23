Bugs Identified in server.rs:
--Infinite Loop with Clients (Client::handle)
--Improper Message Decoding Error Handling
Limitations of the Single-threaded Architecture:
--Scalability Issues:
In the original single-threaded implementation, a single thread handled both client connections and message processing, leading to a bottleneck for multiple concurrent clients.
--Blocking Operations
A single slow client ( one with poor network performance) could block the handling of all other clients.
--Lack of Resource Optimization
Sleeping for a fixed time during the non-blocking listener loop wastes CPU resources, especially under high or variable loads.