contest ?= 

.PHONY: abc
abc:
	@if [ -z "$(contest)" ]; then \
		echo "Usage: make abc contest=XXX"; \
		exit 1; \
	fi; \
	for p in a b c d e; do \
		mkdir -p abc/$(contest)/$$p && \
		cp -n template.go abc/$(contest)/$$p/main.go && \
		echo "Created: abc/$(contest)/$$p/main.go"; \
	done
