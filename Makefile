dev-migrate:
	sea-orm-cli migrate up

dev-migrate-fresh:
	sea-orm-cli migrate fresh

gen-table:
	sea-orm-cli migrate generate create_$(name)

gen-entity:
	sea-orm-cli generate entity -o ./rust-lib/src/database/entity
