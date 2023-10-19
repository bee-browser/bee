// Initially, `Builder` implemented using `serde_json::Value` in order to avoid defining ESTree
// node types.  This caused serious performance issues.  In fact, the initial implementation was
// slower than `acorn` which is written in JavaScript and performed by `deno`.
//
// The first performance issue was caused in `serde_json::json!()` macro.  Specifying a
//`serde_json::Value` in `json!()` works properly but causes a performance problem.  `json!()`
// calls `serde_json::value::to_value()` for the specified `serde_json::Value` even though no type
// conversion is needed.  As a result, a cloned value is created from the specified
// `serde_json::Value`.  Probably, this is intentional.  Imagine the following situation:
//
// ```rust
// let param = SerializableType { ... };
// let value = json!({
//   "param": param,
// });
// ```
//
// The `param` is not moved and `serde_json::value::to_value()` is called to build a
// `serde_json::Value` from it.  This means that we have to pass all non-copyable variables to
// `json!()` by references if we want to avoid undesired conversions described above.  However, we
// don't think this is feasible change.  Because many projects use `json!()` and implement
// functions based on the current behavior.  The change will break those projects.
//
// Therefore, we choose a dirty solution.  For avoiding undesired conversions, we set
// `serde_json::Value`s after a base value is created by using `json!()` like below:
//
// ```rust
// let mut node = json!({
//   "type": "Program",
//    "start": start.offset,
//    "end": end.offset,
//    "body": null,  // it's a placeholder and will be replaced with the actual value soon.
//    "sourceType": source_type,
// });
// node["body"] = body;
// ```
//
// This solution improved the performance but the implementation was still slower than `acorn`.
// Eventually, `serde_json::Value` has been completely removed and `Builder` has been rewritten
// using hand-crafted types defined in the `nodes` module.

mod actions;

use bee_jsparser::Location;
use bee_jsparser::ProductionRule;
use bee_jsparser::SyntaxHandler;
use bee_jsparser::Token;

use crate::nodes::node;
use crate::nodes::Node;
use crate::nodes::NodeRef;
use actions::ACTIONS;

#[derive(Debug)]
enum StackValue {
    Token(String),
    Node(NodeRef),
    Nullable(Option<NodeRef>),
    List(Vec<NodeRef>),
    Array(Vec<Option<NodeRef>>),
    Either(NodeRef, NodeRef),
}

