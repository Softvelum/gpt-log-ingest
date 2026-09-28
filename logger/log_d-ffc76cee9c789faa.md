# MP4 local-file request entered

Product: Nimble Streamer
Component: Non-Windows progressive download modules
Logging function: log_d
Log level: debug
Documentation ID: log_d-ffc76cee9c789faa

## Log pattern

```text
NginxMp4Module::HandleRequest conf_ = %p, hanlder_ = %p, local_file_ = %s
```

## Parameters

- Argument 1 — configuration: Module configuration object address.
- Argument 2 — handler: Request handler address.
- Argument 3 — file: Local media file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
NginxMp4Module::HandleRequest conf_ = 0x1000, hanlder_ = 0x2000, local_file_ = /media/movie.mp4
```

## Short description

MP4 local-file request entered.

## Trigger condition

A local-file request enters the MP4 module wrapper.

## Explanation

The module configuration, handler address, and selected file are logged before request arguments are prepared and the external handler is called.
