/**
 * @file algebraic parser
 * @author none <none>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "alg",
  extras: ($) => [/\s/, $.comment],
  rules: {
    source_file: ($) => repeat($._FORMULA),
    _FORMULA: ($) => choice($.LET, $.DEF, $.EXPR),
    EXPR: ($) => prec(9999, seq($.expr, ";")),
    args: ($) =>
      seq(
        "(",
        optional(
          seq(
            field("parameter", $.expr),
            repeat(seq(",", field("parameter", $.expr))),
          ),
        ),
        ")",
      ),
    DEF: ($) =>
      seq(
        "fn",
        field("name", $.id),
        field("args", $.args),
        "=",
        field("body", $.expr),
        ";",
      ),
    LET: ($) =>
      seq("let", field("name", $.id), "=", field("value", $.expr), ";"),
    expr: ($) =>
      prec(2, choice($.num, $.id, $.call, $.parenthesized, $.unary, $.binary)),
    call: ($) => prec(999, seq(field("called", $.id), field("args", $.args))),
    parenthesized: ($) => seq("(", $.expr, ")"),
    unary: ($) =>
      prec(
        9,
        seq(field("operator", choice("+", "-")), field("operand", $.expr)),
      ),
    binary: ($) =>
      choice(
        prec.left(
          8,
          seq(
            field("left", $.expr),
            field("operator", choice("*", "/")),
            field("right", $.expr),
          ),
        ),
        prec.left(
          7,
          seq(
            field("left", $.expr),
            field("operator", choice("+", "-")),
            field("right", $.expr),
          ),
        ),
      ),
    id: (_) => /[A-Za-z]+/,
    num: (_) => /\d+(\.\d+)?/,
    comment: (_) => token(seq("//", /.*/)),
  },
});