pub struct Builder {
    stack: Vec<(StackValue, Location, Location)>,
    location: Location,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            stack: Vec::with_capacity(512),
            location: Default::default(),
        }
    }

    fn empty_script(&mut self) {
        assert!(self.stack.is_empty());
        let start = Location::default();
        let end = self.location.clone();
        let node = node!(script@start..end);
        self.push_node(node, start, end);
    }

    fn script(&mut self) {
        let (body, ..) = self.pop_list();
        let start = Location::default();
        let end = self.location.clone();
        let node = node!(script@start..end; body);
        self.push_node(node, start, end);
    }

    fn empty_module(&mut self) {
        assert!(self.stack.is_empty());
        let start = Location::default();
        let end = self.location.clone();
        let node = node!(module@start..end);
        self.push_node(node, start, end);
    }

    fn module(&mut self) {
        let (body, ..) = self.pop_list();
        let start = Location::default();
        let end = self.location.clone();
        let node = node!(module@start..end; body);
        self.push_node(node, start, end);
    }

    fn import_from(&mut self) {
        let (_, end) = self.check(";");
        let (source, ..) = self.pop_node();
        let (specifiers, ..) = self.pop_list();
        let (start, ..) = self.check("import");
        let node = node!(import_declaration@start..end; specifiers, source);
        self.push_node(node, start, end);
    }

    fn side_effect_import(&mut self) {
        let (_, end) = self.check(";");
        let (source, ..) = self.pop_node();
        let (start, _) = self.check("import");
        let node = node!(import_declaration@start..end; source);
        self.push_node(node, start, end);
    }

    fn import_default_specifier(&mut self) {
        let (local, start, end) = self.pop_node();
        let node = node!(import_default_specifier@start..end; local);
        self.push_node(node, start, end);
    }

    fn import_namespace_specifier(&mut self) {
        let (local, _, end) = self.pop_node();
        self.check("as");
        let (start, _) = self.check("*");
        let node = node!(import_namespace_specifier@start..end; local);
        self.push_node(node, start, end);
    }

    fn import_specifier(&mut self) {
        let (imported, start, end) = self.pop_node();
        let node = node!(import_specifier@start..end; imported);
        self.push_node(node, start, end);
    }

    fn import_specifier_as(&mut self) {
        let (local, _, end) = self.pop_node();
        self.check("as");
        let (imported, start, _) = self.pop_node();
        let node = node!(import_specifier@start..end; imported => local);
        self.push_node(node, start, end);
    }

    fn export_from(&mut self) {
        let (_, end) = self.check(";");
        let (source, ..) = self.pop_node();
        let (clause, ..) = self.stack.pop().unwrap();
        let (start, _) = self.check("export");
        let node = match clause {
            StackValue::Token(_) => node!(export_all_declaration@start..end; source),
            StackValue::Node(declaration) => {
                node!(export_all_declaration@start..end; declaration, source)
            }
            StackValue::List(specifiers) => {
                node!(export_named_declaration@start..end; specifiers, source)
            }
            _ => panic!(),
        };
        self.push_node(node, start, end);
    }

    fn export_list(&mut self) {
        let (_, end) = self.check(";");
        let (specifiers, ..) = self.pop_list();
        let (start, _) = self.check("export");
        let node = node!(export_named_declaration@start..end; specifiers => specifiers);
        self.push_node(node, start, end);
    }

    fn export_vars(&mut self) {
        let (declaration, _, end) = self.pop_node();
        let (start, _) = self.check("export");
        let node = node!(export_named_declaration@start..end; declaration => declaration);
        self.push_node(node, start, end);
    }

    fn export_decl(&mut self) {
        let (declaration, _, end) = self.pop_node();
        let (start, _) = self.check("export");
        let node = node!(export_named_declaration@start..end; declaration => declaration);
        self.push_node(node, start, end);
    }

    fn default_export_decl(&mut self) {
        let (declaration, _, end) = self.pop_node();
        self.check("default");
        let (start, _) = self.check("export");
        let node = node!(export_default_declaration@start..end; declaration);
        self.push_node(node, start, end);
    }

    fn default_export_class(&mut self) {
        let (declaration, _, end) = self.pop_node();
        self.check("default");
        let (start, _) = self.check("export");
        let node = node!(export_default_declaration@start..end; declaration);
        self.push_node(node, start, end);
    }

    fn default_export_expr(&mut self) {
        let (_, end) = self.check(";");
        let (expression, ..) = self.pop_node();
        self.check("default");
        let (start, _) = self.check("export");
        let node = node!(export_default_declaration@start..end; expression);
        self.push_node(node, start, end);
    }

    fn export_specifier(&mut self) {
        let (local, start, end) = self.pop_node();
        let node = node!(export_specifier@start..end; local);
        self.push_node(node, start, end);
    }

    fn export_specifier_as(&mut self) {
        let (exported, _, end) = self.pop_node();
        self.check("as");
        let (local, start, _) = self.pop_node();
        let node = node!(export_specifier@start..end; local => exported);
        self.push_node(node, start, end);
    }

    fn variable_declaration(&mut self) {
        let (_, end) = self.check(";");
        let (declarations, ..) = self.pop_list();
        let (kind, start, _) = self.pop_token();
        let node = node!(variable_declaration@start..end; kind, declarations);
        self.push_node(node, start, end);
    }

    fn empty_statement(&mut self) {
        let (start, end) = self.check(";");
        let node = node!(empty_statement@start..end);
        self.push_node(node, start, end);
    }

    fn expression_statement(&mut self) {
        let (_, end) = self.check(";");
        let (expression, start, _) = self.pop_node();
        let node = node!(expression_statement@start..end; expression);
        self.push_node(node, start, end);
    }

    fn if_else_statement(&mut self) {
        let (alternate, _, end) = self.pop_node();
        self.check("else");
        let (consequent, ..) = self.pop_node();
        self.check(")");
        let (test, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("if");
        let node = node!(if_statement@start..end; test, consequent, alternate);
        self.push_node(node, start, end);
    }

    fn if_statement(&mut self) {
        let (consequent, _, end) = self.pop_node();
        self.check(")");
        let (test, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("if");
        let node = node!(if_statement@start..end; test, consequent);
        self.push_node(node, start, end);
    }

    fn continue_statement(&mut self) {
        let (_, end) = self.check(";");
        let (start, _) = self.check("continue");
        let node = node!(continue_statement@start..end);
        self.push_node(node, start, end);
    }

    fn labeled_continue_statement(&mut self) {
        let (_, end) = self.check(";");
        let (label, ..) = self.pop_node();
        let (start, _) = self.check("continue");
        let node = node!(continue_statement@start..end; label);
        self.push_node(node, start, end);
    }

    fn break_statement(&mut self) {
        let (_, end) = self.check(";");
        let (start, _) = self.check("break");
        let node = node!(break_statement@start..end);
        self.push_node(node, start, end);
    }

    fn labeled_break_statement(&mut self) {
        let (_, end) = self.check(";");
        let (label, ..) = self.pop_node();
        let (start, _) = self.check("break");
        let node = node!(break_statement@start..end; label);
        self.push_node(node, start, end);
    }

    fn with_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (object, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("with");
        let node = node!(with_statement@start..end; object, body);
        self.push_node(node, start, end);
    }

    fn labeled_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(":");
        let (label, start, _) = self.pop_node();
        let node = node!(labeled_statement@start..end; label, body);
        self.push_node(node, start, end);
    }

    fn throw_statement(&mut self) {
        let (_, end) = self.check(";");
        let (argument, ..) = self.pop_node();
        let (start, _) = self.check("throw");
        let node = node!(throw_statement@start..end; argument);
        self.push_node(node, start, end);
    }

    fn try_catch_statement(&mut self) {
        let (handler, _, end) = self.pop_node();
        let (block, ..) = self.pop_node();
        let (start, _) = self.check("try");
        let node = node!(try_statement@start..end; block, catch => handler);
        self.push_node(node, start, end);
    }

    fn try_finally_statement(&mut self) {
        let (finalizer, _, end) = self.pop_node();
        let (block, ..) = self.pop_node();
        let (start, _) = self.check("try");
        let node = node!(try_statement@start..end; block, finally => finalizer);
        self.push_node(node, start, end);
    }

    fn try_catch_finally_statement(&mut self) {
        let (finalizer, _, end) = self.pop_node();
        let (handler, ..) = self.pop_node();
        let (block, ..) = self.pop_node();
        let (start, _) = self.check("try");
        let node = node!(try_statement@start..end; block, catch => handler, finally => finalizer);
        self.push_node(node, start, end);
    }

    fn catch_clause(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (param, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("catch");
        let node = node!(catch_clause@start..end; param, body);
        self.push_node(node, start, end);
    }

    fn catch_clause_no_param(&mut self) {
        let (body, _, end) = self.pop_node();
        let (start, _) = self.check("catch");
        let node = node!(catch_clause@start..end; body);
        self.push_node(node, start, end);
    }

    fn finally_clause(&mut self) {
        let (node, _, end) = self.pop_node();
        let (start, _) = self.check("finally");
        self.push_node(node, start, end);
    }

    fn debugger_statement(&mut self) {
        let (_, end) = self.check(";");
        let (start, _) = self.check("debugger");
        let node = node!(debugger_statement@start..end);
        self.push_node(node, start, end);
    }

    fn switch_statement(&mut self) {
        let (cases, _, end) = self.pop_list();
        self.check(")");
        let (discriminant, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("switch");
        let node = node!(switch_statement@start..end; discriminant, cases);
        self.push_node(node, start, end);
    }

    fn case_block_default(&mut self) {
        let (_, end) = self.check("}");
        let (default, ..) = self.pop_node();
        let (start, _) = self.check("{");
        self.push_list(vec![default], start, end);
    }

    fn case_block_cases_default(&mut self) {
        let (_, end) = self.check("}");
        let (default, ..) = self.pop_node();
        let (mut cases, ..) = self.pop_list();
        let (start, _) = self.check("{");
        cases.push(default);
        self.push_list(cases, start, end);
    }

    fn case_block_default_cases(&mut self) {
        let (_, end) = self.check("}");
        let (mut cases, ..) = self.pop_list();
        let (default, ..) = self.pop_node();
        let (start, _) = self.check("{");
        cases.insert(0, default);
        self.push_list(cases, start, end);
    }

    fn case_block_cases_default_cases(&mut self) {
        let (_, end) = self.check("}");
        let (mut remaining, ..) = self.pop_list();
        let (default, ..) = self.pop_node();
        let (mut cases, ..) = self.pop_list();
        let (start, _) = self.check("{");
        cases.push(default);
        cases.append(&mut remaining);
        self.push_list(cases, start, end);
    }

    fn switch_case_no_consequent(&mut self) {
        let (_, end) = self.check(":");
        let (test, ..) = self.pop_node();
        let (start, _) = self.check("case");
        let node = node!(switch_case@start..end; test);
        self.push_node(node, start, end);
    }

    fn switch_case(&mut self) {
        let (consequent, _, end) = self.pop_list();
        self.check(":");
        let (test, ..) = self.pop_node();
        let (start, _) = self.check("case");
        let node = node!(switch_case@start..end; test, consequent);
        self.push_node(node, start, end);
    }

    fn switch_case_default_no_consequent(&mut self) {
        let (_, end) = self.check(":");
        let (start, _) = self.check("default");
        let node = node!(switch_default@start..end);
        self.push_node(node, start, end);
    }

    fn switch_case_default(&mut self) {
        let (consequent, _, end) = self.pop_list();
        self.check(":");
        let (start, _) = self.check("default");
        let node = node!(switch_default@start..end; consequent);
        self.push_node(node, start, end);
    }

    fn do_while_statement(&mut self) {
        let (_, end) = self.check(";");
        self.check(")");
        let (test, ..) = self.pop_node();
        self.check("(");
        self.check("while");
        let (body, ..) = self.pop_node();
        let (start, _) = self.check("do");
        let node = node!(do_while_statement@start..end; test, body);
        self.push_node(node, start, end);
    }

    fn while_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (test, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("while");
        let node = node!(while_statement@start..end; test, body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_init_test_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        self.check(";");
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_test_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; init; ; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_init_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; ; test; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; init; test; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_init_test(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        self.check(";");
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; ; ; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_test(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; init; ; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_no_init(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; ; test; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let node = node!(for_statement@start..end; init; test; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_vars_no_test_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        self.check(";");
        let (declarations, _, var_end) = self.pop_list();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(variable_declaration@var_start..var_end; var, declarations);
        let node = node!(for_statement@start..end; init; ; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_vars_no_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        let (declarations, _, var_end) = self.pop_list();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(variable_declaration@var_start..var_end; var, declarations);
        let node = node!(for_statement@start..end; init; test; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_vars_no_test(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        self.check(";");
        let (declarations, _, var_end) = self.pop_list();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(variable_declaration@var_start..var_end; var, declarations);
        let node = node!(for_statement@start..end; init; ; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_vars(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        let (test, ..) = self.pop_node();
        self.check(";");
        let (declarations, _, var_end) = self.pop_list();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(variable_declaration@var_start..var_end; var, declarations);
        let node = node!(for_statement@start..end; init; test; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_decl_no_test_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(for_init_update; init);
        let node = node!(for_statement@start..end; init; ; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_decl_no_update(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        self.check(";");
        let (test, ..) = self.pop_node();
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(for_init_update; init);
        let node = node!(for_statement@start..end; init; test; ; body);
        self.push_node(node, start, end);
    }

    fn for_statement_decl_no_test(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(for_init_update; init);
        let node = node!(for_statement@start..end; init; ; update; body);
        self.push_node(node, start, end);
    }

    fn for_statement_decl(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (update, ..) = self.pop_node();
        self.check(";");
        let (test, ..) = self.pop_node();
        let (init, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        let init = node!(for_init_update; init);
        let node = node!(for_statement@start..end; init; test; update; body);
        self.push_node(node, start, end);
    }

    fn for_in_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("in");
        let (left, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        // If LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral, it must cover
        // an AssignmentPattern.  See "14.7.5.1 Static Semantics: Early Errors" in ECMA-262.
        let left = node!(into_pattern; left);
        let node = node!(for_in_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_in_statement_vars(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("in");
        let (binding, _, var_end) = self.pop_node();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let left = node!(variable_declaration@var_start..var_end; var, vec![binding]);
        let node = node!(for_in_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_of_statement(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("of");
        let (left, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("for");
        // If LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral, it must cover
        // an AssignmentPattern.  See "14.7.5.1 Static Semantics: Early Errors" in ECMA-262.
        let left = node!(into_pattern; left);
        let node = node!(for_of_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_of_statement_vars(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("of");
        let (binding, _, var_end) = self.pop_node();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        let (start, _) = self.check("for");
        let left = node!(variable_declaration@var_start..var_end; var, vec![binding]);
        let node = node!(for_of_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_of_statement_await(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("of");
        let (left, ..) = self.pop_node();
        self.check("(");
        self.check("await");
        let (start, _) = self.check("for");
        let node = node!(for_await_of_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_of_statement_await_vars(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check(")");
        let (right, ..) = self.pop_node();
        self.check("of");
        let (binding, _, var_end) = self.pop_node();
        let (var, var_start, _) = self.pop_token();
        self.check("(");
        self.check("await");
        let (start, _) = self.check("for");
        let left = node!(variable_declaration@var_start..var_end; var, vec![binding]);
        let node = node!(for_await_of_statement@start..end; left, right, body);
        self.push_node(node, start, end);
    }

    fn for_declaration(&mut self) {
        let (binding, _, end) = self.pop_node();
        let (kind, start, _) = self.pop_token();
        let node = node!(variable_declaration@start..end; kind, vec![binding]);
        self.push_node(node, start, end);
    }

    fn for_binding(&mut self) {
        let (id, start, end) = self.pop_node();
        let node = node!(variable_declarator@start..end; id);
        self.push_node(node, start, end);
    }

    fn return_statement_no_argument(&mut self) {
        let (_, end) = self.check(";");
        let (start, _) = self.check("return");
        let node = node!(return_statement@start..end);
        self.push_node(node, start, end);
    }

    fn return_statement(&mut self) {
        let (_, end) = self.check(";");
        let (argument, ..) = self.pop_node();
        let (start, _) = self.check("return");
        let node = node!(return_statement@start..end; argument);
        self.push_node(node, start, end);
    }

    fn class_declaration(&mut self) {
        let (class_tail, _, end) = self.pop_node();
        let (id, ..) = self.pop_node();
        let (start, _) = self.check("class");
        let node = node!(class_declaration@start..end; id, class_tail);
        self.push_node(node, start, end);
    }

    fn anonymous_class_declaration(&mut self) {
        let (class_tail, _, end) = self.pop_node();
        let (start, _) = self.check("class");
        let node = node!(class_declaration@start..end; class_tail);
        self.push_node(node, start, end);
    }

    fn empty_class_tail(&mut self) {
        let (_, end) = self.check("}");
        let (start, _) = self.check("{");
        let body = node!(class_body@start..end);
        let node = node!(class_tail; body);
        self.push_node(node, start, end);
    }

    fn class_tail_no_body(&mut self) {
        let (_, end) = self.check("}");
        let (body_start, _) = self.check("{");
        let (super_class, start, ..) = self.pop_node();
        let body = node!(class_body@body_start..end);
        let node = node!(class_tail; super_class, body);
        self.push_node(node, start, end);
    }

    fn class_tail_no_super_class(&mut self) {
        let (_, end) = self.check("}");
        let (list, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let body = node!(class_body@start..end; list);
        let node = node!(class_tail; body);
        self.push_node(node, start, end);
    }

    fn class_tail(&mut self) {
        let (_, end) = self.check("}");
        let (list, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        let (super_class, start, _) = self.pop_node();
        let body = node!(class_body@body_start..end; list);
        let node = node!(class_tail; super_class, body);
        self.push_node(node, start, end);
    }

    fn class_heritage(&mut self) {
        let (super_class, _, end) = self.pop_node();
        let (start, _) = self.check("extends");
        self.push_node(super_class, start, end);
    }

    fn class_element_list(&mut self) {
        let (nullable, start, end) = self.pop_nullable();
        let list = match nullable {
            Some(node) => vec![node],
            _ => vec![],
        };
        self.push_list(list, start, end);
    }

    fn class_element_list_append(&mut self) {
        let (nullable, _, end) = self.pop_nullable();
        let (mut list, start, _) = self.pop_list();
        if let Some(node) = nullable {
            list.push(node);
        }
        self.push_list(list, start, end);
    }

    fn class_element_static_method_definition(&mut self) {
        let (method, _, end) = self.pop_node();
        let (start, _) = self.check("static");
        let node = node!(static_method_definition@start..end; method);
        self.push_nullable(Some(node), start, end);
    }

    fn class_element_property_definition(&mut self) {
        let (_, end) = self.check(";");
        let (def, start, _) = self.pop_node();
        let node = node!(end_property_definition@start..end; def);
        self.push_node(node, start, end);
    }

    fn class_element_static_property_definition(&mut self) {
        let (_, end) = self.check(";");
        let (def, ..) = self.pop_node();
        let (start, _) = self.check("static");
        let node = node!(static_property_definition@start..end; def);
        self.push_nullable(Some(node), start, end);
    }

    fn class_element_semicolon(&mut self) {
        let (start, end) = self.check(";");
        self.push_nullable(None, start, end);
    }

    fn class_element_name_private(&mut self) {
        let (name, start, end) = self.pop_token();
        let node = node!(private_identifier@start..end; name);
        self.push_node(node, start, end);
    }

    fn static_block(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        self.check("{");
        let (start, _) = self.check("static");
        let node = node!(static_block@start..end; body);
        self.push_node(node, start, end);
    }

    fn property_definition(&mut self) {
        let (key, start, end) = self.pop_node();
        let node = node!(property_definition@start..end; key);
        self.push_node(node, start, end);
    }

    fn property_definition_value(&mut self) {
        let (value, _, end) = self.pop_node();
        let (key, start, _) = self.pop_node();
        let node = node!(property_definition@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        let (func_expr_start, _) = self.check("(");
        let (key, start, _) = self.pop_node();
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(function_expression@func_expr_start..end; params, func_body);
        let node = node!(method_definition@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition_generator(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        let (func_expr_start, _) = self.check("(");
        let (key, ..) = self.pop_node();
        let (start, _) = self.check("*");
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(generator_expression@func_expr_start..end; params, func_body);
        let node = node!(method_definition@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition_async(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        let (func_expr_start, _) = self.check("(");
        let (key, ..) = self.pop_node();
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(async_function_expression@func_expr_start..end; params, func_body);
        let node = node!(method_definition@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition_async_generator(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        let (func_expr_start, _) = self.check("(");
        let (key, ..) = self.pop_node();
        self.check("*");
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(async_generator_expression@func_expr_start..end; params, func_body);
        let node = node!(method_definition@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition_get(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (func_expr_start, _) = self.check("(");
        let (key, ..) = self.pop_node();
        let (start, _) = self.check("get");
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(function_expression@func_expr_start..end; func_body);
        let node = node!(getter@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn method_definition_set(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        let (func_expr_start, _) = self.check("(");
        let (key, ..) = self.pop_node();
        let (start, _) = self.check("set");
        let func_body = node!(function_body@body_start..end; body);
        let value = node!(function_expression@func_expr_start..end; params, func_body);
        let node = node!(setter@start..end; key, value);
        self.push_node(node, start, end);
    }

    fn function_declaration(&mut self) {
        self.function(true);
    }

    fn function_expression(&mut self) {
        self.function(false);
    }

    fn function(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        let (id, ..) = self.pop_node();
        let (start, _) = self.check("function");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(function_declaration@start..end; id, params, func_body)
        } else {
            node!(function_expression@start..end; id, params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn anonymous_function_declaration(&mut self) {
        self.anonymous_function(true);
    }

    fn anonymous_function_expression(&mut self) {
        self.anonymous_function(false);
    }

    fn anonymous_function(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        let (start, _) = self.check("function");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(function_declaration@start..end; params, func_body)
        } else {
            node!(function_expression@start..end; params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn generator_declaration(&mut self) {
        self.generator(true);
    }

    fn generator_expression(&mut self) {
        self.generator(false);
    }

    fn generator(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        let (id, ..) = self.pop_node();
        self.check("*");
        let (start, _) = self.check("function");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(generator_declaration@start..end; id, params, func_body)
        } else {
            node!(generator_expression@start..end; id, params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn anonymous_generator_declaration(&mut self) {
        self.anonymous_generator(true);
    }

    fn anonymous_generator_expression(&mut self) {
        self.anonymous_generator(false);
    }

    fn anonymous_generator(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        self.check("*");
        let (start, _) = self.check("function");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(generator_declaration@start..end; params, func_body)
        } else {
            node!(generator_expression@start..end; params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn async_function_declaration(&mut self) {
        self.async_function(true);
    }

    fn async_function_expression(&mut self) {
        self.async_function(false);
    }

    fn async_function(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        let (id, ..) = self.pop_node();
        self.check("function");
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(async_function_declaration@start..end; id, params, func_body)
        } else {
            node!(async_function_expression@start..end; id, params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn anonymous_async_function_declaration(&mut self) {
        self.anonymous_async_function(true);
    }

    fn anonymous_async_function_expression(&mut self) {
        self.anonymous_async_function(false);
    }

    fn anonymous_async_function(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        self.check("function");
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(async_function_declaration@start..end; params, func_body)
        } else {
            node!(async_function_expression@start..end; params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn async_generator_declaration(&mut self) {
        self.async_generator(true);
    }

    fn async_generator_expression(&mut self) {
        self.async_generator(false);
    }

    fn async_generator(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        let (id, ..) = self.pop_node();
        self.check("*");
        self.check("function");
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(async_generator_declaration@start..end; id, params, func_body)
        } else {
            node!(async_generator_expression@start..end; id, params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn anonymous_async_generator_declaration(&mut self) {
        self.anonymous_async_generator(true);
    }

    fn anonymous_async_generator_expression(&mut self) {
        self.anonymous_async_generator(false);
    }

    fn anonymous_async_generator(&mut self, decl: bool) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (body_start, _) = self.check("{");
        self.check(")");
        let (params, ..) = self.pop_list();
        self.check("(");
        self.check("*");
        self.check("function");
        let (start, _) = self.check("async");
        let func_body = node!(function_body@body_start..end; body);
        let node = if decl {
            node!(async_generator_declaration@start..end; params, func_body)
        } else {
            node!(async_generator_expression@start..end; params, func_body)
        };
        self.push_node(node, start, end);
    }

    fn rest_element(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (start, _) = self.check("...");
        let node = node!(rest_element@start..end; argument);
        self.push_node(node, start, end);
    }

    fn function_body_block(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(block_statement@start..end; body);
        self.push_node(node, start, end);
    }

    fn from_clause(&mut self) {
        let (source, _, end) = self.pop_node();
        let (start, _) = self.check("from");
        self.push_node(source, start, end);
    }

    fn export_all_as(&mut self) {
        let (exported, _, end) = self.pop_node();
        self.check("as");
        let (start, _) = self.check("*");
        self.push_node(exported, start, end);
    }

    fn member_expression(&mut self) {
        let (property, _, end) = self.pop_node();
        self.check(".");
        let (object, start, _) = self.pop_node();
        let node = node!(member_expression@start..end; object, property, false);
        self.push_node(node, start, end);
    }

    fn member_expression_computed(&mut self) {
        let (_, end) = self.check("]");
        let (property, ..) = self.pop_node();
        self.check("[");
        let (object, start, _) = self.pop_node();
        let node = node!(member_expression@start..end; object, property, true);
        self.push_node(node, start, end);
    }

    fn member_expression_private(&mut self) {
        let (name, id_start, end) = self.pop_token();
        self.check(".");
        let (object, start, _) = self.pop_node();
        let property = node!(private_identifier@id_start..end; name);
        let node = node!(member_expression@start..end; object, property, false);
        self.push_node(node, start, end);
    }

    fn member_expression_call(&mut self) {
        let (_, end) = self.check("]");
        let (property, ..) = self.pop_node();
        self.check("[");
        let (object, start, _) = self.pop_node();
        let node = node!(member_expression@start..end; object, property, true);
        self.push_node(node, start, end);
    }

    fn super_property_computed(&mut self) {
        let (_, end) = self.check("]");
        let (property, ..) = self.pop_node();
        self.check("[");
        let (start, super_end) = self.check("super");
        let object = node!(super_@start..super_end);
        let node = node!(member_expression@start..end; object, property, true);
        self.push_node(node, start, end);
    }

    fn super_property(&mut self) {
        let (property, _, end) = self.pop_node();
        self.check(".");
        let (start, super_end) = self.check("super");
        let object = node!(super_@start..super_end);
        let node = node!(member_expression@start..end; object, property, false);
        self.push_node(node, start, end);
    }

    fn tagged_template_expression(&mut self) {
        let (quasi, _, end) = self.pop_node();
        let (tag, start, _) = self.pop_node();
        let node = node!(tagged_template_expression@start..end; tag, quasi);
        self.push_node(node, start, end);
    }

    fn call_expression(&mut self) {
        let (arguments, _, end) = self.pop_list();
        let (callee, start, _) = self.pop_node();
        let node = node!(call_expression@start..end; callee, arguments);
        self.push_node(node, start, end);
    }

    fn call_expression_super(&mut self) {
        let (arguments, _, end) = self.pop_list();
        let (start, super_end) = self.check("super");
        let callee = node!(super_@start..super_end);
        let node = node!(call_expression@start..end; callee, arguments);
        self.push_node(node, start, end);
    }

    fn import_expression(&mut self) {
        let (_, end) = self.check(")");
        let (source, ..) = self.pop_node();
        self.check("(");
        let (start, _) = self.check("import");
        let node = node!(import_expression@start..end; source);
        self.push_node(node, start, end);
    }

    fn assignment_expression(&mut self) {
        let (right, _, end) = self.pop_node();
        let (operator, ..) = self.pop_token();
        let (left, start, _) = self.pop_node();
        // If LeftHandSideExpression is an ObjectLiteral or an ArrayLiteral, it must cover an
        // AssignmentPattern.  See "13.15.1 Static Semantics: Early Errors" in ECMA-262.
        let left = node!(into_pattern; left);
        let node = node!(assignment_expression@start..end; operator, left, right);
        self.push_node(node, start, end);
    }

    fn conditional_expression(&mut self) {
        let (alternate, _, end) = self.pop_node();
        self.check(":");
        let (consequent, ..) = self.pop_node();
        self.check("?");
        let (test, start, _) = self.pop_node();
        let node = node!(conditional_expression@start..end; test, consequent, alternate);
        self.push_node(node, start, end);
    }

    fn binary_expression(&mut self) {
        let (right, _, end) = self.pop_node();
        let (operator, ..) = self.pop_token();
        let (left, start, _) = self.pop_node();
        let node = node!(binary_expression@start..end; operator, left, right);
        self.push_node(node, start, end);
    }

    fn binary_expression_private(&mut self) {
        let (right, _, end) = self.pop_node();
        let (operator, ..) = self.pop_token();
        let (name, start, id_end) = self.pop_token();
        let left = node!(private_identifier@start..id_end; name);
        let node = node!(binary_expression@start..end; operator, left, right);
        self.push_node(node, start, end);
    }

    fn logical_expression(&mut self) {
        let (right, _, end) = self.pop_node();
        let (operator, ..) = self.pop_token();
        let (left, start, _) = self.pop_node();
        let node = node!(logical_expression@start..end; operator, left, right);
        self.push_node(node, start, end);
    }

    fn this_expression(&mut self) {
        let (start, end) = self.check("this");
        let node = node!(this_expression@start..end);
        self.push_node(node, start, end);
    }

    fn update_expression_prefix(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (operator, start, _) = self.pop_token();
        let node = node!(update_expression@start..end; operator, argument, true);
        self.push_node(node, start, end);
    }

    fn update_expression_suffix(&mut self) {
        let (operator, _, end) = self.pop_token();
        let (argument, start, _) = self.pop_node();
        let node = node!(update_expression@start..end; operator, argument, false);
        self.push_node(node, start, end);
    }

    fn unary_expression(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (operator, start, _) = self.pop_token();
        let node = node!(unary_expression@start..end; operator, argument);
        self.push_node(node, start, end);
    }

    fn await_expression(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (start, _) = self.check("await");
        let node = node!(await_expression@start..end; argument);
        self.push_node(node, start, end);
    }

    fn yield_expression_no_argument(&mut self) {
        let (start, end) = self.check("yield");
        let node = node!(yield_expression@start..end; false);
        self.push_node(node, start, end);
    }

    fn yield_expression(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (start, _) = self.check("yield");
        let node = node!(yield_expression@start..end; argument, false);
        self.push_node(node, start, end);
    }

    fn yield_expression_delegate(&mut self) {
        let (argument, _, end) = self.pop_node();
        self.check("*");
        let (start, _) = self.check("yield");
        let node = node!(yield_expression@start..end; argument, true);
        self.push_node(node, start, end);
    }

    fn new_expression(&mut self) {
        let (callee, _, end) = self.pop_node();
        let (start, _) = self.check("new");
        let node = node!(new_expression@start..end; callee);
        self.push_node(node, start, end);
    }

    fn new_expression_arguments(&mut self) {
        let (arguments, _, end) = self.pop_list();
        let (callee, ..) = self.pop_node();
        let (start, _) = self.check("new");
        let node = node!(new_expression@start..end; callee, arguments);
        self.push_node(node, start, end);
    }

    fn arrow_function_expression(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check("=>");
        let (params, start, _) = self.pop_list();
        let node = node!(arrow_function_expression@start..end; params, body);
        self.push_node(node, start, end);
    }

    fn async_arrow_function_expression_single_param(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check("=>");
        let (param, ..) = self.pop_node();
        let (start, _) = self.check("async");
        let node = node!(async_arrow_function_expression@start..end; vec![param], body);
        self.push_node(node, start, end);
    }

    fn async_arrow_function_expression(&mut self) {
        let (body, _, end) = self.pop_node();
        self.check("=>");
        let (params, ..) = self.pop_list();
        let (start, _) = self.check("async");
        let node = node!(async_arrow_function_expression@start..end; params, body);
        self.push_node(node, start, end);
    }

    fn arrow_parameters(&mut self) {
        let (nullable, start, end) = self.pop_nullable();
        let list = node!(into_patterns; nullable);
        self.push_list(list, start, end);
    }

    fn sequence_expression(&mut self) {
        let (expr2, _, end) = self.pop_node();
        self.check(",");
        let (expr1, start, _) = self.pop_node();
        let node = node!(sequence_expression@start..end; match *expr1 {
            Node::SequenceExpression(ref seq) => {
                // TODO: consider to use an immutable collection
                let mut expressions = seq.expressions.clone();
                expressions.push(expr2);
                expressions
            }
            _ => vec![expr1, expr2],
        });
        self.push_node(node, start, end);
    }

    fn anonymous_class_expression(&mut self) {
        let (class_tail, _, end) = self.pop_node();
        let (start, _) = self.check("class");
        let node = node!(class_expression@start..end; class_tail);
        self.push_node(node, start, end);
    }

    fn class_expression(&mut self) {
        let (class_tail, _, end) = self.pop_node();
        let (id, ..) = self.pop_node();
        let (start, _) = self.check("class");
        let node = node!(class_expression@start..end; id, class_tail);
        self.push_node(node, start, end);
    }

    fn new_target(&mut self) {
        let (property_start, end) = self.check("target");
        self.check(".");
        let (start, meta_end) = self.check("meta");
        let meta = node!(identifier@start..meta_end; "meta".to_string());
        let property = node!(identifier@property_start..end; "target".to_string());
        let node = node!(meta_property@start..end; meta, property);
        self.push_node(node, start, end);
    }

    fn import_meta(&mut self) {
        let (property_start, end) = self.check("meta");
        self.check(".");
        let (start, meta_end) = self.check("import");
        let meta = node!(identifier@start..meta_end; "import".to_string());
        let property = node!(identifier@property_start..end; "meta".to_string());
        let node = node!(meta_property@start..end; meta, property);
        self.push_node(node, start, end);
    }

    fn object_expression_empty(&mut self) {
        let (_, end) = self.check("}");
        let (start, _) = self.check("{");
        let node = node!(object_expression@start..end);
        self.push_node(node, start, end);
    }

    fn object_expression(&mut self) {
        let (_, end) = self.check("}");
        let (properties, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(object_expression@start..end; properties);
        self.push_node(node, start, end);
    }

    fn property(&mut self) {
        let (key, start, end) = self.pop_node();
        let node = node!(property@start..end; key);
        self.push_node(node, start, end);
    }

    fn property_value(&mut self) {
        let (value, _, end) = self.pop_node();
        self.check(":");
        let (key, start, _) = self.pop_node();
        let node = node!(property@start..end; key => value);
        self.push_node(node, start, end);
    }

    fn property_method(&mut self) {
        let (method, start, end) = self.pop_node();
        let node = node!(into_property; method);
        self.push_node(node, start, end);
    }

    fn object_expression_comma(&mut self) {
        let (_, end) = self.check("}");
        self.check(",");
        let (properties, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(object_expression@start..end; properties);
        self.push_node(node, start, end);
    }

    fn array_expression_empty(&mut self) {
        let (_, end) = self.check("]");
        let (start, _) = self.check("[");
        let node = node!(array_expression@start..end);
        self.push_node(node, start, end);
    }

    fn array_expression(&mut self) {
        let (_, end) = self.check("]");
        let (elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        let node = node!(array_expression@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_expression_comma(&mut self) {
        let (_, end) = self.check("]");
        self.check(",");
        let (elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        let node = node!(array_expression@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_expression_comma_elision(&mut self) {
        let (_, end) = self.check("]");
        let (mut elision, ..) = self.pop_array();
        self.check(",");
        let (mut elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        elements.append(&mut elision);
        let node = node!(array_expression@start..end; elements);
        self.push_node(node, start, end);
    }

    fn variable_declarator(&mut self) {
        let (id, start, end) = self.pop_node();
        let node = node!(variable_declarator@start..end; id);
        self.push_node(node, start, end);
    }

    fn variable_declarator_init(&mut self) {
        let (init, _, end) = self.pop_node();
        let (id, start, _) = self.pop_node();
        let node = node!(variable_declarator@start..end; id, init);
        self.push_node(node, start, end);
    }

    fn arguments(&mut self) {
        let (_, end) = self.check(")");
        let (list, ..) = self.pop_list();
        let (start, _) = self.check("(");
        self.push_list(list, start, end);
    }

    fn arguments_empty(&mut self) {
        let (_, end) = self.check(")");
        let (start, _) = self.check("(");
        self.push_list(vec![], start, end);
    }

    fn arguments_comma(&mut self) {
        let (_, end) = self.check(")");
        self.check(",");
        let (list, ..) = self.pop_list();
        let (start, _) = self.check("(");
        self.push_list(list, start, end);
    }

    fn argument_list_rest(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (start, _) = self.check("...");
        let node = node!(spread_element@start..end; argument);
        self.push_list(vec![node], start, end);
    }

    fn argument_list_append_rest(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (rest_start, _) = self.check("...");
        self.check(",");
        let (mut list, start, _) = self.pop_list();
        let node = node!(spread_element@rest_start..end; argument);
        list.push(node);
        self.push_list(list, start, end);
    }

    fn expression_or_arguments_expr(&mut self) {
        let (_, end) = self.check(")");
        let (node, ..) = self.pop_node();
        let (start, _) = self.check("(");
        self.push_nullable(Some(node), start, end);
    }

    fn expression_or_arguments_expr_comma(&mut self) {
        let (_, end) = self.check(")");
        self.check(",");
        let (node, ..) = self.pop_node();
        let (start, _) = self.check("(");
        self.push_nullable(Some(node), start, end);
    }

    fn expression_or_arguments_empty(&mut self) {
        let (_, end) = self.check(")");
        let (start, _) = self.check("(");
        self.push_nullable(None, start, end);
    }

    fn expression_or_arguments_rest(&mut self) {
        let (_, end) = self.check(")");
        let (argument, _, rest_end) = self.pop_node();
        let (rest_start, _) = self.check("...");
        let (start, _) = self.check("(");
        let node = node!(rest_element@rest_start..rest_end; argument);
        self.push_nullable(Some(node), start, end);
    }

    fn expression_or_arguments_expr_rest(&mut self) {
        let (_, end) = self.check(")");
        let (argument, _, rest_end) = self.pop_node();
        let (rest_start, _) = self.check("...");
        self.check(",");
        let (expr, expr_start, _) = self.pop_node();
        let (start, _) = self.check("(");
        let rest = node!(rest_element@rest_start..rest_end; argument);
        let expressions = match *expr {
            Node::SequenceExpression(ref seq) => {
                let mut expressions = seq.expressions.clone();
                expressions.push(rest);
                expressions
            }
            _ => vec![expr, rest],
        };
        let node = node!(sequence_expression@expr_start..rest_end; expressions);
        self.push_nullable(Some(node), start, end);
    }

    fn optional_expression(&mut self) {
        let (chains, _, end) = self.pop_list();
        let (expr, start, _) = self.pop_node();
        let node = node!(chain_expression@start..end; expr, chains);
        self.push_node(node, start, end);
    }

    fn either_left(&mut self) {
        let (node, start, end) = self.pop_left();
        self.push_node(node, start, end);
    }

    fn either_right(&mut self) {
        let (node, start, end) = self.pop_right();
        self.push_node(node, start, end);
    }

    fn optional_call(&mut self) {
        let (arguments, _, end) = self.pop_list();
        let (start, _) = self.check("?.");
        let node = node!(optional_call@end; arguments);
        self.push_list(vec![node], start, end);
    }

    fn optional_computed_property(&mut self) {
        let (_, end) = self.check("]");
        let (expr, ..) = self.pop_node();
        self.check("[");
        let (start, _) = self.check("?.");
        let node = node!(optional_member@end; expr, true);
        self.push_list(vec![node], start, end);
    }

    fn optional_member(&mut self) {
        let (expr, _, end) = self.pop_node();
        let (start, _) = self.check("?.");
        let node = node!(optional_member@end; expr, false);
        self.push_list(vec![node], start, end);
    }

    fn optional_chain_append_call(&mut self) {
        let (arguments, _, end) = self.pop_list();
        let (mut list, start, ..) = self.pop_list();
        let node = node!(optional_call@end; arguments);
        list.push(node);
        self.push_list(list, start, end);
    }

    fn optional_chain_append_computed_property(&mut self) {
        let (_, end) = self.check("]");
        let (expr, ..) = self.pop_node();
        self.check("[");
        let (mut list, start, _) = self.pop_list();
        let node = node!(optional_member@end; expr, true);
        list.push(node);
        self.push_list(list, start, end);
    }

    fn optional_chain_append_property(&mut self) {
        let (property, _, end) = self.pop_node();
        self.check(".");
        let (mut list, start, ..) = self.pop_list();
        let node = node!(optional_member@end; property, false);
        list.push(node);
        self.push_list(list, start, end);
    }

    fn optional_chain_append_tagged_template(&mut self) {
        let (template, _, end) = self.pop_node();
        let (mut list, start, ..) = self.pop_list();
        let node = node!(optional_member@end; template, false);
        list.push(node);
        self.push_list(list, start, end);
    }

    fn object_pattern_empty(&mut self) {
        let (_, end) = self.check("}");
        let (start, _) = self.check("{");
        let node = node!(object_pattern@start..end);
        self.push_node(node, start, end);
    }

    fn object_pattern_rest(&mut self) {
        let (_, end) = self.check("}");
        let (rest, ..) = self.pop_node();
        let (start, _) = self.check("{");
        let node = node!(object_pattern@start..end; vec![rest]);
        self.push_node(node, start, end);
    }

    fn object_pattern(&mut self) {
        let (_, end) = self.check("}");
        let (properties, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(object_pattern@start..end; properties);
        self.push_node(node, start, end);
    }

    fn object_pattern_comma(&mut self) {
        let (_, end) = self.check("}");
        self.check(",");
        let (properties, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(object_pattern@start..end; properties);
        self.push_node(node, start, end);
    }

    fn object_pattern_comma_rest(&mut self) {
        let (_, end) = self.check("}");
        let (rest, ..) = self.pop_node();
        self.check(",");
        let (mut properties, ..) = self.pop_list();
        let (start, _) = self.check("{");
        properties.push(rest);
        let node = node!(object_pattern@start..end; properties);
        self.push_node(node, start, end);
    }

    fn binding_identifier_or_property(&mut self) {
        let (left, start, end) = self.pop_node();
        let right = node!(property@start..end; left.clone());
        self.push_either(left, right, start, end);
    }

    fn assignment_pattern_or_property(&mut self) {
        let (value, _, end) = self.pop_node();
        let (key, start, _) = self.pop_node();
        let left = node!(assignment_pattern@start..end; key.clone(), value.clone());
        let right = node!(property@start..end; key => value);
        self.push_either(left, right, start, end);
    }

    fn assignment_pattern(&mut self) {
        let (right, _, end) = self.pop_node();
        let (left, start, _) = self.pop_node();
        let node = node!(assignment_pattern@start..end; left, right);
        self.push_node(node, start, end);
    }

    fn array_pattern_empty(&mut self) {
        let (_, end) = self.check("]");
        let (start, _) = self.check("[");
        let node = node!(array_pattern@start..end);
        self.push_node(node, start, end);
    }

    fn array_pattern_rest(&mut self) {
        let (_, end) = self.check("]");
        let (rest, ..) = self.pop_node();
        let (start, _) = self.check("[");
        let node = node!(array_pattern@start..end; vec![Some(rest)]);
        self.push_node(node, start, end);
    }

    fn array_pattern_elision_rest(&mut self) {
        let (_, end) = self.check("]");
        let (rest, ..) = self.pop_node();
        let (mut elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        elements.push(Some(rest));
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_pattern_comma(&mut self) {
        let (_, end) = self.check("]");
        self.check(",");
        let (elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_pattern_concat(&mut self) {
        let (_, end) = self.check("]");
        let (mut array, ..) = self.pop_array();
        self.check(",");
        let (mut elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        elements.append(&mut array);
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_pattern_comma_rest(&mut self) {
        let (_, end) = self.check("]");
        let (rest, ..) = self.pop_node();
        self.check(",");
        let (mut elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        elements.push(Some(rest));
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_pattern_concat_rest(&mut self) {
        let (_, end) = self.check("]");
        let (rest, ..) = self.pop_node();
        let (mut array, ..) = self.pop_array();
        self.check(",");
        let (mut elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        elements.append(&mut array);
        elements.push(Some(rest));
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn array_pattern(&mut self) {
        let (_, end) = self.check("]");
        let (elements, ..) = self.pop_array();
        let (start, _) = self.check("[");
        let node = node!(array_pattern@start..end; elements);
        self.push_node(node, start, end);
    }

    fn spread_element(&mut self) {
        let (argument, _, end) = self.pop_node();
        let (start, _) = self.check("...");
        let node = node!(spread_element@start..end; argument);
        self.push_node(node, start, end);
    }

    fn computed_property_name(&mut self) {
        let (_, end) = self.check("]");
        let (expr, ..) = self.pop_node();
        let (start, _) = self.check("[");
        let node = node!(computed_property_name; expr);
        self.push_node(node, start, end);
    }

    fn identifier(&mut self) {
        let (name, start, end) = self.pop_token();
        let node = node!(identifier@start..end; name);
        self.push_node(node, start, end);
    }

    fn null_literal(&mut self) {
        let (start, end) = self.check("null");
        let node = node!(null@start..end);
        self.push_node(node, start, end);
    }

    fn boolean_literal_true(&mut self) {
        let (start, end) = self.check("true");
        let node = node!(true@start..end);
        self.push_node(node, start, end);
    }

    fn boolean_literal_false(&mut self) {
        let (start, end) = self.check("false");
        let node = node!(false@start..end);
        self.push_node(node, start, end);
    }

    fn numeric_literal(&mut self) {
        let (token, start, end) = self.pop_token();
        let node = node!(number@start..end; token);
        self.push_node(node, start, end);
    }

    fn string_literal(&mut self) {
        let (token, start, end) = self.pop_token();
        let node = node!(string@start..end; token);
        self.push_node(node, start, end);
    }

    fn regexp_literal(&mut self) {
        let (token, start, end) = self.pop_token();
        let node = node!(regexp@start..end; token);
        self.push_node(node, start, end);
    }

    fn template_literal_no_subst(&mut self) {
        let (token, start, end) = self.pop_token();
        let raw = &token[TEMPLATE_START_LEN..(token.len() - TEMPLATE_END_LEN)];
        let element_start = start.forward(TEMPLATE_START_LEN);
        let element_end = end.backward(TEMPLATE_END_LEN);
        let element = node!(template_element@element_start..element_end; raw, true);
        let node = node!(template_literal@start..end; vec![element], vec![]);
        self.push_node(node, start, end);
    }

    fn template_literal(&mut self) {
        let (mut expressions, ..) = self.pop_list();
        let (mut quasis, _, end) = self.pop_list();
        let (expr, ..) = self.pop_node();
        let (token, start, token_end) = self.pop_token();
        let raw = &token[TEMPLATE_START_LEN..(token.len() - TEMPLATE_SUBST_START_LEN)];
        let element_start = start.forward(TEMPLATE_START_LEN);
        let element_end = token_end.backward(TEMPLATE_SUBST_START_LEN);
        let node = node!(template_element@element_start..element_end; raw, false);
        quasis.insert(0, node);
        expressions.insert(0, expr);
        let node = node!(template_literal@start..end; quasis, expressions);
        self.push_node(node, start, end);
    }

    fn template_spans_tail(&mut self) {
        let (token, start, end) = self.pop_token();
        let raw = &token[TEMPLATE_SUBST_END_LEN..(token.len() - TEMPLATE_END_LEN)];
        let element_start = start.forward(TEMPLATE_SUBST_END_LEN);
        let element_end = end.backward(TEMPLATE_END_LEN);
        let node = node!(template_element@element_start..element_end; raw, true);
        self.push_list(vec![node], start, end);
        self.push_list(vec![], Default::default(), Default::default());
    }

    fn template_spans_append(&mut self) {
        let (token, token_start, end) = self.pop_token();
        let (expressions, ..) = self.pop_list();
        let (mut quasis, start, _) = self.pop_list();
        let raw = &token[TEMPLATE_SUBST_END_LEN..(token.len() - TEMPLATE_END_LEN)];
        let element_start = token_start.forward(TEMPLATE_SUBST_END_LEN);
        let element_end = end.backward(TEMPLATE_END_LEN);
        let node = node!(template_element@element_start..element_end; raw, true);
        quasis.push(node);
        self.push_list(quasis, start, end);
        self.push_list(expressions, Default::default(), Default::default());
    }

    fn template_middle_list(&mut self) {
        let (expr, _, end) = self.pop_node();
        let (token, start, token_end) = self.pop_token();
        let raw = &token[TEMPLATE_SUBST_END_LEN..(token.len() - TEMPLATE_SUBST_START_LEN)];
        let element_start = start.forward(TEMPLATE_SUBST_END_LEN);
        let element_end = token_end.backward(TEMPLATE_SUBST_START_LEN);
        let node = node!(template_element@element_start..element_end; raw, false);
        self.push_list(vec![node], start, end);
        self.push_list(vec![expr], Default::default(), Default::default());
    }

    fn template_middle_list_append(&mut self) {
        let (expr, _, end) = self.pop_node();
        let (token, token_start, token_end) = self.pop_token();
        let (mut expressions, ..) = self.pop_list();
        let (mut quasis, start, _) = self.pop_list();
        let raw = &token[TEMPLATE_SUBST_END_LEN..(token.len() - TEMPLATE_SUBST_START_LEN)];
        let element_start = token_start.forward(TEMPLATE_SUBST_END_LEN);
        let element_end = token_end.backward(TEMPLATE_SUBST_START_LEN);
        let node = node!(template_element@element_start..element_end; raw, false);
        quasis.push(node);
        expressions.push(expr);
        self.push_list(quasis, start, end);
        self.push_list(expressions, Default::default(), Default::default());
    }

    fn block_statement_empty(&mut self) {
        let (_, end) = self.check("}");
        let (start, _) = self.check("{");
        let node = node!(block_statement@start..end);
        self.push_node(node, start, end);
    }

    fn block_statement(&mut self) {
        let (_, end) = self.check("}");
        let (body, ..) = self.pop_list();
        let (start, _) = self.check("{");
        let node = node!(block_statement@start..end; body);
        self.push_node(node, start, end);
    }

    fn empty_list_block(&mut self) {
        let (_, end) = self.check("}");
        let (start, _) = self.check("{");
        self.push_list(vec![], start, end);
    }

    fn list_block(&mut self) {
        let (_, end) = self.check("}");
        let (list, ..) = self.pop_list();
        let (start, _) = self.check("{");
        self.push_list(list, start, end);
    }

    fn list_block_ended_with_comma(&mut self) {
        let (_, end) = self.check("}");
        self.check(",");
        let (list, ..) = self.pop_list();
        let (start, _) = self.check("{");
        self.push_list(list, start, end);
    }

    fn csv_list(&mut self) {
        let (node1, _, end) = self.pop_node();
        self.check(",");
        let (node0, start, _) = self.pop_node();
        self.push_list(vec![node0, node1], start, end);
    }

    fn prepend_to_csv_list(&mut self) {
        let (mut list, _, end) = self.pop_list();
        self.check(",");
        let (node, start, _) = self.pop_node();
        list.insert(0, node);
        self.push_list(list, start, end);
    }

    fn elision(&mut self) {
        let (start, end) = self.check(",");
        self.push_array(vec![None], start, end);
    }

    fn elision_append(&mut self) {
        let (_, end) = self.check(",");
        let (mut array, start, _) = self.pop_array();
        array.push(None);
        self.push_array(array, start, end);
    }

    fn initializer(&mut self) {
        let (expr, _, end) = self.pop_node();
        let (start, _) = self.check("=");
        self.push_node(expr, start, end);
    }

    fn append_to_csv_list(&mut self) {
        let (node, _, end) = self.pop_node();
        self.check(",");
        let (mut list, start, _) = self.pop_list();
        list.push(node);
        self.push_list(list, start, end);
    }

    fn into_nullable(&mut self) {
        let (node, start, end) = self.pop_node();
        self.push_nullable(Some(node), start, end);
    }

    fn unwrap_nullable(&mut self) {
        let (nullable, start, end) = self.pop_nullable();
        self.push_node(nullable.unwrap(), start, end);
    }

    fn empty_list(&mut self) {
        let start = self.location.clone();
        let end = self.location.clone();
        self.push_list(vec![], start, end);
    }

    fn into_list(&mut self) {
        let (node, start, end) = self.pop_node();
        self.push_list(vec![node], start, end);
    }

    fn append_to_list(&mut self) {
        let (node, _, end) = self.pop_node();
        let (mut list, start, _) = self.pop_list();
        list.push(node);
        self.push_list(list, start, end);
    }

    fn into_array(&mut self) {
        let (node, start, end) = self.pop_node();
        self.push_array(vec![Some(node)], start, end);
    }

    fn append_to_array(&mut self) {
        let (node, _, end) = self.pop_node();
        let (mut array, start, _) = self.pop_array();
        array.push(Some(node));
        self.push_array(array, start, end);
    }

    fn append_to_csv_array(&mut self) {
        let (node, _, end) = self.pop_node();
        self.check(",");
        let (mut array, start, _) = self.pop_array();
        array.push(Some(node));
        self.push_array(array, start, end);
    }

    fn concat_csv_arrays(&mut self) {
        let (mut tail, _, end) = self.pop_array();
        self.check(",");
        let (mut array, start, _) = self.pop_array();
        array.append(&mut tail);
        self.push_array(array, start, end);
    }

    fn concat_and_append_array(&mut self) {
        let (node, _, end) = self.pop_node();
        let (mut sparse, ..) = self.pop_array();
        self.check(",");
        let (mut array, start, _) = self.pop_array();
        array.append(&mut sparse);
        array.push(Some(node));
        self.push_array(array, start, end);
    }

    fn remove_comma(&mut self) {
        self.check(",");
    }

    fn nop(&mut self) {}

    fn push_token(&mut self, token: String, start: Location, end: Location) {
        self.stack.push((StackValue::Token(token), start, end))
    }

    fn pop_token(&mut self) -> (String, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Token(token), start, end) => (token, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn check(&mut self, expected: &str) -> (Location, Location) {
        let (token, start, end) = self.pop_token();
        assert_eq!(token, expected);
        (start, end)
    }

    fn push_node(&mut self, node: NodeRef, start: Location, end: Location) {
        self.stack.push((StackValue::Node(node), start, end));
    }

    fn pop_node(&mut self) -> (NodeRef, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Node(node), start, end) => (node, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn push_nullable(&mut self, nullable: Option<NodeRef>, start: Location, end: Location) {
        self.stack
            .push((StackValue::Nullable(nullable), start, end));
    }

    fn pop_nullable(&mut self) -> (Option<NodeRef>, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Nullable(nullable), start, end) => (nullable, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn push_list(&mut self, list: Vec<NodeRef>, start: Location, end: Location) {
        self.stack.push((StackValue::List(list), start, end));
    }

    fn pop_list(&mut self) -> (Vec<NodeRef>, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::List(list), start, end) => (list, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn push_array(&mut self, array: Vec<Option<NodeRef>>, start: Location, end: Location) {
        self.stack.push((StackValue::Array(array), start, end));
    }

    fn pop_array(&mut self) -> (Vec<Option<NodeRef>>, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Array(array), start, end) => (array, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn push_either(&mut self, left: NodeRef, right: NodeRef, start: Location, end: Location) {
        self.stack
            .push((StackValue::Either(left, right), start, end));
    }

    fn pop_left(&mut self) -> (NodeRef, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Either(left, _), start, end) => (left, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }

    fn pop_right(&mut self) -> (NodeRef, Location, Location) {
        match self.stack.pop().unwrap() {
            (StackValue::Either(_, right), start, end) => (right, start, end),
            (value, ..) => panic!("{value:?}"),
        }
    }
}

impl SyntaxHandler for Builder {
    type Artifact = NodeRef;

    fn start(&mut self) {
        tracing::debug!(op = "start");
    }

    fn accept(&mut self) -> Self::Artifact {
        tracing::debug!(op = "accept");
        assert_eq!(self.stack.len(), 1);
        let (node, ..) = self.pop_node();
        // The program may have trailing whitespaces.
        //node["end"] = json!(self.location.offset);
        node
    }

    fn shift<'a>(&mut self, token: &Token<'a>) {
        let start = self.location.clone();
        let end = token.compute_end(&start);
        tracing::debug!(
            op = "shift",
            ?token.kind,
            auto=token.inserted_automatically(),
            %start,
            %end,
        );
        self.push_token(token.lexeme.to_owned(), start, end);
    }

    fn reduce(&mut self, rule: ProductionRule) {
        match ACTIONS[rule.id() as usize] {
            Some((action, name)) => {
                tracing::debug!(op = "reduce", action = name, %rule);
                action(self);
            }
            None => tracing::error!("No action defined for: {rule}"),
        }
    }

    fn error(&mut self) {
        tracing::error!("error");
    }

    fn location(&mut self, location: &Location) {
        tracing::debug!(op = "location", %location);
        self.location = location.clone();
    }
}

const TEMPLATE_START_LEN: usize = 1; // "`"
const TEMPLATE_END_LEN: usize = 1; // "`"
const TEMPLATE_SUBST_START_LEN: usize = 2; // "${"
const TEMPLATE_SUBST_END_LEN: usize = 1; // "}"
