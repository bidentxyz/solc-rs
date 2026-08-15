.PHONY: lint
lint: # Run linter
	@echo "Run formatter check"
	@cargo fmt --check
	@uvx --from panache-cli==2.61.0 panache format --check .
	@echo "Run clippy"
	@cargo clippy -- -D warnings
	@echo "Run checkrs"
	@uvx --from git+https://github.com/pyk/checkrs checkrs run src/

.PHONY: fmt
fmt: # Run formatter
	@echo "Run rust formatter"
	@cargo fmt
	@echo "Run markdown formatter"
	@uvx --from panache-cli==2.61.0 panache format .

.PHONY: test
test: # Run tests
	@echo "Run tests"
	@cargo test --quiet

.PHONY: doc
doc: # Build docs and serve them
	@echo "Run doc build"
	@cargo doc --no-deps
	@IP=$$(python3 -c 'import socket; s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.connect(("8.8.8.8", 80)); print(s.getsockname()[0])'); \
	echo "Serving docs on http://$$IP:8000/solc/"; \
	cd target/doc && python3 -m http.server 8000 --bind 0.0.0.0

.PHONY: fixtures
fixtures: # Compile all fixture inputs listed in fixtures/solc*/*.json
	@echo "Compiling fixtures"
	@./scripts/compile.py

.PHONY: clean
clean: # Remove all fixture build outputs
	@echo "Cleaning fixture outputs"
	@rm -rf fixtures/solc*/out
