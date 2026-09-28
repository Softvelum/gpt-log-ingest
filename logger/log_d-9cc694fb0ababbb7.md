# HTTP request method unsupported

Product: Nimble Streamer
Component: HTTP/2 and HTTP/3 request handling
Logging function: log_d
Log level: debug
Documentation ID: log_d-9cc694fb0ababbb7

## Log pattern

```text
not a GET/POST/DELETE/HEAD/OPTIONS/CONNECT request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
not a GET/POST/DELETE/HEAD/OPTIONS/CONNECT request
```

## Short description

HTTP request method unsupported.

## Trigger condition

The parsed HTTP/2 or HTTP/3 method is not one of GET, POST, DELETE, HEAD, OPTIONS, or CONNECT.

## Explanation

The request stream is refused through a stream reset; normal request routing does not run.

## Possible causes

- The client uses an unsupported or unrecognized method.

## What to check

1. Check the client request method.

## Recommended action

Use a supported method for the requested operation.
