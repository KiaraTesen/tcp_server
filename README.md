# TCP Rust server

TCP Rust server that accept serialized struct.

## Usage

The program receives 4 arguments:

* ```--ip``` : IP address of the server.
* ```--port``` : Service port.
* ```--service``` : Service type.
* ```--subnet``` : IP addresses allowed to receive service.

## Examples

PSO server service : 

```
cargo run -- --ip 127.0.0.1 --port 8888 --service PSO --subnet 127.0.0 
```

DE server service

```
cargo run -- --ip 127.0.0.1 --port 8888 --service DE --subnet 127.0.0 
```

LP server service:

```
cargo run -- --ip 127.0.0.1 --port 8888 --service LP --subnet 127.0.0 
```
