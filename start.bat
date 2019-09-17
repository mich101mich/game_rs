if "%1"=="server" (
	cargo web start --release --auto-reload --port 8080
) else (
	if "%1"=="watch" (
		if "%2"=="web" (
			cargo watch -x "web check"
		) else (
			cargo watch -x "check"
		)
	)
)