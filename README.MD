
# Simple stress test tool for websockets

```
Usage: ws_stress [OPTIONS] --addr <ADDR> [MESSAGE]

Arguments:
  [MESSAGE]  Message to send [default: "hello world!"]

Options:
  -a, --addr <ADDR>
          The websocket address to connect to, e.g. ws://0.0.0.0:8080
  -c, --connections <CONNECTIONS>
          How many concurrent sockets to open [default: 1]
  -m, --messages <MESSAGES>
          How many messages send per 1 socket [default: 1]
  -e, --log-errors
          Collect and print errors in the end of the tests
  -r, --resp-sample-rate <RESP_SAMPLE_RATE>
          Show responses from the server with the specified sample rate [0-1] [default: 0.0]
  -h, --help
          Print help
  -V, --version
          Print version

```
                      
## Examples

### Simple run: 1 connection send 1 default message

```bash
./ws_stress --addr ws://0.0.0.0:8080
```
    Benchmarking: 1 messages, 1 connections
    Errors: {}
    Benchmark results:
    Ok: 1
    Err: 0
    Total: 1
    Elapsed time: 0.00 s
    Connections: 1
    Msg len: 12 B
    Requests per second: 11810.98 rps
    Throughput: 1.13 Mbit/sec

### Stress test: 1 connection sends 100000 specifies json messages
    
```bash
./ws_stress --addr ws://0.0.0.0:8080 '{"traceId":"2644795231141946144","correlationId":121212121212,"payload":{"type":"Foo","data":[{"time":"2023-01-01T10:28:31.236453182Z","amount":0.01},{"time":"2023-02-02T10:28:31.236453182Z","amount":0.0051}]}}' \
-m 100000 -c 1 -e
```
    Benchmarking: 100000 messages, 1 connections
    Errors: {}
    Benchmark results:
    Ok: 100000
    Err: 0
    Total: 100000
    Elapsed time: 0.55 s
    Connections: 1
    Msg len: 511 B
    Requests per second: 183013.38 rps
    Throughput: 748.16 Mbit/sec
       
