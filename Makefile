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
	cargo tarpaulin -t 1200 --out Html

doc:
	cargo doc --no-deps --workspace --all-features

env:
	cargo fetch
	yarn install

frontend:
	@yarn install
	@-rm -rf ./static/cache/bundle/
	@-mkdir ./static/cache/bundle/css/
	@yarn run dart-sass -s compressed templates/main.scss  ./static/cache/bundle/css/main.css

migrate:
	cargo run --bin tests-migrate

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

release: frontend
	cargo build --release

run: frontend
	cargo run

test: frontend
	echo 'static/' && tree static || true
	echo 'tree/' && tree assets || true
	cargo test --all-features --no-fail-fast

xml-test-coverage: migrate
	cargo tarpaulin -t 1200 --out Xml
