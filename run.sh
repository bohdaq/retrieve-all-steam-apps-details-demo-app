i=0
PID=0

while true
do
	echo "iteration $i"
	pkill -9 $PID &
	cargo run > out.log &
	PID=$!
	let i++
	echo "end of iteration $i"
	sleep 600
done
