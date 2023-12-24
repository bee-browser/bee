use std::ops::Deref;
use std::rc::Rc;

use serde::Serialize;

use bee_jsparser::literal_content_to_string;
use bee_jsparser::string_literal_to_string;
use bee_jsparser::Location;

#[derive(Clone, Debug, Serialize)]
pub struct NodeRef(Rc<Node>);

impl NodeRef {
    #[inline(always)]
    fn new(node: Node) -> Self {
        Self(Rc::new(node))
    }
}

impl Deref for NodeRef {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// The name of each variant must be the same as the corresponding ESTree AST variant type.
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Node {
    Identifier(Identifier),
    PrivateIdentifier(Identifier),
    Literal(Literal),
    Program(Program),
    // statements
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
    EmptyStatement(EmptyStatement),
    DebuggerStatement(DebuggerStatement),
    WithStatement(WithStatement),
    ReturnStatement(ReturnStatement),
    LabeledStatement(LabeledStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    IfStatement(IfStatement),
    SwitchStatement(SwitchStatement),
    SwitchCase(SwitchCase),
    ThrowStatement(ThrowStatement),
    TryStatement(TryStatement),
    CatchClause(CatchClause),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ForStatement(ForStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    // declarations
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    VariableDeclarator(VariableDeclarator),
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ExportAllDeclaration(ExportAllDeclaration),
    // expressions
    ThisExpression(ThisExpression),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
    Property(Property),
    FunctionExpression(FunctionExpression),
    UnaryExpression(UnaryExpression),
    UpdateExpression(UpdateExpression),
    BinaryExpression(BinaryExpression),
    AssignmentExpression(AssignmentExpression),
    LogicalExpression(LogicalExpression),
    MemberExpression(MemberExpression),
    ConditionalExpression(ConditionalExpression),
    CallExpression(CallExpression),
    NewExpression(NewExpression),
    SequenceExpression(SequenceExpression),
    ArrowFunctionExpression(ArrowFunctionExpression),
    YieldExpression(YieldExpression),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplateExpression(TaggedTemplateExpression),
    ClassExpression(ClassExpression),
    MetaProperty(MetaProperty),
    AwaitExpression(AwaitExpression),
    ImportExpression(ImportExpression),
    ChainExpression(ChainExpression),
    // patterns
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
    RestElement(RestElement),
    AssignmentPattern(AssignmentPattern),
    // others
    Super(Super),
    SpreadElement(SpreadElement),
    TemplateElement(TemplateElement),
    ClassBody(ClassBody),
    StaticBlock(StaticBlock),
    PropertyDefinition(PropertyDefinition),
    MethodDefinition(MethodDefinition),
    ImportSpecifier(ImportSpecifier),
    ImportDefaultSpecifier(ImportDefaultSpecifier),
    ImportNamespaceSpecifier(ImportNamespaceSpecifier),
    ExportSpecifier(ExportSpecifier),
    // internals
    #[serde(skip)]
    ClassTail(ClassTail),
    #[serde(skip)]
    ComputedPropertyName(NodeRef),
    #[serde(skip)]
    OptionalCall((Vec<NodeRef>, Location)),
    #[serde(skip)]
    OptionalMember((NodeRef, bool, Location)),
    #[serde(skip)]
    CoverInitializedName(CoverInitializedName),
}

impl Node {
    pub fn identifier(start: &Location, end: &Location, name: String) -> NodeRef {
        NodeRef::new(Self::Identifier(Identifier::new(start, end, name)))
    }

    // literals

    pub fn null(start: &Location, end: &Location) -> NodeRef {
        NodeRef::new(Self::Literal(Literal::null(start, end)))
    }

    pub fn boolean(start: &Location, end: &Location, value: bool) -> NodeRef {
        NodeRef::new(Self::Literal(Literal::boolean(start, end, value)))
    }

    pub fn number(start: &Location, end: &Location, raw: String) -> NodeRef {
        NodeRef::new(Self::Literal(Literal::number(start, end, raw)))
    }

    pub fn string(start: &Location, end: &Location, raw: String) -> NodeRef {
        NodeRef::new(Self::Literal(Literal::string(start, end, raw)))
    }

    pub fn regexp(start: &Location, end: &Location, raw: String) -> NodeRef {
        NodeRef::new(Self::Literal(Literal::regexp(start, end, raw)))
    }

    pub fn program(
        start: &Location,
        end: &Location,
        body: Vec<NodeRef>,
        source_type: SourceType,
    ) -> NodeRef {
        let body = Self::into_statement_list_with_directive_prologue(body);
        NodeRef::new(Self::Program(Program::new(start, end, body, source_type)))
    }

    // statements

    pub fn expression_statement(start: &Location, end: &Location, expression: NodeRef) -> NodeRef {
        NodeRef::new(Self::ExpressionStatement(ExpressionStatement::new(
            start, end, expression,
        )))
    }

    pub fn function_body(start: &Location, end: &Location, body: Vec<NodeRef>) -> NodeRef {
        let body = Self::into_statement_list_with_directive_prologue(body);
        NodeRef::new(Self::BlockStatement(BlockStatement::new(start, end, body)))
    }

    pub fn block_statement(start: &Location, end: &Location, body: Vec<NodeRef>) -> NodeRef {
        NodeRef::new(Self::BlockStatement(BlockStatement::new(start, end, body)))
    }

    pub fn empty_statement(start: &Location, end: &Location) -> NodeRef {
        NodeRef::new(Self::EmptyStatement(EmptyStatement::new(start, end)))
    }

    pub fn debugger_statement(start: &Location, end: &Location) -> NodeRef {
        NodeRef::new(Self::DebuggerStatement(DebuggerStatement::new(start, end)))
    }

    pub fn with_statement(
        start: &Location,
        end: &Location,
        object: NodeRef,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::WithStatement(WithStatement::new(
            start, end, object, body,
        )))
    }

    pub fn return_statement(
        start: &Location,
        end: &Location,
        argument: Option<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::ReturnStatement(ReturnStatement::new(
            start, end, argument,
        )))
    }

    pub fn labeled_statement(
        start: &Location,
        end: &Location,
        label: NodeRef,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::LabeledStatement(LabeledStatement::new(
            start, end, label, body,
        )))
    }

    pub fn break_statement(start: &Location, end: &Location, label: Option<NodeRef>) -> NodeRef {
        NodeRef::new(Self::BreakStatement(BreakStatement::new(start, end, label)))
    }

    pub fn continue_statement(start: &Location, end: &Location, label: Option<NodeRef>) -> NodeRef {
        NodeRef::new(Self::ContinueStatement(ContinueStatement::new(
            start, end, label,
        )))
    }

