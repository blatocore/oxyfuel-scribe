#!/bin/bash


# This script acts as a makeshift clipboard, for environments where clipboard 
# support is not enabled/not working (i.e poorly configured VM, IDRAC, ...)
#
# You can copy your selection like usual, using Ctrl+(Shift)+C
#
# You can paste using Ctrl+Alt+Shift+V and releasing said keys,
# to prevent triggering other keyboard shortcuts(can be changed in the script)
# The script will begin pasting clipboard contents 1000ms after pressing the shortcut
#
# Requirements: scribe, xclip


while [ "1" -eq "1" ]; do 
	echo "WAITFOR \"control+alt+shift+v\"" | scribe
	clipboard_content=$(xclip -selection clipboard -out)
	echo "TYPE \"$clipboard_content\""
	echo "SLEEP 1000; TYPE \"$clipboard_content\"" | scribe
done
