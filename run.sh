i=0
PID=0

while true
do
	echo "\n\niteration $i"
	sh stop-instance.sh
	echo "killing previously started instance PID"
	cargo run > out.log &
	echo "started app with PID $!, output redirected to out.log"
	PID=$!
	echo "pkill -9 $PID">stop-instance.sh
	echo "instance will be restarted in 600s"
	echo "end of iteration $i"
	let i++
	sleep 600
done
