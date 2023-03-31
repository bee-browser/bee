.PHONY: all
all:

.PHONY: update-deps
update-deps:
	@find .. -name '.js' | xargs deno run --allow-net --allow-read --allow-write https://deno.land/x/udd@0.8.2/main.ts
