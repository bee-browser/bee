.PHONY: all
all:

.PHONY: update-deps
update-deps:
	@deno run --allow-net --allow-read --allow-write https://deno.land/x/udd@0.7.3/main.ts import-map.json resources/workers/*.js
