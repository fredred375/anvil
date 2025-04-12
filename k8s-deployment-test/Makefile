# Makefile

TEST_FILES = $(wildcard manifests/*.yaml)

test:
	@for file in $(TEST_FILES); do \
		echo "Testing $$file:"; \
		kubectl apply -f $$file --dry-run=client; \
		echo ""; \
	done

.PHONY: test