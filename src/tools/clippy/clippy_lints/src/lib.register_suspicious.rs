// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::suspicious", Some("clippy_suspicious"), vec![
    LintId::of(assign_ops::MISREFACTORED_ASSIGN_OP),
    LintId::of(attrs::BLANKET_CLIPPY_RESTRICTION_LINTS),
    LintId::of(eval_order_dependence::EVAL_ORDER_DEPENDENCE),
    LintId::of(float_equality_without_abs::FLOAT_EQUALITY_WITHOUT_ABS),
    LintId::of(formatting::SUSPICIOUS_ASSIGNMENT_FORMATTING),
    LintId::of(formatting::SUSPICIOUS_ELSE_FORMATTING),
    LintId::of(formatting::SUSPICIOUS_UNARY_OP_FORMATTING),
    LintId::of(loops::EMPTY_LOOP),
    LintId::of(loops::FOR_LOOPS_OVER_FALLIBLES),
    LintId::of(loops::MUT_RANGE_BOUND),
    LintId::of(methods::SUSPICIOUS_MAP),
    LintId::of(mut_key::MUTABLE_KEY_TYPE),
    LintId::of(octal_escapes::OCTAL_ESCAPES),
    LintId::of(suspicious_trait_impl::SUSPICIOUS_ARITHMETIC_IMPL),
    LintId::of(suspicious_trait_impl::SUSPICIOUS_OP_ASSIGN_IMPL),
])
