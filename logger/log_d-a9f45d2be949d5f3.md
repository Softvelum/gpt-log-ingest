# DVR segment stored and indexed

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-a9f45d2be949d5f3

## Log pattern

```text
added dvr seg str='%s/%s' f='%s' offset=%llu size=%u t=%d p=%u st=%llu dur=%llu dbof=%u no=%u
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Storage file: Text naming the recorded media storage file.
- Argument 4 — File offset: Unsigned 64-bit byte offset within the media storage file.
- Argument 5 — Segment bytes: Unsigned segment size in bytes.
- Argument 6 — Segment type: Integer category: 0 video initialization, 1 audio initialization, 2 video, 3 audio, 4 shared data file, 5 JPEG, 6 DVB subtitles, 7 KLV metadata, 8 WebVTT.
- Argument 7 — Period: Unsigned recording period identifier.
- Argument 8 — Media start: Unsigned 64-bit media start timestamp in the segment track timescale.
- Argument 9 — Media duration: Unsigned 64-bit duration in the segment track timescale.
- Argument 10 — Database offset: Unsigned byte offset of the segment record in its database.
- Argument 11 — Sequence number: Unsigned playback sequence number assigned to the segment; initialization entries may be zero.

## Log examples

Illustrative examples; these are not captured server logs.

```text
added dvr seg str='live/camera' f='/data/dvr/live/camera/1700000000000.dat' offset=1024 size=65536 t=2 p=1 st=90000 dur=540000 dbof=512 no=1
```

## Short description

DVR segment stored and indexed.

## Trigger condition

Segment bytes and the database record are written and in-memory period metadata is updated.

## Explanation

Retention cleanup runs next and the archive update time advances.

## Possible causes

- Segment bytes and the database record are written and in-memory period metadata is updated.
