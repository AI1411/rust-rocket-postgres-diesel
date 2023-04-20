ROCKET_DATABASES='{rockets={url="postgres://postgres:postgres@localhost:5432/rockets"}}'

run:
	ROCKET_DATABASES=$(ROCKET_DATABASES) cargo run
watch:
	ROCKET_DATABASES=$(ROCKET_DATABASES) cargo watch -x run