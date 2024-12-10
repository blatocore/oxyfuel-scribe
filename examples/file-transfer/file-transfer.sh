#!/bin/bash

INFILE=$1
INFILE_SIZE=$(stat --printf '%s' $INFILE)

CHUNK_SIZE=256
IDX=1


emergency_button(){
	echo 'WAITFOR "control+escape"' | scribe
	kill -SIGKILL $1
}


emergency_button $$ &
e_button_pid=$!

echo "Press enter in the transfer window to begin"
echo "Press control+escape anywhere to stop"

echo 'WAITFOR "enter"' | scribe


while [ $IDX -lt $INFILE_SIZE ]; do
	CHUNK=$(cat $INFILE | tail -c "+$IDX" | head -c "$CHUNK_SIZE" | base64 | tr -d '\n');
	echo "index $IDX/$INFILE_SIZE"
	echo "TYPE 'echo $CHUNK | base64 -d >> transferred' <enter>"  | scribe

	IDX=$(($IDX + $CHUNK_SIZE))
done

kill -SIGKILL $e_button_pid

