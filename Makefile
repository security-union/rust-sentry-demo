install-latest-sentry:
	./install-sentry-latest.sh

run-sentry:
	docker-compose -f ./sentry-client/docker-compose.yml up -d

down-sentry:
	docker-compose -f ./sentry-client/docker-compose.yml down