    pub fn if_statement(
        start: &Location,
        end: &Location,
        test: NodeRef,
        consequent: NodeRef,
        alternate: Option<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::IfStatement(IfStatement::new(
            start, end, test, consequent, alternate,
        )))
    }

    pub fn switch_statement(
        start: &Location,
        end: &Location,
        discriminant: NodeRef,
        cases: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::SwitchStatement(SwitchStatement::new(
            start,
            end,
            discriminant,
            cases,
        )))
    }

    pub fn switch_case(
        start: &Location,
        end: &Location,
        test: Option<NodeRef>,
        consequent: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::SwitchCase(SwitchCase::new(
            start, end, test, consequent,
        )))
    }

    pub fn throw_statement(start: &Location, end: &Location, argument: NodeRef) -> NodeRef {
        NodeRef::new(Self::ThrowStatement(ThrowStatement::new(
            start, end, argument,
        )))
    }

    pub fn try_statement(
        start: &Location,
        end: &Location,
        block: NodeRef,
        handler: Option<NodeRef>,
        finalizer: Option<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::TryStatement(TryStatement::new(
            start, end, block, handler, finalizer,
        )))
    }

    pub fn catch_clause(
        start: &Location,
        end: &Location,
        param: Option<NodeRef>,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::CatchClause(CatchClause::new(start, end, param, body)))
    }

    pub fn while_statement(
        start: &Location,
        end: &Location,
        test: NodeRef,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::WhileStatement(WhileStatement::new(
            start, end, test, body,
        )))
    }

    pub fn do_while_statement(
        start: &Location,
        end: &Location,
        test: NodeRef,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::DoWhileStatement(DoWhileStatement::new(
            start, end, test, body,
        )))
    }

    pub fn for_statement(
        start: &Location,
        end: &Location,
        init: Option<NodeRef>,
        test: Option<NodeRef>,
        update: Option<NodeRef>,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ForStatement(ForStatement::new(
            start, end, init, test, update, body,
        )))
    }

    pub fn for_in_statement(
        start: &Location,
        end: &Location,
        left: NodeRef,
        right: NodeRef,
        body: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ForInStatement(ForInStatement::new(
            start, end, left, right, body,
        )))
    }

    pub fn for_of_statement(
        start: &Location,
        end: &Location,
        left: NodeRef,
        right: NodeRef,
        body: NodeRef,
        r#await: bool,
    ) -> NodeRef {
        NodeRef::new(Self::ForOfStatement(ForOfStatement::new(
            start, end, left, right, body, r#await,
        )))
    }

    // declarations

    pub fn function_declaration(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        generator: bool,
        r#async: bool,
    ) -> NodeRef {
        NodeRef::new(Self::FunctionDeclaration(FunctionDeclaration::new(
            start, end, id, params, body, generator, r#async,
        )))
    }

    pub fn variable_declaration(
        start: &Location,
        end: &Location,
        kind: DeclarationKind,
        declarations: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::VariableDeclaration(VariableDeclaration::new(
            start,
            end,
            kind,
            declarations,
        )))
    }

    pub fn variable_declarator(
        start: &Location,
        end: &Location,
        id: NodeRef,
        init: Option<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::VariableDeclarator(VariableDeclarator::new(
            start, end, id, init,
        )))
    }

    pub fn class_declaration(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        class_tail: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ClassDeclaration(ClassDeclaration::new(
            start, end, id, class_tail,
        )))
    }

    pub fn import_declaration(
        start: &Location,
        end: &Location,
        specifiers: Vec<NodeRef>,
        source: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ImportDeclaration(ImportDeclaration::new(
            start, end, specifiers, source,
        )))
    }

    pub fn export_named_declaration(
        start: &Location,
        end: &Location,
        declaration: Option<NodeRef>,
        specifiers: Vec<NodeRef>,
        source: Option<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::ExportNamedDeclaration(ExportNamedDeclaration::new(
            start,
            end,
            declaration,
            specifiers,
            source,
        )))
    }

    pub fn export_default_declaration(
        start: &Location,
        end: &Location,
        declaration: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ExportDefaultDeclaration(
            ExportDefaultDeclaration::new(start, end, declaration),
        ))
    }

    pub fn export_all_declaration(
        start: &Location,
        end: &Location,
        exported: Option<NodeRef>,
        source: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ExportAllDeclaration(ExportAllDeclaration::new(
            start, end, exported, source,
        )))
    }

    // expressions

    pub fn this_expression(start: &Location, end: &Location) -> NodeRef {
        NodeRef::new(Self::ThisExpression(ThisExpression::new(start, end)))
    }

    pub fn array_expression(
        start: &Location,
        end: &Location,
        elements: Vec<Option<NodeRef>>,
        trailing_comma: bool,
    ) -> NodeRef {
        NodeRef::new(Self::ArrayExpression(ArrayExpression::new(
            start, end, elements, trailing_comma,
        )))
    }

    pub fn object_expression(
        start: &Location,
        end: &Location,
        properties: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::ObjectExpression(ObjectExpression::new(
            start, end, properties,
        )))
    }

    pub fn property(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: NodeRef,
        kind: PropertyKind,
        shorthand: bool,
    ) -> NodeRef {
        NodeRef::new(Self::Property(Property::new(
            start, end, key, value, kind, shorthand,
        )))
    }

    pub fn function_expression(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        generator: bool,
        r#async: bool,
    ) -> NodeRef {
        NodeRef::new(Self::FunctionExpression(FunctionExpression::new(
            start, end, id, params, body, generator, r#async,
        )))
    }

    pub fn unary_expression(
        start: &Location,
        end: &Location,
        operator: &str,
        argument: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::UnaryExpression(UnaryExpression::new(
            start, end, operator, argument,
        )))
    }

    pub fn update_expression(
        start: &Location,
        end: &Location,
        operator: &str,
        argument: NodeRef,
        prefix: bool,
    ) -> NodeRef {
        NodeRef::new(Self::UpdateExpression(UpdateExpression::new(
            start, end, operator, argument, prefix,
        )))
    }

    pub fn binary_expression(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::BinaryExpression(BinaryExpression::new(
            start, end, operator, left, right,
        )))
    }

    pub fn assignment_expression(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::AssignmentExpression(AssignmentExpression::new(
            start, end, operator, left, right,
        )))
    }

    pub fn logical_expression(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::LogicalExpression(LogicalExpression::new(
            start, end, operator, left, right,
        )))
    }

    pub fn member_expression(
        start: &Location,
        end: &Location,
        object: NodeRef,
        property: NodeRef,
        computed: bool,
        optional: bool,
    ) -> NodeRef {
        NodeRef::new(Self::MemberExpression(MemberExpression::new(
            start, end, object, property, computed, optional,
        )))
    }

    pub fn conditional_expression(
        start: &Location,
        end: &Location,
        test: NodeRef,
        consequent: NodeRef,
        alternate: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ConditionalExpression(ConditionalExpression::new(
            start, end, test, consequent, alternate,
        )))
    }

    pub fn call_expression(
        start: &Location,
        end: &Location,
        callee: NodeRef,
        arguments: Vec<NodeRef>,
        optional: bool,
    ) -> NodeRef {
        NodeRef::new(Self::CallExpression(CallExpression::new(
            start, end, callee, arguments, optional,
        )))
    }

    pub fn new_expression(
        start: &Location,
        end: &Location,
        callee: NodeRef,
        arguments: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::NewExpression(NewExpression::new(
            start, end, callee, arguments,
        )))
    }

    pub fn sequence_expression(
        start: &Location,
        end: &Location,
        expressions: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::SequenceExpression(SequenceExpression::new(
            start,
            end,
            expressions,
        )))
    }

    pub fn arrow_function_expression(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        r#async: bool,
    ) -> NodeRef {
        NodeRef::new(Self::ArrowFunctionExpression(ArrowFunctionExpression::new(
            start, end, id, params, body, r#async,
        )))
    }

    pub fn yield_expression(
        start: &Location,
        end: &Location,
        argument: Option<NodeRef>,
        delegate: bool,
    ) -> NodeRef {
        NodeRef::new(Self::YieldExpression(YieldExpression::new(
            start, end, argument, delegate,
        )))
    }

    pub fn template_literal(
        start: &Location,
        end: &Location,
        quasis: Vec<NodeRef>,
        expressions: Vec<NodeRef>,
    ) -> NodeRef {
        NodeRef::new(Self::TemplateLiteral(TemplateLiteral::new(
            start,
            end,
            quasis,
            expressions,
        )))
    }

    pub fn tagged_template_expression(
        start: &Location,
        end: &Location,
        tag: NodeRef,
        quasi: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::TaggedTemplateExpression(
            TaggedTemplateExpression::new(start, end, tag, quasi),
        ))
    }

    pub fn class_expression(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        class_tail: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ClassExpression(ClassExpression::new(
            start, end, id, class_tail,
        )))
    }

    pub fn meta_property(
        start: &Location,
        end: &Location,
        meta: NodeRef,
        property: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::MetaProperty(MetaProperty::new(
            start, end, meta, property,
        )))
    }

    pub fn await_expression(start: &Location, end: &Location, argument: NodeRef) -> NodeRef {
        NodeRef::new(Self::AwaitExpression(AwaitExpression::new(
            start, end, argument,
        )))
    }

    pub fn import_expression(start: &Location, end: &Location, source: NodeRef) -> NodeRef {
        NodeRef::new(Self::ImportExpression(ImportExpression::new(
            start, end, source,
        )))
    }

    pub fn chain_expression(
        start: &Location,
        end: &Location,
        expr: NodeRef,
        chains: Vec<NodeRef>,
    ) -> NodeRef {
        let expr = match *expr {
            Node::ChainExpression(ref expr) => expr.expression.clone(),
            _ => expr,
        };
        let mut optional = true;
        let expr = chains.into_iter().fold(expr, |expr, chain| {
            let expr = match *chain {
                Node::OptionalCall((ref arguments, ref chain_end)) => {
                    Self::call_expression(start, chain_end, expr, arguments.clone(), optional)
                }
                Node::OptionalMember((ref property, computed, ref chain_end)) => {
                    Self::member_expression(
                        start,
                        chain_end,
                        expr,
                        property.clone(),
                        computed,
                        optional,
                    )
                }
                _ => panic!(),
            };
            optional = false;
            expr
        });
        NodeRef::new(Self::ChainExpression(ChainExpression::new(
            start, end, expr,
        )))
    }

    pub fn object_pattern(start: &Location, end: &Location, properties: Vec<NodeRef>) -> NodeRef {
        NodeRef::new(Self::ObjectPattern(ObjectPattern::new(
            start, end, properties,
        )))
    }

    pub fn array_pattern(
        start: &Location,
        end: &Location,
        elements: Vec<Option<NodeRef>>,
    ) -> NodeRef {
        NodeRef::new(Self::ArrayPattern(ArrayPattern::new(start, end, elements)))
    }

    pub fn rest_element(start: &Location, end: &Location, argument: NodeRef) -> NodeRef {
        NodeRef::new(Self::RestElement(RestElement::new(start, end, argument)))
    }

    pub fn assignment_pattern(
        start: &Location,
        end: &Location,
        left: NodeRef,
        right: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::AssignmentPattern(AssignmentPattern::new(
            start, end, left, right,
        )))
    }

    pub fn super_(start: &Location, end: &Location) -> NodeRef {
        NodeRef::new(Self::Super(Super::new(start, end)))
    }

    pub fn spread_element(start: &Location, end: &Location, argument: NodeRef) -> NodeRef {
        NodeRef::new(Self::SpreadElement(SpreadElement::new(
            start, end, argument,
        )))
    }

    pub fn template_element(start: &Location, end: &Location, raw: &str, tail: bool) -> NodeRef {
        NodeRef::new(Self::TemplateElement(TemplateElement::new(
            start, end, raw, tail,
        )))
    }

    pub fn class_body(start: &Location, end: &Location, body: Vec<NodeRef>) -> NodeRef {
        NodeRef::new(Self::ClassBody(ClassBody::new(start, end, body)))
    }

    pub fn static_block(start: &Location, end: &Location, body: Vec<NodeRef>) -> NodeRef {
        NodeRef::new(Self::StaticBlock(StaticBlock::new(start, end, body)))
    }

    pub fn property_definition(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: Option<NodeRef>,
    ) -> NodeRef {
        let (key, computed) = match *key {
            Self::ComputedPropertyName(ref key) => (key.clone(), true),
            _ => (key, false),
        };
        NodeRef::new(Self::PropertyDefinition(PropertyDefinition::new(
            start, end, key, value, computed, false,
        )))
    }

    pub fn property_definition_update(
        start: &Location,
        end: &Location,
        def: NodeRef,
        r#static: bool,
    ) -> NodeRef {
        let def = match *def {
            Self::PropertyDefinition(ref def) => def,
            _ => panic!(),
        };
        NodeRef::new(Self::PropertyDefinition(PropertyDefinition::new(
            start,
            end,
            def.key.clone(),
            def.value.clone(),
            def.computed,
            r#static,
        )))
    }

    pub fn method_definition(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: NodeRef,
    ) -> NodeRef {
        let (key, kind, computed) = match *key {
            Self::ComputedPropertyName(ref key) => (key.clone(), MethodKind::Method, true),
            Self::Identifier(ref id) if id.name == "constructor" => {
                (key, MethodKind::Constructor, false)
            }
            _ => (key, MethodKind::Method, false),
        };
        NodeRef::new(Self::MethodDefinition(MethodDefinition::new(
            start, end, key, value, kind, computed, false,
        )))
    }

    pub fn static_method_definition(start: &Location, end: &Location, method: NodeRef) -> NodeRef {
        let method = match *method {
            Self::MethodDefinition(ref method) => method,
            _ => panic!(),
        };
        let kind = match method.kind {
            MethodKind::Constructor => MethodKind::Method,
            kind => kind,
        };
        NodeRef::new(Self::MethodDefinition(MethodDefinition::new(
            start,
            end,
            method.key.clone(),
            method.value.clone(),
            kind,
            method.computed,
            true,
        )))
    }

    pub fn getter(start: &Location, end: &Location, key: NodeRef, value: NodeRef) -> NodeRef {
        let (key, computed) = match *key {
            Self::ComputedPropertyName(ref key) => (key.clone(), true),
            _ => (key, false),
        };
        NodeRef::new(Self::MethodDefinition(MethodDefinition::new(
            start,
            end,
            key,
            value,
            MethodKind::Get,
            computed,
            false,
        )))
    }

    pub fn setter(start: &Location, end: &Location, key: NodeRef, value: NodeRef) -> NodeRef {
        let (key, computed) = match *key {
            Self::ComputedPropertyName(ref key) => (key.clone(), true),
            _ => (key, false),
        };
        NodeRef::new(Self::MethodDefinition(MethodDefinition::new(
            start,
            end,
            key,
            value,
            MethodKind::Set,
            computed,
            false,
        )))
    }

    pub fn import_specifier(
        start: &Location,
        end: &Location,
        imported: NodeRef,
        local: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ImportSpecifier(ImportSpecifier::new(
            start, end, imported, local,
        )))
    }

    pub fn import_default_specifier(start: &Location, end: &Location, local: NodeRef) -> NodeRef {
        NodeRef::new(Self::ImportDefaultSpecifier(ImportDefaultSpecifier::new(
            start, end, local,
        )))
    }

    pub fn import_namespace_specifier(start: &Location, end: &Location, local: NodeRef) -> NodeRef {
        NodeRef::new(Self::ImportNamespaceSpecifier(
            ImportNamespaceSpecifier::new(start, end, local),
        ))
    }

    pub fn export_specifier(
        start: &Location,
        end: &Location,
        local: NodeRef,
        exported: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::ExportSpecifier(ExportSpecifier::new(
            start, end, local, exported,
        )))
    }

    pub fn private_identifier(start: &Location, end: &Location, name: String) -> NodeRef {
        NodeRef::new(Self::PrivateIdentifier(Identifier::new(start, end, name)))
    }

    // internals

    pub fn class_tail(super_class: Option<NodeRef>, body: NodeRef) -> NodeRef {
        NodeRef::new(Self::ClassTail(ClassTail::new(super_class, body)))
    }

    pub fn computed_property_name(expr: NodeRef) -> NodeRef {
        NodeRef::new(Self::ComputedPropertyName(expr))
    }

    pub fn optional_call(arguments: Vec<NodeRef>, end: Location) -> NodeRef {
        NodeRef::new(Self::OptionalCall((arguments, end)))
    }

    pub fn optional_member(expr: NodeRef, computed: bool, end: Location) -> NodeRef {
        NodeRef::new(Self::OptionalMember((expr, computed, end)))
    }

    pub fn cover_initialized_name(
        start: &Location,
        end: &Location,
        name: NodeRef,
        value: NodeRef,
    ) -> NodeRef {
        NodeRef::new(Self::CoverInitializedName(CoverInitializedName::new(
            start, end, name, value,
        )))
    }

    pub fn for_init_update(init: NodeRef) -> NodeRef {
        match *init {
            Node::VariableDeclaration(ref decl) => {
                let last = decl.declarations.last().unwrap();
                let end = match **last {
                    Node::VariableDeclarator(ref var) => var.location.end_location(),
                    _ => panic!(),
                };
                Self::variable_declaration(
                    &decl.location.start_location(),
                    &end,
                    decl.kind,
                    decl.declarations.clone(),
                )
            }
            _ => panic!(),
        }
    }

    pub fn into_patterns(nullable: Option<NodeRef>) -> Result<Vec<NodeRef>, String> {
        match nullable {
            Some(node) => match *node {
                Node::SequenceExpression(ref seq) => seq
                    .expressions
                    .iter()
                    .cloned()
                    .map(Self::into_pattern)
                    .collect::<Result<Vec<_>, _>>(),
                _ => Ok(vec![Self::into_pattern(node)?]),
            },
            None => Ok(vec![]),
        }
    }

    pub fn into_pattern(node: NodeRef) -> Result<NodeRef, String> {
        match *node {
            Node::ObjectExpression(ref expr) => Self::to_object_pattern(expr),
            Node::ArrayExpression(ref expr) => Self::to_array_pattern(expr),
            Node::AssignmentExpression(ref expr) => Self::to_assignment_pattern(expr),
            Node::SpreadElement(ref expr) => Self::to_rest_element(expr),
            Node::Property(ref property) => Self::to_assignment_property(property),
            Node::CoverInitializedName(ref cover) => {
                let start = cover.location.start_location();
                let end = cover.location.end_location();
                Ok(Self::assignment_pattern(&start, &end, cover.name.clone(), cover.value.clone()))
            }
            _ => Ok(node),
        }
    }

    pub fn into_property(node: NodeRef) -> NodeRef {
        match *node {
            Node::MethodDefinition(ref method) => {
                let start = method.location.start_location();
                let end = method.location.end_location();
                NodeRef::new(Self::Property(Property {
                    location: LocationData::new(&start, &end),
                    key: method.key.clone(),
                    value: method.value.clone(),
                    kind: match method.kind {
                        MethodKind::Get => PropertyKind::Get,
                        MethodKind::Set => PropertyKind::Set,
                        _ => PropertyKind::Init,
                    },
                    method: match method.kind {
                        MethodKind::Get => false,
                        MethodKind::Set => false,
                        _ => true,
                    },
                    shorthand: false,
                    computed: method.computed,
                }))
            }
            _ => panic!("{node:?}"),
        }
    }

    fn to_object_pattern(expr: &ObjectExpression) -> Result<NodeRef, String> {
        let start = expr.location.start_location();
        let end = expr.location.end_location();
        let properties = expr
            .properties
            .iter()
            .cloned()
            .map(Self::into_pattern)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::object_pattern(&start, &end, properties))
    }

    fn to_array_pattern(expr: &ArrayExpression) -> Result<NodeRef, String> {
        let start = expr.location.start_location();
        let end = expr.location.end_location();
        let mut elements = vec![];
        let mut rest_found = false;
        for element in expr.elements.iter() {
            match element {
                Some(node) => {
                    let node = Self::into_pattern(node.clone())?;
                    if let Node::RestElement(_) = *node {
                        if rest_found {
                            return Err("Multiple RestElements are not allowed in ArrayAssignmentPattern".to_string());
                        }
                        if expr.trailing_comma {
                            return Err("Trailing comma is not allowed in ArrayAssignmentPattern".to_string());
                        }
                        rest_found = true;
                    }
                    elements.push(Some(node));
                }
                None => {
                    if rest_found {
                        return Err("Trailing comma is not allowed in ArrayAssignmentPattern".to_string());
                    }
                    elements.push(None)
                }
            }
        }
        Ok(Self::array_pattern(&start, &end, elements))
    }

    fn to_assignment_pattern(expr: &AssignmentExpression) -> Result<NodeRef, String> {
        let start = expr.location.start_location();
        let end = expr.location.end_location();
        Ok(Self::assignment_pattern(&start, &end, expr.left.clone(), expr.right.clone()))
    }

    fn to_rest_element(expr: &SpreadElement) -> Result<NodeRef, String> {
        let start = expr.location.start_location();
        let end = expr.location.end_location();
        let argument = Self::into_pattern(expr.argument.clone())?;
        Ok(Self::rest_element(&start, &end, argument))
    }

    fn to_assignment_property(property: &Property) -> Result<NodeRef, String> {
        let start = property.location.start_location();
        let end = property.location.end_location();
        let value = Self::into_pattern(property.value.clone())?;
        let shorthand = property.shorthand
            || match *value {
                Node::AssignmentPattern(_) => true,
                _ => false,
            };
        Ok(Self::property(
            &start,
            &end,
            property.key.clone(),
            value,
            property.kind,
            shorthand,
        ))
    }

    fn into_statement_list_with_directive_prologue(mut list: Vec<NodeRef>) -> Vec<NodeRef> {
        for node in list.iter_mut() {
            match node.0.as_ref() {
                Node::ExpressionStatement(ref stmt) if stmt.is_likely_directive() => {
                    *node = NodeRef::new(Node::ExpressionStatement(stmt.to_directive()));
                }
                _ => break,
            }
        }
        list
    }

    // validation

    pub fn validate_expression(&self) -> Result<(), String> {
        match *self {
            Node::ArrayExpression(ref expr) => expr.validate(),
            Node::ObjectExpression(ref expr) => expr.validate(),
            Node::Property(ref prop) => prop.validate(),
            Node::FunctionExpression(ref expr) => expr.validate(),
            Node::UnaryExpression(ref expr) => expr.validate(),
            Node::UpdateExpression(ref expr) => expr.validate(),
            Node::BinaryExpression(ref expr) => expr.validate(),
            Node::AssignmentExpression(ref expr) => expr.validate(),
            Node::LogicalExpression(ref expr) => expr.validate(),
            Node::MemberExpression(ref expr) => expr.validate(),
            Node::ConditionalExpression(ref expr) => expr.validate(),
            Node::CallExpression(ref expr) => expr.validate(),
            Node::NewExpression(ref expr) => expr.validate(),
            Node::SequenceExpression(ref expr) => expr.validate(),
            Node::ArrowFunctionExpression(ref expr) => expr.validate(),
            Node::YieldExpression(ref expr) => expr.validate(),
            Node::TemplateLiteral(ref expr) => expr.validate(),
            Node::TaggedTemplateExpression(ref expr) => expr.validate(),
            Node::AwaitExpression(ref expr) => expr.validate(),
            Node::ImportExpression(ref expr) => expr.validate(),
            Node::ChainExpression(ref expr) => expr.validate(),
            Node::SpreadElement(ref elem) => elem.validate(),
            // 13.2.5.1 Static Semantics: Early Errors
            // CoverInitializedName is not allowed in ObjectLiteral
            Node::CoverInitializedName(_) => Err("Early error: CoverInitializedName".to_string()),
            _ => Ok(()),
        }
    }

    pub fn validate_primary_expression(&self) -> Result<(), String> {
        match *self {
            Node::ObjectExpression(ref expr) => expr.validate(),
            Node::ArrayExpression(ref expr) => expr.validate(),
            _ => Ok(()),
        }
    }

    // TODO: implement properly
    pub fn validate_pattern(&self) -> Result<(), String> {
        match *self {
            Node::SequenceExpression(_) => {
                Err("LeftHandSideExpression must cover an AssignmentPattern".to_string())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LocationData {
    // Node
    pub start: usize,
    pub end: usize,
    #[serde(skip)]
    pub loc: SourceLocation,
}

impl LocationData {
    fn new(start: &Location, end: &Location) -> Self {
        Self {
            start: start.offset,
            end: end.offset,
            loc: SourceLocation {
                start: Position {
                    line: start.line,
                    column: start.column,
                },
                end: Position {
                    line: end.line,
                    column: end.column,
                },
            },
        }
    }

    fn start_location(&self) -> Location {
        Location {
            offset: self.start,
            line: self.loc.start.line,
            column: self.loc.start.column,
        }
    }

    fn end_location(&self) -> Location {
        Location {
            offset: self.end,
            line: self.loc.end.line,
            column: self.loc.end.column,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone, Debug, Serialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize)]
pub struct Identifier {
    #[serde(flatten)]
    pub location: LocationData,
    pub name: String,
}

impl Identifier {
    fn new(start: &Location, end: &Location, name: String) -> Self {
        Self {
            location: LocationData::new(start, end),
            name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Literal {
    #[serde(flatten)]
    pub location: LocationData,
    pub value: Scalar,
    pub raw: RawString,
    // RegExp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<RegExp>,
    // BigInt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bigint: Option<String>,
}

impl Literal {
    fn null(start: &Location, end: &Location) -> Self {
        Self {
            location: LocationData::new(start, end),
            value: Scalar::Null,
            raw: RawString::Static("null"),
            regex: None,
            bigint: None,
        }
    }

    fn boolean(start: &Location, end: &Location, value: bool) -> Self {
        Self {
            location: LocationData::new(start, end),
            value: Scalar::Boolean(value),
            raw: RawString::Static(if value { "true" } else { "false" }),
            regex: None,
            bigint: None,
        }
    }

    fn number(start: &Location, end: &Location, raw: String) -> Self {
        let value = numeric_literal_to_scalar(&raw);
        let bigint = if raw.ends_with('n') {
            Some(raw[0..(raw.len() - 1)].to_owned())
        } else {
            None
        };
        Self {
            location: LocationData::new(start, end),
            value,
            raw: RawString::Dynamic(raw),
            regex: None,
            bigint,
        }
    }

    fn string(start: &Location, end: &Location, raw: String) -> Self {
        Self {
            location: LocationData::new(start, end),
            value: Scalar::String(string_literal_to_string(&raw)),
            raw: RawString::Dynamic(raw),
            regex: None,
            bigint: None,
        }
    }

    fn regexp(start: &Location, end: &Location, raw: String) -> Self {
        let (pattern, flags) = match raw[1..].rsplit_once('/') {
            Some((pattern, flags)) => (pattern.to_owned(), flags.to_owned()),
            _ => unreachable!(),
        };
        Self {
            location: LocationData::new(start, end),
            value: Scalar::EmptyObject {},
            raw: RawString::Dynamic(raw),
            regex: Some(RegExp { pattern, flags }),
            bigint: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Program {
    #[serde(flatten)]
    pub location: LocationData,
    pub body: Vec<NodeRef>, // [ Statement | ImportOrExportDeclaration ]
    #[serde(rename = "sourceType")]
    pub source_type: SourceType,
}

impl Program {
    fn new(start: &Location, end: &Location, body: Vec<NodeRef>, source_type: SourceType) -> Self {
        Self {
            location: LocationData::new(start, end),
            body,
            source_type,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, Serialize)]
pub struct Function {
    pub id: Option<NodeRef>,  // Identifier | null
    pub params: Vec<NodeRef>, // [ Pattern ]
    pub body: NodeRef,        // FunctionBody | Expression
    // Deprecated
    pub expression: bool,
    // ES2015
    pub generator: bool,
    // ES2017
    pub r#async: bool,
}

impl Function {
    fn new(
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        generator: bool,
        r#async: bool,
    ) -> Self {
        let expression = match *body {
            Node::BlockStatement(_) => false,
            _ => true,
        };
        Self {
            id,
            params,
            body,
            expression,
            generator,
            r#async,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.body.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct Class {
    pub id: Option<NodeRef>, // Identifier | null
    #[serde(rename = "superClass")]
    pub super_class: Option<NodeRef>, // Expression | null
    pub body: NodeRef,       // ClassBody
}

impl Class {
    fn new(id: Option<NodeRef>, class_tail: NodeRef) -> Self {
        let (super_class, body) = match *class_tail {
            Node::ClassTail(ref class_tail) => {
                (class_tail.super_class.clone(), class_tail.body.clone())
            }
            _ => panic!(),
        };
        Self {
            id,
            super_class,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ClassBody {
    #[serde(flatten)]
    pub location: LocationData,
    pub body: Vec<NodeRef>, // [ MethodDefinition | PropertyDefinition | StaticBlock ]
}

impl ClassBody {
    fn new(start: &Location, end: &Location, body: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            body,
        }
    }
}

#[derive(Debug)]
pub struct ClassTail {
    super_class: Option<NodeRef>,
    body: NodeRef,
}

impl ClassTail {
    fn new(super_class: Option<NodeRef>, body: NodeRef) -> Self {
        Self { super_class, body }
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
pub enum MethodKind {
    #[serde(rename = "constructor")]
    Constructor,
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "set")]
    Set,
}

#[derive(Debug, Serialize)]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub expression: NodeRef, // Expression
    // Directive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,
}

impl ExpressionStatement {
    fn new(start: &Location, end: &Location, expression: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            expression,
            directive: None,
        }
    }

    fn is_likely_directive(&self) -> bool {
        match *self.expression {
            Node::Literal(Literal {
                location: LocationData { start, .. },
                value: Scalar::String(_),
                ..
            }) if self.location.start == start => true,
            _ => false,
        }
    }

    fn to_directive(&self) -> Self {
        let directive = match *self.expression {
            Node::Literal(Literal {
                location: LocationData { start, .. },
                value: Scalar::String(_),
                ref raw,
                ..
            }) if self.location.start == start => Some(raw[1..(raw.len() - 1)].to_owned()),
            _ => panic!(),
        };
        Self {
            location: self.location.clone(),
            expression: self.expression.clone(),
            directive,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BlockStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub body: Vec<NodeRef>, // [Statement]
}

impl BlockStatement {
    fn new(start: &Location, end: &Location, body: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub location: LocationData,
}

impl EmptyStatement {
    fn new(start: &Location, end: &Location) -> Self {
        Self {
            location: LocationData::new(start, end),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub location: LocationData,
}

impl DebuggerStatement {
    fn new(start: &Location, end: &Location) -> Self {
        Self {
            location: LocationData::new(start, end),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WithStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub object: NodeRef, // Expression
    pub body: NodeRef,   // Statement
}

impl WithStatement {
    fn new(start: &Location, end: &Location, object: NodeRef, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            object,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: Option<NodeRef>, // Expression | null
}

impl ReturnStatement {
    fn new(start: &Location, end: &Location, argument: Option<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub label: NodeRef, // Identifier
    pub body: NodeRef,  // Statement
}

impl LabeledStatement {
    fn new(start: &Location, end: &Location, label: NodeRef, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            label,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BreakStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub label: Option<NodeRef>, // Identifier | null
}

impl BreakStatement {
    fn new(start: &Location, end: &Location, label: Option<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            label,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContinueStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub label: Option<NodeRef>, // Identifier | null
}

impl ContinueStatement {
    fn new(start: &Location, end: &Location, label: Option<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            label,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IfStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub test: NodeRef,              // Expression
    pub consequent: NodeRef,        // Statement
    pub alternate: Option<NodeRef>, // Statement | null
}

impl IfStatement {
    fn new(
        start: &Location,
        end: &Location,
        test: NodeRef,
        consequent: NodeRef,
        alternate: Option<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            test,
            consequent,
            alternate,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub discriminant: NodeRef, // Expression
    pub cases: Vec<NodeRef>,   // [ SwitchCase ]
}

impl SwitchStatement {
    fn new(start: &Location, end: &Location, discriminant: NodeRef, cases: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            discriminant,
            cases,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SwitchCase {
    #[serde(flatten)]
    pub location: LocationData,
    pub test: Option<NodeRef>,    // Expression | null
    pub consequent: Vec<NodeRef>, // [ Statement ]
}

impl SwitchCase {
    fn new(
        start: &Location,
        end: &Location,
        test: Option<NodeRef>,
        consequent: Vec<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            test,
            consequent,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ThrowStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: NodeRef, // Expression
}

impl ThrowStatement {
    fn new(start: &Location, end: &Location, argument: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TryStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub block: NodeRef,             // BlockStatement
    pub handler: Option<NodeRef>,   // CatchClause | null
    pub finalizer: Option<NodeRef>, // BlockStatement | null
}

impl TryStatement {
    fn new(
        start: &Location,
        end: &Location,
        block: NodeRef,
        handler: Option<NodeRef>,
        finalizer: Option<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            block,
            handler,
            finalizer,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CatchClause {
    #[serde(flatten)]
    pub location: LocationData,
    pub param: Option<NodeRef>, // Pattern | null
    pub body: NodeRef,          // BlockStatement
}

impl CatchClause {
    fn new(start: &Location, end: &Location, param: Option<NodeRef>, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            param,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WhileStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub test: NodeRef, // Expression
    pub body: NodeRef, // Statement
}

impl WhileStatement {
    fn new(start: &Location, end: &Location, test: NodeRef, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            test,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub test: NodeRef, // Expression
    pub body: NodeRef, // Statement
}

impl DoWhileStatement {
    fn new(start: &Location, end: &Location, test: NodeRef, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            test,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ForStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub init: Option<NodeRef>,   // VariableDeclaration | Expression | null
    pub test: Option<NodeRef>,   // Expression | null
    pub update: Option<NodeRef>, // Expression | null
    pub body: NodeRef,           // Statement
}

impl ForStatement {
    fn new(
        start: &Location,
        end: &Location,
        init: Option<NodeRef>,
        test: Option<NodeRef>,
        update: Option<NodeRef>,
        body: NodeRef,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            init,
            test,
            update,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ForInStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub left: NodeRef,  // VariableDeclaration | Pattern
    pub right: NodeRef, // Expression
    pub body: NodeRef,  // Statement
}

impl ForInStatement {
    fn new(start: &Location, end: &Location, left: NodeRef, right: NodeRef, body: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            left,
            right,
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub location: LocationData,
    pub left: NodeRef,  // VariableDeclaration | Pattern
    pub right: NodeRef, // Expression
    pub body: NodeRef,  // Statement
    pub r#await: bool,
}

impl ForOfStatement {
    fn new(
        start: &Location,
        end: &Location,
        left: NodeRef,
        right: NodeRef,
        body: NodeRef,
        r#await: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            left,
            right,
            body,
            r#await,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    #[serde(flatten)]
    pub function: Function,
}

impl FunctionDeclaration {
    fn new(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        generator: bool,
        r#async: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            function: Function::new(id, params, body, generator, r#async),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    pub declarations: Vec<NodeRef>, // [ VariableDeclarator ]
    pub kind: DeclarationKind,
}

impl VariableDeclaration {
    fn new(
        start: &Location,
        end: &Location,
        kind: DeclarationKind,
        declarations: Vec<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            declarations,
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum DeclarationKind {
    #[serde(rename = "var")]
    Var,
    #[serde(rename = "let")]
    Let,
    #[serde(rename = "const")]
    Const,
}

impl std::str::FromStr for DeclarationKind {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "var" => Self::Var,
            "let" => Self::Let,
            "const" => Self::Const,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub location: LocationData,
    pub id: NodeRef,           // Pattern
    pub init: Option<NodeRef>, // Expression | null
}

impl VariableDeclarator {
    fn new(start: &Location, end: &Location, id: NodeRef, init: Option<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            id,
            init,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    #[serde(flatten)]
    pub class: Class,
}

impl ClassDeclaration {
    fn new(start: &Location, end: &Location, id: Option<NodeRef>, class_tail: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            class: Class::new(id, class_tail),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImportDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    // [ ImportSpecifier | ImportDefaultSpecifier | ImportNamespaceSpecifier ]
    pub specifiers: Vec<NodeRef>, // [ ImportSpecifier ]
    pub source: NodeRef,          // Literal
}

impl ImportDeclaration {
    fn new(start: &Location, end: &Location, specifiers: Vec<NodeRef>, source: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            specifiers,
            source,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExportNamedDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    pub declaration: Option<NodeRef>, // Declaration | null
    pub specifiers: Vec<NodeRef>,     // [ ExportSpecifier ]
    pub source: Option<NodeRef>,      // Literal | null
}

impl ExportNamedDeclaration {
    fn new(
        start: &Location,
        end: &Location,
        declaration: Option<NodeRef>,
        specifiers: Vec<NodeRef>,
        source: Option<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            declaration,
            specifiers,
            source,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExportDefaultDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    // AnonymousDefaultExportedFunctionDeclaration | FunctionDeclaration
    //   | AnonymousDefaultExportedClassDeclaration | ClassDeclaration | Expression
    pub declaration: NodeRef,
}

impl ExportDefaultDeclaration {
    fn new(start: &Location, end: &Location, declaration: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            declaration,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ExportAllDeclaration {
    #[serde(flatten)]
    pub location: LocationData,
    pub exported: Option<NodeRef>, // Identifier | null
    pub source: NodeRef,           // Literal
}

impl ExportAllDeclaration {
    fn new(start: &Location, end: &Location, exported: Option<NodeRef>, source: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            exported,
            source,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ThisExpression {
    #[serde(flatten)]
    pub location: LocationData,
}

impl ThisExpression {
    fn new(start: &Location, end: &Location) -> Self {
        Self {
            location: LocationData::new(start, end),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub elements: Vec<Option<NodeRef>>, // [ Expression | SpreadElement | null ]
    #[serde(skip)]
    trailing_comma: bool,
}

impl ArrayExpression {
    fn new(start: &Location, end: &Location, elements: Vec<Option<NodeRef>>, trailing_comma: bool) -> Self {
        Self {
            location: LocationData::new(start, end),
            elements,
            trailing_comma,
        }
    }

    fn validate(&self) -> Result<(), String> {
        for element in self.elements.iter().filter_map(Option::as_ref) {
            element.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub properties: Vec<NodeRef>, // [ Property | SpreadElement ]
}

impl ObjectExpression {
    fn new(start: &Location, end: &Location, properties: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            properties,
        }
    }

    fn validate(&self) -> Result<(), String> {
        for property in self.properties.iter() {
            property.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Property {
    #[serde(flatten)]
    pub location: LocationData,
    pub key: NodeRef,   // Literal | Identifier
    pub value: NodeRef, // Expression | Pattern
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}

impl Property {
    fn new(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: NodeRef,
        kind: PropertyKind,
        shorthand: bool,
    ) -> Self {
        let (key, computed) = match *key {
            Node::ComputedPropertyName(ref expr) => (expr.clone(), true),
            _ => (key.clone(), false),
        };
        let method = match kind {
            PropertyKind::Init => false,
            _ => true,
        };
        Self {
            location: LocationData::new(start, end),
            key,
            value,
            kind,
            method,
            shorthand,
            computed,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.value.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum PropertyKind {
    #[serde(rename = "init")]
    Init,
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "set")]
    Set,
}

#[derive(Debug, Serialize)]
pub struct FunctionExpression {
    #[serde(flatten)]
    pub location: LocationData,
    #[serde(flatten)]
    pub function: Function,
}

impl FunctionExpression {
    fn new(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        generator: bool,
        r#async: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            function: Function::new(id, params, body, generator, r#async),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.function.validate()
    }
}

#[derive(Debug, Serialize)]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub operator: UnaryOperator,
    pub argument: NodeRef, // Expression
    pub prefix: bool,
}

impl UnaryExpression {
    fn new(start: &Location, end: &Location, operator: &str, argument: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            operator: operator.parse().unwrap(),
            argument,
            prefix: true,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.argument.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum UnaryOperator {
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "~")]
    BitwiseNot,
    #[serde(rename = "typeof")]
    Typeof,
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "delete")]
    Delete,
}

impl std::str::FromStr for UnaryOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "-" => Self::Minus,
            "+" => Self::Plus,
            "!" => Self::Not,
            "~" => Self::BitwiseNot,
            "typeof" => Self::Typeof,
            "void" => Self::Void,
            "delete" => Self::Delete,
            _ => panic!("{s}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub operator: UpdateOperator,
    pub argument: NodeRef, // Expression
    pub prefix: bool,
}

impl UpdateExpression {
    fn new(
        start: &Location,
        end: &Location,
        operator: &str,
        argument: NodeRef,
        prefix: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            operator: operator.parse().unwrap(),
            argument,
            prefix,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.argument.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum UpdateOperator {
    #[serde(rename = "++")]
    Increment,
    #[serde(rename = "--")]
    Decrement,
}

impl std::str::FromStr for UpdateOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "++" => Self::Increment,
            "--" => Self::Decrement,
            _ => panic!("{s}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub operator: BinaryOperator,
    pub left: NodeRef,  // Expression | PrivateIdentifier
    pub right: NodeRef, // Expression
}

impl BinaryExpression {
    fn new(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            operator: operator.parse().unwrap(),
            left,
            right,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.right.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum BinaryOperator {
    #[serde(rename = "==")]
    Equality,
    #[serde(rename = "!=")]
    Inequality,
    #[serde(rename = "===")]
    StrictEquality,
    #[serde(rename = "!==")]
    StrictInequality,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<<")]
    LeftShift,
    #[serde(rename = ">>")]
    RightShift,
    #[serde(rename = ">>>")]
    UnsignedRightShift,
    #[serde(rename = "+")]
    Addition,
    #[serde(rename = "-")]
    Subtraction,
    #[serde(rename = "*")]
    Multiplication,
    #[serde(rename = "/")]
    Division,
    #[serde(rename = "%")]
    Remainder,
    #[serde(rename = "|")]
    BitwiseOr,
    #[serde(rename = "^")]
    BitwiseXor,
    #[serde(rename = "&")]
    BitwiseAnd,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "instanceof")]
    Instanceof,
    #[serde(rename = "**")]
    Exponentiation,
}

impl std::str::FromStr for BinaryOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "==" => Self::Equality,
            "!=" => Self::Inequality,
            "===" => Self::StrictEquality,
            "!==" => Self::StrictInequality,
            "<" => Self::LessThan,
            "<=" => Self::LessThanOrEqual,
            ">" => Self::GreaterThan,
            ">=" => Self::GreaterThanOrEqual,
            "<<" => Self::LeftShift,
            ">>" => Self::RightShift,
            ">>>" => Self::UnsignedRightShift,
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "*" => Self::Multiplication,
            "/" => Self::Division,
            "%" => Self::Remainder,
            "|" => Self::BitwiseOr,
            "^" => Self::BitwiseXor,
            "&" => Self::BitwiseAnd,
            "in" => Self::In,
            "instanceof" => Self::Instanceof,
            "**" => Self::Exponentiation,
            _ => panic!("{s}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub operator: AssignmentOperator,
    pub left: NodeRef,  // Pattern
    pub right: NodeRef, // Expression
}

impl AssignmentExpression {
    fn new(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            operator: operator.parse().unwrap(),
            left,
            right,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.right.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Assignment,
    #[serde(rename = "+=")]
    AdditionAssignment,
    #[serde(rename = "-=")]
    SubtractionAssignment,
    #[serde(rename = "*=")]
    MultiplicationAssignment,
    #[serde(rename = "/=")]
    DivisionAssignment,
    #[serde(rename = "%=")]
    RemainderAssignment,
    #[serde(rename = "<<=")]
    LeftShiftAssignment,
    #[serde(rename = ">>=")]
    RightShiftAssignment,
    #[serde(rename = ">>>=")]
    UnsignedRightShiftAssignment,
    #[serde(rename = "|=")]
    BitwiseOrAssignment,
    #[serde(rename = "^=")]
    BitwiseXorAssignment,
    #[serde(rename = "&=")]
    BitwiseAndAssignment,
    #[serde(rename = "||=")]
    LogicalOrAssignment,
    #[serde(rename = "&&=")]
    LogicalAndAssignment,
    #[serde(rename = "**=")]
    ExponentiationAssignment,
    #[serde(rename = "??=")]
    NullishCoalescingAssignment,
}

impl std::str::FromStr for AssignmentOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "=" => Self::Assignment,
            "+=" => Self::AdditionAssignment,
            "-=" => Self::SubtractionAssignment,
            "*=" => Self::MultiplicationAssignment,
            "/=" => Self::DivisionAssignment,
            "%=" => Self::RemainderAssignment,
            "<<=" => Self::LeftShiftAssignment,
            ">>=" => Self::RightShiftAssignment,
            ">>>=" => Self::UnsignedRightShiftAssignment,
            "|=" => Self::BitwiseOrAssignment,
            "^=" => Self::BitwiseXorAssignment,
            "&=" => Self::BitwiseAndAssignment,
            "||=" => Self::LogicalOrAssignment,
            "&&=" => Self::LogicalAndAssignment,
            "**=" => Self::ExponentiationAssignment,
            "??=" => Self::NullishCoalescingAssignment,
            _ => panic!("{s}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub operator: LogicalOperator,
    pub left: NodeRef,  // Expression
    pub right: NodeRef, // Expression
}

impl LogicalExpression {
    fn new(
        start: &Location,
        end: &Location,
        operator: &str,
        left: NodeRef,
        right: NodeRef,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            operator: operator.parse().unwrap(),
            left,
            right,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.left.validate_expression()?;
        self.right.validate_expression()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum LogicalOperator {
    #[serde(rename = "||")]
    LogicalOr,
    #[serde(rename = "&&")]
    LogicalAnd,
    #[serde(rename = "??")]
    NullishCoalescing,
}

impl std::str::FromStr for LogicalOperator {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "||" => Self::LogicalOr,
            "&&" => Self::LogicalAnd,
            "??" => Self::NullishCoalescing,
            _ => panic!("{s}"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MemberExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub object: NodeRef,   // Expression | Super
    pub property: NodeRef, // Expression | PrivateIdentifier
    pub computed: bool,
    pub optional: bool,
}

impl MemberExpression {
    fn new(
        start: &Location,
        end: &Location,
        object: NodeRef,
        property: NodeRef,
        computed: bool,
        optional: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            object,
            property,
            computed,
            optional,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.object.validate_expression()?;
        self.property.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct ConditionalExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub test: NodeRef,       // Expression
    pub consequent: NodeRef, // Expression
    pub alternate: NodeRef,  // Expression
}

impl ConditionalExpression {
    fn new(
        start: &Location,
        end: &Location,
        test: NodeRef,
        consequent: NodeRef,
        alternate: NodeRef,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            test,
            consequent,
            alternate,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.test.validate_expression()?;
        self.consequent.validate_expression()?;
        self.alternate.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct CallExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub callee: NodeRef,         // Expression | Super
    pub arguments: Vec<NodeRef>, // [ Expression | SpreadElement ]
    pub optional: bool,
}

impl CallExpression {
    fn new(
        start: &Location,
        end: &Location,
        callee: NodeRef,
        arguments: Vec<NodeRef>,
        optional: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            callee,
            arguments,
            optional,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.callee.validate_expression()?;
        for argument in self.arguments.iter() {
            argument.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct NewExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub callee: NodeRef,         // Expression
    pub arguments: Vec<NodeRef>, // [ Expression | SpreadElement ]
}

impl NewExpression {
    fn new(start: &Location, end: &Location, callee: NodeRef, arguments: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            callee,
            arguments,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.callee.validate_expression()?;
        for argument in self.arguments.iter() {
            argument.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SequenceExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub expressions: Vec<NodeRef>, // [ Expression ]
}

impl SequenceExpression {
    fn new(start: &Location, end: &Location, expressions: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            expressions,
        }
    }

    fn validate(&self) -> Result<(), String> {
        for expr in self.expressions.iter() {
            expr.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub location: LocationData,
    #[serde(flatten)]
    pub function: Function,
}

impl ArrowFunctionExpression {
    fn new(
        start: &Location,
        end: &Location,
        id: Option<NodeRef>,
        params: Vec<NodeRef>,
        body: NodeRef,
        r#async: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            function: Function::new(id, params, body, false, r#async),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.function.validate()
    }
}

#[derive(Debug, Serialize)]
pub struct YieldExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: Option<NodeRef>, // Expression | null
    pub delegate: bool,
}

impl YieldExpression {
    fn new(start: &Location, end: &Location, argument: Option<NodeRef>, delegate: bool) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
            delegate,
        }
    }

    fn validate(&self) -> Result<(), String> {
        match self.argument {
            Some(ref argument) => argument.validate_expression(),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateLiteral {
    #[serde(flatten)]
    pub location: LocationData,
    pub quasis: Vec<NodeRef>,      // [ TemplateElement ]
    pub expressions: Vec<NodeRef>, // [ Expression ]
}

impl TemplateLiteral {
    fn new(
        start: &Location,
        end: &Location,
        quasis: Vec<NodeRef>,
        expressions: Vec<NodeRef>,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            quasis,
            expressions,
        }
    }

    fn validate(&self) -> Result<(), String> {
        for expr in self.expressions.iter() {
            expr.validate_expression()?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct TaggedTemplateExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub tag: NodeRef,   // Expression
    pub quasi: NodeRef, // TemplateLiteral
}

impl TaggedTemplateExpression {
    fn new(start: &Location, end: &Location, tag: NodeRef, quasi: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            tag,
            quasi,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.tag.validate_expression()?;
        self.quasi.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct ClassExpression {
    #[serde(flatten)]
    pub location: LocationData,
    #[serde(flatten)]
    pub class: Class,
}

impl ClassExpression {
    fn new(start: &Location, end: &Location, id: Option<NodeRef>, class_tail: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            class: Class::new(id, class_tail),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MetaProperty {
    #[serde(flatten)]
    pub location: LocationData,
    pub meta: NodeRef,     // Identifier
    pub property: NodeRef, // Identifier
}

impl MetaProperty {
    fn new(start: &Location, end: &Location, meta: NodeRef, property: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            meta,
            property,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AwaitExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: NodeRef, // Expression
}

impl AwaitExpression {
    fn new(start: &Location, end: &Location, argument: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.argument.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct ImportExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub source: NodeRef, // Expression
}

impl ImportExpression {
    fn new(start: &Location, end: &Location, source: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            source,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.source.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct ChainExpression {
    #[serde(flatten)]
    pub location: LocationData,
    pub expression: NodeRef, // ChainElement (CallExpression | MemberExpression)
}

impl ChainExpression {
    fn new(start: &Location, end: &Location, expression: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            expression,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.expression.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub location: LocationData,
    pub properties: Vec<NodeRef>, // [ AssignmentProperty | RestElement ]
}

impl ObjectPattern {
    fn new(start: &Location, end: &Location, properties: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            properties,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub location: LocationData,
    pub elements: Vec<Option<NodeRef>>, // [ Pattern | null ]
}

impl ArrayPattern {
    fn new(start: &Location, end: &Location, elements: Vec<Option<NodeRef>>) -> Self {
        Self {
            location: LocationData::new(start, end),
            elements,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RestElement {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: NodeRef, // Pattern
}

impl RestElement {
    fn new(start: &Location, end: &Location, argument: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub location: LocationData,
    pub left: NodeRef,  // Pattern
    pub right: NodeRef, // Expression
}

impl AssignmentPattern {
    fn new(start: &Location, end: &Location, left: NodeRef, right: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            left,
            right,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Super {
    #[serde(flatten)]
    pub location: LocationData,
}

impl Super {
    fn new(start: &Location, end: &Location) -> Self {
        Self {
            location: LocationData::new(start, end),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SpreadElement {
    #[serde(flatten)]
    pub location: LocationData,
    pub argument: NodeRef, // Expression
}

impl SpreadElement {
    fn new(start: &Location, end: &Location, argument: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            argument,
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.argument.validate_expression()
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateElement {
    #[serde(flatten)]
    pub location: LocationData,
    pub value: TemplateValue,
    pub tail: bool,
}

impl TemplateElement {
    fn new(start: &Location, end: &Location, raw: &str, tail: bool) -> Self {
        Self {
            location: LocationData::new(start, end),
            value: TemplateValue::new(raw),
            tail,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateValue {
    pub cooked: String,
    pub raw: String,
}

impl TemplateValue {
    fn new(raw: &str) -> Self {
        Self {
            cooked: literal_content_to_string(raw),
            raw: raw.to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StaticBlock {
    #[serde(flatten)]
    pub location: LocationData,
    pub body: Vec<NodeRef>, // [Statement]
}

impl StaticBlock {
    fn new(start: &Location, end: &Location, body: Vec<NodeRef>) -> Self {
        Self {
            location: LocationData::new(start, end),
            body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PropertyDefinition {
    #[serde(flatten)]
    pub location: LocationData,
    pub key: NodeRef,           // Expression | PrivateIdentifier
    pub value: Option<NodeRef>, // Expression | null
    pub computed: bool,
    pub r#static: bool,
}

impl PropertyDefinition {
    fn new(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: Option<NodeRef>,
        computed: bool,
        r#static: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            key,
            value,
            computed,
            r#static,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MethodDefinition {
    #[serde(flatten)]
    pub location: LocationData,
    pub key: NodeRef,   // Expression
    pub value: NodeRef, // FunctionExpression
    pub kind: MethodKind,
    pub computed: bool,
    pub r#static: bool,
}

impl MethodDefinition {
    fn new(
        start: &Location,
        end: &Location,
        key: NodeRef,
        value: NodeRef,
        kind: MethodKind,
        computed: bool,
        r#static: bool,
    ) -> Self {
        Self {
            location: LocationData::new(start, end),
            key,
            value,
            kind,
            computed,
            r#static,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImportSpecifier {
    #[serde(flatten)]
    pub location: LocationData,
    pub imported: NodeRef, // Identifier | Literal
    pub local: NodeRef,    // Identifier
}

impl ImportSpecifier {
    fn new(start: &Location, end: &Location, imported: NodeRef, local: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            imported,
            local,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImportDefaultSpecifier {
    #[serde(flatten)]
    pub location: LocationData,
    pub local: NodeRef, // Identifier
}

impl ImportDefaultSpecifier {
    fn new(start: &Location, end: &Location, local: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            local,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImportNamespaceSpecifier {
    #[serde(flatten)]
    pub location: LocationData,
    pub local: NodeRef, // Identifier
}

impl ImportNamespaceSpecifier {
    fn new(start: &Location, end: &Location, local: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            local,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExportSpecifier {
    #[serde(flatten)]
    pub location: LocationData,
    pub local: NodeRef,    // Identifier
    pub exported: NodeRef, // Identifier
}

impl ExportSpecifier {
    fn new(start: &Location, end: &Location, local: NodeRef, exported: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            local,
            exported,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CoverInitializedName {
    #[serde(flatten)]
    pub location: LocationData,
    pub name: NodeRef,  // Identifier
    pub value: NodeRef, // Expression
}

impl CoverInitializedName {
    fn new(start: &Location, end: &Location, name: NodeRef, value: NodeRef) -> Self {
        Self {
            location: LocationData::new(start, end),
            name,
            value,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Scalar {
    Null,
    Boolean(bool),
    U64(u64),
    F64(f64),
    String(String),
    EmptyObject {},
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum RawString {
    Static(&'static str),
    Dynamic(String),
}

impl std::ops::Deref for RawString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Static(s) => s,
            Self::Dynamic(s) => s.as_str(),
        }
    }
}

impl std::string::ToString for RawString {
    fn to_string(&self) -> String {
        match self {
            Self::Static(s) => s.to_string(),
            Self::Dynamic(ref s) => s.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
}

fn numeric_literal_to_scalar(literal: &str) -> Scalar {
    // TODO
    if literal.starts_with("0b") || literal.starts_with("0B") {
        if let Ok(n) = u64::from_str_radix(&literal[2..], 2) {
            return Scalar::U64(n);
        }
    }
    if literal.starts_with("0o") || literal.starts_with("0O") {
        if let Ok(n) = u64::from_str_radix(&literal[2..], 8) {
            return Scalar::U64(n);
        }
    }
    if literal.starts_with("0x") || literal.starts_with("0X") {
        if let Ok(n) = u64::from_str_radix(&literal[2..], 16) {
            return Scalar::U64(n);
        }
    }
    if let Ok(n) = literal.parse::<f64>() {
        if n.fract() == 0.0 && n <= (i64::MAX as f64) {
            return Scalar::U64(n as u64);
        }
        return Scalar::F64(n);
    }
    Scalar::Null
}

macro_rules! node {
    (identifier @ $start:ident .. $end:ident; $name:expr) => {
        crate::nodes::Node::identifier(&$start, &$end, $name)
    };
    (null @ $start:ident .. $end:ident) => {
        crate::nodes::Node::null(&$start, &$end)
    };
    (true @ $start:ident .. $end:ident) => {
        crate::nodes::Node::boolean(&$start, &$end, true)
    };
    (false @ $start:ident .. $end:ident) => {
        crate::nodes::Node::boolean(&$start, &$end, false)
    };
    (number @ $start:ident .. $end:ident ; $raw:expr) => {
        crate::nodes::Node::number(&$start, &$end, $raw)
    };
    (string @ $start:ident .. $end:ident ; $raw:expr) => {
        crate::nodes::Node::string(&$start, &$end, $raw)
    };
    (regexp @ $start:ident .. $end:ident ; $raw:expr) => {
        crate::nodes::Node::regexp(&$start, &$end, $raw)
    };
    (script @ $start:ident .. $end:ident) => {
        crate::nodes::Node::program(&$start, &$end, vec![], crate::nodes::SourceType::Script)
    };
    (script @ $start:ident .. $end:ident ; $body:expr) => {
        crate::nodes::Node::program(&$start, &$end, $body, crate::nodes::SourceType::Script)
    };
    (module @ $start:ident .. $end:ident) => {
        crate::nodes::Node::program(&$start, &$end, vec![], crate::nodes::SourceType::Module)
    };
    (module @ $start:ident .. $end:ident ; $body:expr) => {
        crate::nodes::Node::program(&$start, &$end, $body, crate::nodes::SourceType::Module)
    };
    // statements
    (expression_statement @ $start:ident .. $end:ident ; $expr:expr) => {
        crate::nodes::Node::expression_statement(&$start, &$end, $expr)
    };
    (block_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::block_statement(&$start, &$end, vec![])
    };
    (block_statement @ $start:ident .. $end:ident ; $body:expr) => {
        crate::nodes::Node::block_statement(&$start, &$end, $body)
    };
    (function_body@$start:ident..$end:ident) => {
        crate::nodes::Node::function_body(&$start, &$end, vec![])
    };
    (function_body@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::function_body(&$start, &$end, $body)
    };
    (empty_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::empty_statement(&$start, &$end)
    };
    (debugger_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::debugger_statement(&$start, &$end)
    };
    (with_statement @ $start:ident .. $end:ident ; $object:expr, $body:expr) => {
        crate::nodes::Node::with_statement(&$start, &$end, $object, $body)
    };
    (return_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::return_statement(&$start, &$end, None)
    };
    (return_statement @ $start:ident .. $end:ident ; $argument:expr) => {
        crate::nodes::Node::return_statement(&$start, &$end, Some($argument))
    };
    (labeled_statement @ $start:ident .. $end:ident ; $label:expr, $body:expr) => {
        crate::nodes::Node::labeled_statement(&$start, &$end, $label, $body)
    };
    (break_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::break_statement(&$start, &$end, None)
    };
    (break_statement @ $start:ident .. $end:ident ; $label:expr) => {
        crate::nodes::Node::break_statement(&$start, &$end, Some($label))
    };
    (continue_statement @ $start:ident .. $end:ident) => {
        crate::nodes::Node::continue_statement(&$start, &$end, None)
    };
    (continue_statement @ $start:ident .. $end:ident ; $label:expr) => {
        crate::nodes::Node::continue_statement(&$start, &$end, Some($label))
    };
    (if_statement @ $start:ident .. $end:ident ; $test:expr, $consequent:expr) => {
        crate::nodes::Node::if_statement(&$start, &$end, $test, $consequent, None)
    };
    (if_statement @ $start:ident .. $end:ident ; $test:expr, $consequent:expr, $alternate:expr) => {
        crate::nodes::Node::if_statement(&$start, &$end, $test, $consequent, Some($alternate))
    };
    (switch_statement@$start:ident..$end:ident; $discriminant:expr, $cases:expr) => {
        crate::nodes::Node::switch_statement(&$start, &$end, $discriminant, $cases)
    };
    (switch_case@$start:ident..$end:ident; $test:expr) => {
        crate::nodes::Node::switch_case(&$start, &$end, Some($test), vec![])
    };
    (switch_case@$start:ident..$end:ident; $test:expr, $consequent:expr) => {
        crate::nodes::Node::switch_case(&$start, &$end, Some($test), $consequent)
    };
    (switch_default@$start:ident..$end:ident) => {
        crate::nodes::Node::switch_case(&$start, &$end, None, vec![])
    };
    (switch_default@$start:ident..$end:ident; $consequent:expr) => {
        crate::nodes::Node::switch_case(&$start, &$end, None, $consequent)
    };
    (throw_statement@$start:ident..$end:ident; $argument:expr) => {
        crate::nodes::Node::throw_statement(&$start, &$end, $argument)
    };
    (try_statement@$start:ident..$end:ident; $block:expr, catch => $handler:expr, finally => $finalizer:expr) => {
        crate::nodes::Node::try_statement(&$start, &$end, $block, Some($handler), Some($finalizer))
    };
    (try_statement@$start:ident..$end:ident; $block:expr, catch => $handler:expr) => {
        crate::nodes::Node::try_statement(&$start, &$end, $block, Some($handler), None)
    };
    (try_statement@$start:ident..$end:ident; $block:expr, finally => $finalizer:expr) => {
        crate::nodes::Node::try_statement(&$start, &$end, $block, None, Some($finalizer))
    };
    (catch_clause@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::catch_clause(&$start, &$end, None, $body)
    };
    (catch_clause@$start:ident..$end:ident; $param:expr, $body:expr) => {
        crate::nodes::Node::catch_clause(&$start, &$end, Some($param), $body)
    };
    (while_statement@$start:ident..$end:ident; $test:expr, $body:expr) => {
        crate::nodes::Node::while_statement(&$start, &$end, $test, $body)
    };
    (do_while_statement@$start:ident..$end:ident; $test:expr, $body:expr) => {
        crate::nodes::Node::do_while_statement(&$start, &$end, $test, $body)
    };
    (for_statement@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, None, None, None, $body)
    };
    (for_statement@$start:ident..$end:ident; $init:expr; ; ; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, Some($init), None, None, $body)
    };
    (for_statement@$start:ident..$end:ident; ; $test:expr; ; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, None, Some($test), None, $body)
    };
    (for_statement@$start:ident..$end:ident; $init:expr; $test:expr; ; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, Some($init), Some($test), None, $body)
    };
    (for_statement@$start:ident..$end:ident; ; ; $update:expr; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, None, None, Some($update), $body)
    };
    (for_statement@$start:ident..$end:ident; $init:expr; ; $update:expr; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, Some($init), None, Some($update), $body)
    };
    (for_statement@$start:ident..$end:ident; ; $test:expr; $update:expr; $body:expr) => {
        crate::nodes::Node::for_statement(&$start, &$end, None, Some($test), Some($update), $body)
    };
    (for_statement@$start:ident..$end:ident; $init:expr; $test:expr; $update:expr; $body:expr) => {
        crate::nodes::Node::for_statement(
            &$start,
            &$end,
            Some($init),
            Some($test),
            Some($update),
            $body,
        )
    };
    (for_in_statement@$start:ident..$end:ident; $left:expr, $right:expr, $body:expr) => {
        crate::nodes::Node::for_in_statement(&$start, &$end, $left, $right, $body)
    };
    (for_of_statement@$start:ident..$end:ident; $left:expr, $right:expr, $body:expr) => {
        crate::nodes::Node::for_of_statement(&$start, &$end, $left, $right, $body, false)
    };
    (for_await_of_statement@$start:ident..$end:ident; $left:expr, $right:expr, $body:expr) => {
        crate::nodes::Node::for_of_statement(&$start, &$end, $left, $right, $body, true)
    };
    (function_declaration@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(&$start, &$end, None, $params, $body, false, false)
    };
    (function_declaration@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            false,
            false,
        )
    };
    (generator_declaration@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(&$start, &$end, None, $params, $body, true, false)
    };
    (generator_declaration@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            true,
            false,
        )
    };
    (async_function_declaration@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(&$start, &$end, None, $params, $body, false, true)
    };
    (async_function_declaration@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            false,
            true,
        )
    };
    (async_generator_declaration@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(&$start, &$end, None, $params, $body, true, true)
    };
    (async_generator_declaration@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_declaration(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            true,
            true,
        )
    };
    (variable_declaration@$start:ident..$end:ident; $kind:expr, $declarations:expr) => {
        crate::nodes::Node::variable_declaration(
            &$start,
            &$end,
            $kind.parse().unwrap(),
            $declarations,
        )
    };
    (variable_declarator@$start:ident..$end:ident; $id:expr) => {
        crate::nodes::Node::variable_declarator(&$start, &$end, $id, None)
    };
    (variable_declarator@$start:ident..$end:ident; $id:expr, $init:expr) => {
        crate::nodes::Node::variable_declarator(&$start, &$end, $id, Some($init))
    };
    (class_declaration@$start:ident..$end:ident; $class_tail:expr) => {
        crate::nodes::Node::class_declaration(&$start, &$end, None, $class_tail)
    };
    (class_declaration@$start:ident..$end:ident; $id:expr, $class_tail:expr) => {
        crate::nodes::Node::class_declaration(&$start, &$end, Some($id), $class_tail)
    };
    (import_declaration@$start:ident..$end:ident; $source:expr) => {
        crate::nodes::Node::import_declaration(&$start, &$end, vec![], $source)
    };
    (import_declaration@$start:ident..$end:ident; $specifiers:expr, $source:expr) => {
        crate::nodes::Node::import_declaration(&$start, &$end, $specifiers, $source)
    };
    (export_named_declaration@$start:ident..$end:ident; $specifiers:expr, $source:expr) => {
        crate::nodes::Node::export_named_declaration(
            &$start,
            &$end,
            None,
            $specifiers,
            Some($source),
        )
    };
    (export_named_declaration@$start:ident..$end:ident; declaration => $declaration:expr) => {
        crate::nodes::Node::export_named_declaration(
            &$start,
            &$end,
            Some($declaration),
            vec![],
            None,
        )
    };
    (export_named_declaration@$start:ident..$end:ident; specifiers => $specifiers:expr) => {
        crate::nodes::Node::export_named_declaration(&$start, &$end, None, $specifiers, None)
    };
    (export_default_declaration@$start:ident..$end:ident; $declaration:expr) => {
        crate::nodes::Node::export_default_declaration(&$start, &$end, $declaration)
    };
    (export_all_declaration@$start:ident..$end:ident; $source:expr) => {
        crate::nodes::Node::export_all_declaration(&$start, &$end, None, $source)
    };
    (export_all_declaration@$start:ident..$end:ident; $exported:expr, $source:expr) => {
        crate::nodes::Node::export_all_declaration(&$start, &$end, Some($exported), $source)
    };
    // expressions
    (this_expression@$start:ident..$end:ident) => {
        crate::nodes::Node::this_expression(&$start, &$end)
    };
    (array_expression@$start:ident..$end:ident) => {
        crate::nodes::Node::array_expression(&$start, &$end, vec![], false)
    };
    (array_expression@$start:ident..$end:ident; $elements:expr) => {
        crate::nodes::Node::array_expression(&$start, &$end, $elements, false)
    };
    (array_expression@$start:ident..$end:ident; $elements:expr; trailing_comma) => {
        crate::nodes::Node::array_expression(&$start, &$end, $elements, true)
    };
    (object_expression@$start:ident..$end:ident) => {
        crate::nodes::Node::object_expression(&$start, &$end, vec![])
    };
    (object_expression@$start:ident..$end:ident; $properties:expr) => {
        crate::nodes::Node::object_expression(&$start, &$end, $properties)
    };
    (property@$start:ident..$end:ident; $key:expr) => {
        crate::nodes::Node::property(
            &$start,
            &$end,
            $key.clone(),
            $key,
            crate::nodes::PropertyKind::Init,
            true,
        )
    };
    (property@$start:ident..$end:ident; $key:expr => $value:expr) => {
        crate::nodes::Node::property(
            &$start,
            &$end,
            $key,
            $value,
            crate::nodes::PropertyKind::Init,
            false,
        )
    };
    (property@$start:ident..$end:ident; get $key:expr => $value:expr) => {
        crate::nodes::Node::property(
            &$start,
            &$end,
            $key,
            $value,
            crate::nodes::PropertyKind::Get,
            false,
        )
    };
    (property@$start:ident..$end:ident; set $key:expr => $value:expr) => {
        crate::nodes::Node::property(
            &$start,
            &$end,
            $key,
            $value,
            crate::nodes::PropertyKind::Set,
            false,
        )
    };
    (function_expression@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::function_expression(&$start, &$end, None, vec![], $body, false, false)
    };
    (function_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(&$start, &$end, None, $params, $body, false, false)
    };
    (function_expression@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            false,
            false,
        )
    };
    (generator_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(&$start, &$end, None, $params, $body, true, false)
    };
    (generator_expression@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            true,
            false,
        )
    };
    (async_function_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(&$start, &$end, None, $params, $body, false, true)
    };
    (async_function_expression@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            false,
            true,
        )
    };
    (async_generator_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(&$start, &$end, None, $params, $body, true, true)
    };
    (async_generator_expression@$start:ident..$end:ident; $id:expr, $params:expr, $body:expr) => {
        crate::nodes::Node::function_expression(
            &$start,
            &$end,
            Some($id),
            $params,
            $body,
            true,
            true,
        )
    };
    (unary_expression@$start:ident..$end:ident; $operator:expr, $argument:expr) => {
        crate::nodes::Node::unary_expression(&$start, &$end, &$operator, $argument)
    };
    (update_expression@$start:ident..$end:ident; $operator:expr, $argument:expr, $prefix:expr) => {
        crate::nodes::Node::update_expression(&$start, &$end, &$operator, $argument, $prefix)
    };
    (binary_expression@$start:ident..$end:ident; $operator:expr, $left:expr, $right:expr) => {
        crate::nodes::Node::binary_expression(&$start, &$end, &$operator, $left, $right)
    };
    (assignment_expression@$start:ident..$end:ident; $operator:expr, $left:expr, $right:expr) => {
        crate::nodes::Node::assignment_expression(&$start, &$end, &$operator, $left, $right)
    };
    (logical_expression@$start:ident..$end:ident; $operator:expr, $left:expr, $right:expr) => {
        crate::nodes::Node::logical_expression(&$start, &$end, &$operator, $left, $right)
    };
    (member_expression@$start:ident..$end:ident; $object:expr, $property:expr, $computed:expr) => {
        crate::nodes::Node::member_expression(&$start, &$end, $object, $property, $computed, false)
    };
    (conditional_expression@$start:ident..$end:ident; $test:expr, $consequent:expr, $alternate:expr) => {
        crate::nodes::Node::conditional_expression(&$start, &$end, $test, $consequent, $alternate)
    };
    (call_expression@$start:ident..$end:ident; $callee:expr, $arguments:expr) => {
        crate::nodes::Node::call_expression(&$start, &$end, $callee, $arguments, false)
    };
    (new_expression@$start:ident..$end:ident; $callee:expr) => {
        crate::nodes::Node::new_expression(&$start, &$end, $callee, vec![])
    };
    (new_expression@$start:ident..$end:ident; $callee:expr, $arguments:expr) => {
        crate::nodes::Node::new_expression(&$start, &$end, $callee, $arguments)
    };
    (sequence_expression@$start:ident..$end:ident; $expressions:expr) => {
        crate::nodes::Node::sequence_expression(&$start, &$end, $expressions)
    };
    (arrow_function_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::arrow_function_expression(&$start, &$end, None, $params, $body, false)
    };
    (async_arrow_function_expression@$start:ident..$end:ident; $params:expr, $body:expr) => {
        crate::nodes::Node::arrow_function_expression(&$start, &$end, None, $params, $body, true)
    };
    (yield_expression@$start:ident..$end:ident; $delegate:expr) => {
        crate::nodes::Node::yield_expression(&$start, &$end, None, $delegate)
    };
    (yield_expression@$start:ident..$end:ident; $argument:expr, $delegate:expr) => {
        crate::nodes::Node::yield_expression(&$start, &$end, Some($argument), $delegate)
    };
    (template_literal@$start:ident..$end:ident; $quasis:expr, $expressions:expr) => {
        crate::nodes::Node::template_literal(&$start, &$end, $quasis, $expressions)
    };
    (tagged_template_expression@$start:ident..$end:ident; $tag:expr, $quasi:expr) => {
        crate::nodes::Node::tagged_template_expression(&$start, &$end, $tag, $quasi)
    };
    (class_expression@$start:ident..$end:ident; $class_tail:expr) => {
        crate::nodes::Node::class_expression(&$start, &$end, None, $class_tail)
    };
    (class_expression@$start:ident..$end:ident; $id:expr, $class_tail:expr) => {
        crate::nodes::Node::class_expression(&$start, &$end, Some($id), $class_tail)
    };
    (meta_property@$start:ident..$end:ident; $meta:expr, $property:expr) => {
        crate::nodes::Node::meta_property(&$start, &$end, $meta, $property)
    };
    (await_expression@$start:ident..$end:ident; $argument:expr) => {
        crate::nodes::Node::await_expression(&$start, &$end, $argument)
    };
    (import_expression@$start:ident..$end:ident; $source:expr) => {
        crate::nodes::Node::import_expression(&$start, &$end, $source)
    };
    (chain_expression@$start:ident..$end:ident; $expr:expr, $chains:expr) => {
        crate::nodes::Node::chain_expression(&$start, &$end, $expr, $chains)
    };
    // patterns
    (object_pattern@$start:ident..$end:ident) => {
        crate::nodes::Node::object_pattern(&$start, &$end, vec![])
    };
    (object_pattern@$start:ident..$end:ident; $properties:expr) => {
        crate::nodes::Node::object_pattern(&$start, &$end, $properties)
    };
    (array_pattern@$start:ident..$end:ident) => {
        crate::nodes::Node::array_pattern(&$start, &$end, vec![])
    };
    (array_pattern@$start:ident..$end:ident; $elements:expr) => {
        crate::nodes::Node::array_pattern(&$start, &$end, $elements)
    };
    (rest_element@$start:ident..$end:ident; $argument:expr) => {
        crate::nodes::Node::rest_element(&$start, &$end, $argument)
    };
    (assignment_pattern@$start:ident..$end:ident; $left:expr, $right:expr) => {
        crate::nodes::Node::assignment_pattern(&$start, &$end, $left, $right)
    };
    // others
    (super_@$start:ident..$end:ident) => {
        crate::nodes::Node::super_(&$start, &$end)
    };
    (spread_element@$start:ident..$end:ident; $argument:expr) => {
        crate::nodes::Node::spread_element(&$start, &$end, $argument)
    };
    (template_element@$start:ident..$end:ident; $raw:expr, $tail:expr) => {
        crate::nodes::Node::template_element(&$start, &$end, $raw, $tail)
    };
    (class_body@$start:ident..$end:ident) => {
        crate::nodes::Node::class_body(&$start, &$end, vec![])
    };
    (class_body@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::class_body(&$start, &$end, $body)
    };
    (static_block@$start:ident..$end:ident; $body:expr) => {
        crate::nodes::Node::static_block(&$start, &$end, $body)
    };
    (static_property_definition@$start:ident..$end:ident; $def:expr) => {
        crate::nodes::Node::property_definition_update(&$start, &$end, $def, true)
    };
    (static_method_definition@$start:ident..$end:ident; $method:expr) => {
        crate::nodes::Node::static_method_definition(&$start, &$end, $method)
    };
    (property_definition@$start:ident..$end:ident; $key:expr) => {
        crate::nodes::Node::property_definition(&$start, &$end, $key, None)
    };
    (property_definition@$start:ident..$end:ident; $key:expr, $value:expr) => {
        crate::nodes::Node::property_definition(&$start, &$end, $key, Some($value))
    };
    (end_property_definition@$start:ident..$end:ident; $def:expr) => {
        crate::nodes::Node::property_definition_update(&$start, &$end, $def, false)
    };
    (method_definition@$start:ident..$end:ident; $key:expr, $value:expr) => {
        crate::nodes::Node::method_definition(&$start, &$end, $key, $value)
    };
    (getter@$start:ident..$end:ident; $key:expr, $value:expr) => {
        crate::nodes::Node::getter(&$start, &$end, $key, $value)
    };
    (setter@$start:ident..$end:ident; $key:expr, $value:expr) => {
        crate::nodes::Node::setter(&$start, &$end, $key, $value)
    };
    (import_specifier@$start:ident..$end:ident; $imported:expr) => {
        crate::nodes::Node::import_specifier(&$start, &$end, $imported.clone(), $imported)
    };
    (import_specifier@$start:ident..$end:ident; $imported:expr => $local:expr) => {
        crate::nodes::Node::import_specifier(&$start, &$end, $imported, $local)
    };
    (import_default_specifier@$start:ident..$end:ident; $local:expr) => {
        crate::nodes::Node::import_default_specifier(&$start, &$end, $local)
    };
    (import_namespace_specifier@$start:ident..$end:ident; $local:expr) => {
        crate::nodes::Node::import_namespace_specifier(&$start, &$end, $local)
    };
    (export_specifier@$start:ident..$end:ident; $local:expr) => {
        crate::nodes::Node::export_specifier(&$start, &$end, $local.clone(), $local)
    };
    (export_specifier@$start:ident..$end:ident; $local:expr => $exported:expr) => {
        crate::nodes::Node::export_specifier(&$start, &$end, $local, $exported)
    };
    (private_identifier@$start:ident..$end:ident; $name:expr) => {
        crate::nodes::Node::private_identifier(&$start, &$end, $name)
    };
    // internals
    (class_tail; $body:expr) => {
        crate::nodes::Node::class_tail(None, $body)
    };
    (class_tail; $super_class:expr, $body:expr) => {
        crate::nodes::Node::class_tail(Some($super_class), $body)
    };
    (computed_property_name; $expr:expr) => {
        crate::nodes::Node::computed_property_name($expr)
    };
    (optional_call@$end:ident; $arguments:expr) => {
        crate::nodes::Node::optional_call($arguments, $end.clone())
    };
    (optional_member@$end:ident; $expr:expr, $computed:expr) => {
        crate::nodes::Node::optional_member($expr, $computed, $end.clone())
    };
    (cover_initialized_name@$start:ident..$end:ident; $name:expr, $value:expr) => {
        crate::nodes::Node::cover_initialized_name(&$start, &$end, $name, $value)
    };
    (for_init_update; $init:expr) => {
        crate::nodes::Node::for_init_update($init)
    };
    (into_patterns; $nullable:expr) => {
        crate::nodes::Node::into_patterns($nullable)
    };
    (into_pattern; $expr:expr) => {
        crate::nodes::Node::into_pattern($expr)
    };
    (into_property; $method:expr) => {
        crate::nodes::Node::into_property($method)
    };
}

pub(crate) use node;
