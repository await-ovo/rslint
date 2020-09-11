use crate::rule_prelude::*;
use ast::SwitchStmt;

declare_lint! {
    /**
    Disallow duplicate test cases in `switch` statements. 

    `switch` statement clauses can freely have duplicate tests, however this is almost always a mistake, because
    the second case is unreachable. It is likely that the programmer copied a case clause but did not change the test for it.

    ## Invalid Code Examples 

    ```ignore
    switch (a) {
        case 1:
            break;
        case 2:
            break;
        case 1:
            break;
        default:
            break;
    }
    ```

    ```ignore
    switch (a) {
        case foo.bar:
            break;

        case foo . bar:
            break;
    }
    ```
    */
    #[derive(Default)]
    NoDuplicateCases,
    errors,
    "no-duplicate-cases"
}

#[typetag::serde]
impl CstRule for NoDuplicateCases {
    fn check_node(&self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        if let Some(switch) = node.try_to::<SwitchStmt>() {
            let mut seen: Vec<SyntaxNode> = vec![];
            for case in switch.cases().filter_map(|case| case.into_case()) {
                if let Some(expr) = case.test() {
                    if let Some(old) = seen.iter().find(|clause| clause.lexical_eq(expr.syntax())) {
                        let err = ctx.err(self.name(), format!("Duplicate switch statement test `{}`", old.trimmed_text()))
                            .secondary(old.trimmed_range(), format!("`{}` is first tested for here", old.trimmed_text()))
                            .primary(expr.syntax().trimmed_range(), format!("`{}` is then tested for again here", expr.syntax().trimmed_text()));

                        ctx.add_err(err)
                    } else {
                        seen.push(expr.syntax().clone());
                    }
                }
            }
        }
        None
    }
}

rule_tests! {
    NoDuplicateCases::default(),
    err: {
        "
        switch (foo) {
            case foo. bar:
            break;

            case foo.bar:
            break;
        }
        ",
        "
        switch foo {
            case 5:
            break;

            case 6:
            break;

            case 5:
            break;
        }
        "
    },
    ok: {}
}
