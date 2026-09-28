# Media region memory mapping failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-fdaaf0ff8acd6a26

## Log pattern

```text
open_and_map_read_only_file mmap() failed errno=%d fd=%d mmap_prot=%d st_size=%lld offset=%lld
```

## Parameters

- Argument 1 — Error: Operating-system error number.
- Argument 2 — Descriptor: Open file descriptor.
- Argument 3 — Protection: Numeric memory-protection bit mask.
- Argument 4 — Length: Page-aligned mapping length in bytes, despite the st_size label.
- Argument 5 — Offset: Page-aligned file offset in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file mmap() failed errno=12 fd=42 mmap_prot=1 st_size=8192 offset=4096
```

## Short description

Media region memory mapping failed.

## Trigger condition

The operating system rejects mapping the page-aligned region of a media file.

## Explanation

The mapping reader is not returned. The printed size includes alignment bytes before the requested offset.

## Possible causes

- The operating system rejects mapping the page-aligned region of a media file.

## What to check

1. Check the mapping error, file size, alignment, and available address space.
