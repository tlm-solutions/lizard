# Lizard

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

The Service that serves the state.

## Purpose

- Serves the current state of the transport network to its clients
- Reads the state of the network from the redis.

## Service Configuration

Following Environment Variables are read by there service

- **--host** host of http server
- **--port**  port of http server
- **RUST_LOG** log level
- **RUST_BACKTRACE** stack traces
- **REDIS_HOST** host of the redis server
- **REDIS_PORT** port of the redis server
