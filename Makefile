default: frontend
	cargo build

clean:
	@cargo clean
	@yarn cache clean
	@-rm -rf browser/pkg
	@-rm ./src/cache_buster_data.json
	@-rm -rf ./static/cache/bundle
	@-rm -rf ./assets

coverage: migrate
	cargo tarpaulin -t 1200 --out Html --skip-clean  --all-features --no-fail-fast --workspace=database/db-sqlx-postgres,database/db-sqlx-sqlite,.

doc:
	cargo doc --no-deps --workspace=database/db-core,database/db-sqlx-postgres,database/db-sqlx-sqlite. --all-features

env:
	cargo fetch
	yarn install
	@-virtualenv venv
	. ./venv/bin/activate && pip install codespell

frontend:
	@yarn install
	@-rm -rf ./static/cache/bundle/
	@-mkdir ./static/cache/bundle/css/
	@yarn run dart-sass -s compressed templates/main.scss  ./static/cache/bundle/css/main.css

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

lint: ## Lint codebase
	. ./venv/bin/activate && ./scripts/spellcheck.sh --write
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

migrate:
	@-rm -rf database/db-sqlx-sqlite/tmp && mkdir database/db-sqlx-sqlite/tmp
	cd database/migrator && cargo run

test: migrate #frontend
	cd database/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo test --no-fail-fast
	cd database/db-sqlx-sqlite &&\
		DATABASE_URL=${SQLITE_DATABASE_URL}\
		cargo test --no-fail-fast
	cargo test --all-features --no-fail-fast \
		--workspace=database/db-sqlx-postgres,database/db-sqlx-sqlite,.

xml-test-coverage: migrate
	cargo tarpaulin -t 1200 --out Xml --skip-clean --all-features --no-fail-fast --workspace=database/db-sqlx-postgres,database/db-sqlx-sqlite,.
