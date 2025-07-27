.PHONY: prod
prod: server-docker client-docker

.PHONY: server-lint
server-lint:
	@echo "Running server lint"
	cd server && cargo clippy

.PHONY: server-test
server-test:
	@echo "Running server test"
	cd server && cargo test

.PHONY: server-debug
server-debug:
	@echo "Running server debug"
	cd server && cargo run

.PHONY: server-build
server-build:
	@echo "Building server"
	cd server && cargo build

.PHONY: server-build-release
server-build-release:
	@echo "Building server release"
	cd server && cargo build --release

.PHONY: server-docker
server-docker:
	cd server && docker build -t tinyretro-server .

.PHONY: client-debug
client-debug:	
	cd client && pnpm run dev

.PHONY: client-lint
client-lint:
	cd client && pnpm run lint

.PHONY: client-test
client-test:
	cd client && pnpm run test

.PHONY: client-build
client-build:
	cd client && pnpm run build

.PHONY: client-docker
client-docker:
	cd client && docker build -t tinyretro-client .

.PHONY: client-install
client-install:
	cd client && pnpm install