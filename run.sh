i=0
PID=0

while true
do
	echo "\n\niteration $i"
	rm out.log
	touch out.log
	echo "killing previously started instance PID"
	killall retrieve-all-steam-apps-details-demo-app
	cargo run >> out.log &
	echo "started app with PID $!, output redirected to out.log"
	echo "instance will be restarted in 600s"
	echo "end of iteration $i"
	let i++
	sleep 600
done
