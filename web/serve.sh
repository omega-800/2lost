#!/usr/bin/env bash

while true; do
  response=$(cat ./index.html)
  echo -e "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: ${#response}\r\n\r\n$response" | nc -l 8080
done
