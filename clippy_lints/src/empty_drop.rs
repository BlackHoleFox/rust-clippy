use clippy_utils::diagnostics::span_lint_and_help;
// use clippy_utils::ty::is_type_lang_item;
use if_chain::if_chain;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for empty `Drop` implementations.
    ///
    /// ### Why is this bad?
    /// Empty `Drop` implementations do nothing except hurt code readability. They also prevent the type from being destructured.
    ///
    /// ### Example
    /// ```rust
    /// struct S;
    ///
    /// impl Drop for S {
    ///     fn drop(&mut self) {}
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// struct S;
    /// ```
    #[clippy::version = "1.61.0"]
    pub EMPTY_DROP,
    correctness,
    "empty `Drop` implementations"
}
declare_lint_pass!(EmptyDrop => [EMPTY_DROP]);

impl LateLintPass<'_> for EmptyDrop {
    fn check_item(&mut self, cx: &LateContext<'_>, item: &Item<'_>) {
        if_chain! {
            if let ItemKind::Impl(Impl {
                of_trait: Some(ref trait_ref),
                items: [child],
                ..
            }) = item.kind;
            if trait_ref.trait_def_id() == cx.tcx.lang_items().drop_trait();
            if let impl_item_hir = child.id.hir_id();
            if let Some(Node::ImplItem(impl_item)) = cx.tcx.hir().find(impl_item_hir);
            if let ImplItemKind::Fn(_, b) = &impl_item.kind;
            if let Body {value: func_expr, ..} = cx.tcx.hir().body(*b);
            if let ExprKind::Block(ref block, _) = func_expr.kind;
            if block.stmts.is_empty();
            then {
                span_lint_and_help(cx, EMPTY_DROP, item.span, "empty drop implementation", None, "try removing this impl");
            }
        }
    }
}
