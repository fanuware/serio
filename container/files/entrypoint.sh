#!/bin/bash

setsid socat -d -d \
  pty,raw,echo=0,mode=660,group=dialout,link=/dev/ttyGetty \
  pty,raw,echo=0,mode=660,group=dialout,link=/dev/ttyUSB0 \
  > /dev/null 2>&1 &

setsid sh -c '
  while true; do
    getty -L -a root ttyGetty 115200 vt220
  done' > /dev/null 2>&1 &

echo 'bind '\''"\e[A": history-search-backward'\' >> ~/.bashrc
echo 'bind '\''"\e[B": history-search-forward'\' >> ~/.bashrc

exec "$@"

