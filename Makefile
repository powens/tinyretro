.PHONY: server-lint
server-lint:
	@echo "Running server lint"
	cd server && cargo clippy

.PHONY: server-debug
server-debug:
	@echo "Running server debug"
	cd server && cargo run

.PHONY: client-debug
client-debug:	
	cd client && pnpm run dev

.PHONY: client-lint
client-lint:
	cd client && pnpm run lint