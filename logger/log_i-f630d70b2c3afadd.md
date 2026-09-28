# Buffered NDI sender is starting its worker threads

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-f630d70b2c3afadd

## Log pattern

```text
start ndi sender threads, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
start ndi sender threads, ndi_name="HOST (Output)"
```

## Short description

Buffered NDI sender is starting its worker threads.

## Trigger condition

The NDI sender exists and its full output name has been obtained.

## Explanation

Video and audio worker objects are created and thread startup is attempted after this message. It does not confirm that both starts succeeded.

## Possible causes

- Normal buffered NDI output initialization.

## Related log messages

- [Buffered NDI worker startup reported failure](log_e-9788ebc25c005def.md)
- [Buffered NDI audio sender started](log_i-593de6c3e3aebdd4.md)
- [Buffered NDI video sender started](log_i-6c97d41a08157713.md)
