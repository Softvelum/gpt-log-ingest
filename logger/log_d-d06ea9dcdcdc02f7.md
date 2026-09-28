# File block TLS write result

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_d
Log level: debug
Documentation ID: log_d-d06ea9dcdcdc02f7

## Log pattern

```text
SSL_write %lldB to %s s=%d
```

## Parameters

- Argument 1 — Write result: Signed byte count returned by TLS writing, or a negative error result.
- Argument 2 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 3 — Socket: Integer socket descriptor.

The signed platform-size result is formatted as long long; exact type compatibility depends on the build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SSL_write 4096B to client s=12
```

## Short description

File block TLS write result.

## Trigger condition

A file block has been read and submitted to a TLS write.

## Explanation

The signed result is reported, including a negative failure value. A negative result stores the TLS error; otherwise the file offset advances by the result.
