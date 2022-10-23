i=0
PID=0
RESTART_AT_IN_SECONDS=900

while true
do
	echo "\n\niteration $i"
	echo "" > out.log
	echo "killing previously started instance PID"
	killall retrieve-all-steam-apps-details-demo-app >> out.log &
	touch stop.sh
	sh stop.sh
	cargo run >> out.log &
	echo "kill -9 $!" > stop.sh
	echo "started app with PID $!, output redirected to out.log"
	echo "instance will be restarted in $RESTART_AT_IN_SECONDS seconds"
	echo "end of iteration $i"
	let i++
	sleep $RESTART_AT_IN_SECONDS
done
