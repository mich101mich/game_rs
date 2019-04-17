if "$1"=="server"
then cargo web start --release --bin web_game --auto-reload
else if "$1"=="watch"
	then if "$2"=="web"
		then cargo watch -x "web build --release --bin web_game"
		else cargo watch -x "build --bin game"
		fi
	fi
fi
