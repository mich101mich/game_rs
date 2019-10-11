if [ "$1"=="server" ]
then cargo web start --release --auto-reload --port 8080 --host 0.0.0.0
else if [ "$1"=="watch" ]
	then if [ "$2"=="web" ]
		then cargo watch -x "web check"
		else cargo watch -x "check"
		fi
	fi
fi
