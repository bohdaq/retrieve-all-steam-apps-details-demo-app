i=0
PID=0

while true
do
	echo "\n\niteration $i"
	echo "" > out.log
	echo "killing previously started instance PID"
	killall retrieve-all-steam-apps-details-demo-app >> out.log &
	cargo run >> out.log &
	echo "started app with PID $!, output redirected to out.log"
	echo "instance will be restarted in 900s"
	echo "end of iteration $i"
	let i++
	sleep 900
done
