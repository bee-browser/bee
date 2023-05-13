.PHONY: all
all:

.PHONY: test
test: codegen
	cargo nextest run --all-features

# Generate DFAs.
#
# We don't use build.rs for the code generation.  We generate DFAs in
# development time and commit the generated files to the repository.
#
# DFAs has no compilation flag at this time.  If we use build.rs for the code
# generation, always the same DFAs will be generated on every build as long as
# the lexical grammar and/or scripts to generate code are changed.  Therefore,
# using build.rs is inefficient in our use case.
.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen
