BEE_TOOLS := ../../../tools/bin/bee-tools
ES2022_LEX_YAML := ../data/es2022.lex.yaml
LEXGEN_JS := ../scripts/lexgen.js

# Specify `-d` if you want to see debug logs.
LEXGEN_ARGS ?=

TOKENS := \
  WhiteSpace \
  LineTerminatorSequence \
  Comment \
  OPTIONAL_CHAINING \
  LBRACE \
  LPAREN \
  RPAREN \
  LBRACK \
  RBRACK \
  DOT \
  ELLIPSIS \
  SEMI_COLON \
  COMMA \
  LT \
  GT \
  LTE \
  GTE \
  EQ \
  NE \
  EQ_STRICT \
  NE_STRICT \
  ADD \
  SUB \
  MUL \
  MOD \
  EXP \
  INC \
  DEC \
  SHL \
  SAR \
  SHR \
  BIT_AND \
  BIT_OR \
  BIT_XOR \
  NOT \
  BIT_NOT \
  AND \
  OR \
  NULLISH \
  CONDITIONAL \
  COLON \
  ASSIGN \
  ADD_ASSIGN \
  SUB_ASSIGN \
  MUL_ASSIGN \
  MOD_ASSIGN \
  EXP_ASSIGN \
  SHL_ASSIGN \
  SAR_ASSIGN \
  SHR_ASSIGN \
  BIT_AND_ASSIGN \
  BIT_OR_ASSIGN \
  BIT_XOR_ASSIGN \
  AND_ASSIGN \
  OR_ASSIGN \
  NULLISH_ASSIGN \
  ARROW \
  DIV \
  DIV_ASSIGN \
  RBRACE \
  AWAIT \
  BREAK \
  CASE \
  CATCH \
  CLASS \
  CONST \
  CONTINUE \
  DEBUGGER \
  DEFAULT \
  DELETE \
  DO \
  ELSE \
  ENUM \
  EXPORT \
  EXTENDS \
  FALSE \
  FINALLY \
  FOR \
  FUNCTION \
  IF \
  IMPORT \
  IN \
  INSTANCEOF \
  NEW \
  NULL \
  RETURN \
  SUPER \
  SWITCH \
  THIS \
  THROW \
  TRUE \
  TRY \
  TYPEOF \
  VAR \
  VOID \
  WHILE \
  WITH \
  YIELD \
  ASYNC \
  FROM \
  GET \
  META \
  OF \
  SET \
  TARGET \
  LET \
  STATIC \
  IMPLEMENTS \
  INTERFACE \
  PACKEGE \
  PRIVATE \
  PROTECTED \
  PUBLIC \
  IdentifierName \
  PrivateIdentifier \
  NumericLiteral \
  StringLiteral \
  Template

.PHONY: codegen
codegen: tables.rs

tables.rs: tables.rs.hbs lexer.dfa.json
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen tables.rs.hbs lexer.dfa.json | rustfmt --emit=stdout >$@

.PRECIOUS: lexer.dfa.json
lexer.dfa.json: $(ES2022_LEX_YAML) $(LEXGEN_JS)
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run $(LEXGEN_JS) $(LEXGEN_ARGS) $(TOKENS) | jq '.' >$@
