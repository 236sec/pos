.PHONY: test lint format migrate-add migrate-run create-db prepare

test:
	cargo test

lint:
	cargo clippy

format:
	cargo fmt

migrate-add:
	sqlx migrate add -r $(name)

migrate-run:
	sqlx migrate run

create-db:
	sqlx database create

prepare:
	cargo sqlx prepare
