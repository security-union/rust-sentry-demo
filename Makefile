install-latest-sentry:
	./install-sentry-latest.sh

run-sentry:
	cd sentry-client && docker-compose up -d

down-sentry:
	cd sentry-client && docker-compose down
