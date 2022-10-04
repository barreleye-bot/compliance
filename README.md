# Barreleye Compliance

Regulatory compliance tool for businesses dealing with digital assets.

This is a work-in-progress and not ready for production.

## Setup

First, fetch the data:

```bash
cargo run scan
```

Then, start the server:

```bash
cargo run server
```

## Run

To check for a sanctioned digital currency address:

```
http://localhost:3000/v0/entities?address=0x0
```

```json
{
  "address": "0x0",
  "sanctioned": false
}
```

