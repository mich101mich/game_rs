if "%1"=="server" (
	cargo web start --release --bin web_game --auto-reload
) else (
	if "%1"=="watch" (
		if "%2"=="web" (
			cargo watch -x "web build --release --bin web_game"
		) else (
			cargo watch -x "build --bin game"
		)
	)
)