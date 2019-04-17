if "$1"=="server"
then cargo web start --release --auto-reload
else if "$1"=="watch"
	then if "$2"=="web"
		then cargo watch -x "web build --release"
		else cargo watch -x "build"
		fi
	fi
fi
