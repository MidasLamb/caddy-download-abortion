#!/bin/bash

/app/target/debug/temp-download-test &

/usr/bin/caddy run --config /app/Caddyfile