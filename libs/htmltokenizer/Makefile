HTML5LIBTESTS_NAMES := \
  contentModelFlags \
  domjs \
  entities \
  escapeFlag \
  namedEntities \
  numericEntities \
  pendingSpecChanges \
  test1 \
  test2 \
  test3 \
  test4 \
  unicodeCharsProblematic \
  unicodeChars  # xmlViolation is excluded

HTML5LIBTESTS_JSON_FILES := $(addprefix src/html5libtests/,$(addsuffix .codegen.json,$(HTML5LIBTESTS_NAMES)))

.PHONY: codegen
codegen: src/charref/trie.rs

.PHONY: testgen
testgen: $(HTML5LIBTESTS_JSON_FILES)

src/charref/trie.rs: src/charref/trie.codegen.json src/charref/trie.rs.hbs
	@echo Generating $@...
	@bee-tools-codegen --no-escape src/charref/trie.rs.hbs $< >$@

# TODO: Use JSON5 and add comments at the beginning of the file like other
#       generated files.
.PRECIOUS: src/charref/trie.codegen.json
src/charref/trie.codegen.json: src/charref/entities.json src/charref/trie.codegen.json.js
	@echo Generating $@...
	@deno run src/charref/trie.codegen.json.js <$< >$@

# TODO: Use JSON5 and add comments at the beginning of the file like other
#       generated files.
src/html5libtests/%.codegen.json: src/html5libtests/%.test src/html5libtests/codegen.json.js
	@echo Generating $@...
	@deno run src/html5libtests/codegen.json.js "$(subst .test,,$(notdir $<))" <$< >$@
