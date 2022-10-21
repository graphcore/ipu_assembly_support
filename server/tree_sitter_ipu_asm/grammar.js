
const PREC = {
  PAREN_DECLARATOR: -10,
  ASSIGNMENT: -1,
  CONDITIONAL: -2,
  DEFAULT: 0,
  LOGICAL_OR: 1,
  LOGICAL_AND: 2,
  INCLUSIVE_OR: 3,
  EXCLUSIVE_OR: 4,
  BITWISE_AND: 5,
  EQUAL: 6,
  RELATIONAL: 7,
  SIZEOF: 8,
  SHIFT: 9,
  ADD: 10,
  MULTIPLY: 11,
  CAST: 12,
  UNARY: 13,
  CALL: 14,
  FIELD: 15,
  SUBSCRIPT: 16
};

module.exports = grammar({
  name: "ipu_asm",

  extras: $ => [
    $.comment,
    // Any space character or '\<cr><lf>' or '\<lf>'.
    /\s|\\\r?\n/,
  ],

  word: $ => $.identifier,

  rules: {
    translation_unit: $ => repeat($._top_level_item),

    _top_level_item: $ => choice(
      // Preprocessor.
      $.preproc_if,
      $.preproc_ifdef,
      $.preproc_include,
      $.preproc_def,
      $.preproc_function_def,
      $.preproc_call,

      // Assembly
      $.dot_directive,
      $.label_declaration,
      $.bundled_instruction,
      $.instruction,
    ),

    // Preprocesser

    preproc_include: $ => seq(
      preprocessor('include'),
      field('path', choice(
        $.string_literal,
        $.system_lib_string,
        $.identifier,
        alias($.preproc_call_expression, $.call_expression),
      )),
      '\n'
    ),

    preproc_def: $ => seq(
      preprocessor('define'),
      field('name', $.identifier),
      field('value', optional($.preproc_arg)),
      '\n'
    ),

    preproc_function_def: $ => seq(
      preprocessor('define'),
      field('name', $.identifier),
      field('parameters', $.preproc_params),
      field('value', optional($.preproc_arg)),
      '\n'
    ),

    preproc_params: $ => seq(
      token.immediate('('), commaSep(choice($.identifier, '...')), ')'
    ),

    preproc_call: $ => seq(
      field('directive', $.preproc_directive),
      field('argument', optional($.preproc_arg)),
      '\n'
    ),

    ...preprocIf('', $ => $._top_level_item),
    // ...preprocIf('_in_field_declaration_list', $ => $._field_declaration_list_item),

    preproc_directive: $ => /#[ \t]*[a-zA-Z]\w*/,
    preproc_arg: $ => token(prec(-1, repeat1(/.|\\\r?\n/))),

    _preproc_expression: $ => choice(
      $.identifier,
      alias($.preproc_call_expression, $.call_expression),
      $.number_literal,
      $.char_literal,
      $.preproc_defined,
      alias($.preproc_unary_expression, $.unary_expression),
      alias($.preproc_binary_expression, $.binary_expression),
      alias($.preproc_parenthesized_expression, $.parenthesized_expression)
    ),

    preproc_parenthesized_expression: $ => seq(
      '(',
      $._preproc_expression,
      ')'
    ),

    preproc_defined: $ => choice(
      prec(PREC.CALL, seq('defined', '(', $.identifier, ')')),
      seq('defined', $.identifier),
    ),

    preproc_unary_expression: $ => prec.left(PREC.UNARY, seq(
      field('operator', choice('!', '~', '-', '+')),
      field('argument', $._preproc_expression)
    )),

    preproc_call_expression: $ => prec(PREC.CALL, seq(
      field('function', $.identifier),
      field('arguments', alias($.preproc_argument_list, $.argument_list))
    )),

    preproc_argument_list: $ => seq(
      '(',
      commaSep($._preproc_expression),
      ')'
    ),

    preproc_binary_expression: $ => {
      const table = [
        ['+', PREC.ADD],
        ['-', PREC.ADD],
        ['*', PREC.MULTIPLY],
        ['/', PREC.MULTIPLY],
        ['%', PREC.MULTIPLY],
        ['||', PREC.LOGICAL_OR],
        ['&&', PREC.LOGICAL_AND],
        ['|', PREC.INCLUSIVE_OR],
        ['^', PREC.EXCLUSIVE_OR],
        ['&', PREC.BITWISE_AND],
        ['==', PREC.EQUAL],
        ['!=', PREC.EQUAL],
        ['>', PREC.RELATIONAL],
        ['>=', PREC.RELATIONAL],
        ['<=', PREC.RELATIONAL],
        ['<', PREC.RELATIONAL],
        ['<<', PREC.SHIFT],
        ['>>', PREC.SHIFT],
      ];

      return choice(...table.map(([operator, precedence]) => {
        return prec.left(precedence, seq(
          field('left', $._preproc_expression),
          field('operator', operator),
          field('right', $._preproc_expression)
        ))
      }));
    },

    string_literal: $ => seq(
      choice('L"', 'u"', 'U"', 'u8"', '"'),
      repeat(choice(
        token.immediate(prec(1, /[^\\"\n]+/)),
        $.escape_sequence
      )),
      '"',
    ),

    number_literal: $ => {
      const separator = "'";
      const hex = /[0-9a-fA-F]/;
      const decimal = /[0-9]/;
      const hexDigits = seq(repeat1(hex), repeat(seq(separator, repeat1(hex))));
      const decimalDigits = seq(repeat1(decimal), repeat(seq(separator, repeat1(decimal))));
      return token(seq(
        optional(/[-+]/),
        optional(choice('0x', '0b')),
        choice(
          seq(
            choice(
              decimalDigits,
              seq('0b', decimalDigits),
              seq('0x', hexDigits)
            ),
            optional(seq('.', optional(hexDigits)))
          ),
          seq('.', decimalDigits)
        ),
        optional(seq(
          /[eEpP]/,
          optional(seq(
            optional(/[-+]/),
            hexDigits
          ))
        )),
        repeat(choice('u', 'l', 'U', 'L', 'f', 'F'))
      ))
    },

    escape_sequence: $ => token(prec(1, seq(
      '\\',
      choice(
        /[^xuU]/,
        /\d{2,3}/,
        /x[0-9a-fA-F]{2,}/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/
      )
    ))),

    char_literal: $ => seq(
      choice('L\'', 'u\'', 'U\'', 'u8\'', '\''),
      choice(
        $.escape_sequence,
        token.immediate(/[^\n']/)
      ),
      '\''
    ),

    system_lib_string: $ => token(seq(
      '<',
      repeat(choice(/[^>\n]/, '\\>')),
      '>'
    )),

    // Main Grammar

    identifier: $ => /[a-zA-Z_]\w*/,

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    //
    // AsmLexer.cpp:205.
    comment: $ => token(choice(
      seq(choice('//', ';'), /(\\(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/'
      )
    )),

    register: $ => seq(
      optional("$"),
      $.identifier,
      optional(seq(":", $.integer)),
      optional("+="),
    ),


    instruction_parameter: $ => choice(
      prec(4, $.register),
      prec(3, $.label),
      // Reference to a .macro parameter.
      prec(2, seq("\\", $.identifier)),
      prec(1, $.identifier),
      // Cheat by reusing preproc expressions. They're probably pretty close.
      prec(0, $._preproc_expression),
    ),

    // Instruction ends at newline, or statement separator character (;).
    instruction: $ => seq(
      field("mnemonic", $.identifier),
      commaSep($.instruction_parameter),
      choice(";", "\n"),
    ),

    bundled_instruction: $ => seq(
      "{",
      $.instruction,
      $.instruction,
      "}",
    ),

    // The place where the label is defined.
    label_declaration: $ => seq(
      $.label,
      ":",
    ),

    // The name of a label.
    label: $ => choice(
      seq(
        optional(".L"),
        $.identifier,
      ),
      $.integer,
    ),

    integer: $ => /[0-9]+/,

    // Assembler directives. I'm not sure of the grammar so just capture
    // the entire line. See https://ftp.gnu.org/old-gnu/Manuals/gas-2.9.1/html_chapter/as_7.html
    dot_directive: $ => seq(
      ".",
      field("directive", $.identifier),
      field("arguments", $.rest_of_line),
    ),

    rest_of_line: $ => /[^\n]*/,

    // AsmParser.cpp:1397
    //
    ///  expr ::= expr &&,|| expr               -> lowest.
    ///  expr ::= expr |,^,&,! expr
    ///  expr ::= expr ==,!=,<>,<,<=,>,>= expr
    ///  expr ::= expr <<,>> expr
    ///  expr ::= expr +,- expr
    ///  expr ::= expr *,/,% expr               -> highest.
    ///  expr ::= primaryexpr

    /// NOTE: This assumes the leading '(' has already been consumed.
    ///
    /// parenexpr ::= expr)
    ///

    /// NOTE: This assumes the leading '[' has already been consumed.
    ///
    /// bracketexpr ::= expr]
    ///

    ///  primaryexpr ::= (parenexpr
    ///  primaryexpr ::= symbol
    ///  primaryexpr ::= number
    ///  primaryexpr ::= '.'
    ///  primaryexpr ::= ~,+,- primaryexpr

    // asm_expression: $ => ??,

    // AsmLexer.cpp:92
    //
    /// (.[0-9a-fA-F]*)?[pP][+-]?[0-9a-fA-F]+

    // hex_float_literal: $ => (.[0-9a-fA-F]*)?[pP][+-]?[0-9a-fA-F]+,

    // AsmLexer.cpp:139
    //
    /// LexIdentifier: [a-zA-Z_.][a-zA-Z0-9_$.@?]*

    // asm_identifier: $ =>

    // AsmLexer.cpp:281
    //
    /// LexDigit: First character is [0-9].
    ///   Local Label: [0-9][:]
    ///   Forward/Backward Label: [0-9][fb]
    ///   Binary integer: 0b[01]+
    ///   Octal integer: 0[0-7]+
    ///   Hex integer: 0x[0-9a-fA-F]+ or [0x]?[0-9][0-9a-fA-F]*[hH]
    ///   Decimal integer: [1-9][0-9]*

    // C-style 'c'hars and "strings".

    // AsmParser.cpp:1678
    //
    /// ParseStatement:
    ///   ::= EndOfStatement
    ///   ::= Label* Directive ...Operands... EndOfStatement
    ///   ::= Label* Identifier OperandList* EndOfStatement
  },
});


function preprocIf (suffix, content) {
  function elseBlock ($) {
    return choice(
      suffix ? alias($['preproc_else' + suffix], $.preproc_else) : $.preproc_else,
      suffix ? alias($['preproc_elif' + suffix], $.preproc_elif) : $.preproc_elif,
    );
  }

  return {
    ['preproc_if' + suffix]: $ => seq(
      preprocessor('if'),
      field('condition', $._preproc_expression),
      '\n',
      repeat(content($)),
      field('alternative', optional(elseBlock($))),
      preprocessor('endif')
    ),

    ['preproc_ifdef' + suffix]: $ => seq(
      choice(preprocessor('ifdef'), preprocessor('ifndef')),
      field('name', $.identifier),
      repeat(content($)),
      field('alternative', optional(elseBlock($))),
      preprocessor('endif')
    ),

    ['preproc_else' + suffix]: $ => seq(
      preprocessor('else'),
      repeat(content($))
    ),

    ['preproc_elif' + suffix]: $ => seq(
      preprocessor('elif'),
      field('condition', $._preproc_expression),
      '\n',
      repeat(content($)),
      field('alternative', optional(elseBlock($))),
    )
  }
}

function preprocessor (command) {
  return alias(new RegExp('#[ \t]*' + command), '#' + command)
}

function commaSep (rule) {
  return optional(commaSep1(rule))
}

function commaSep1 (rule) {
  return seq(rule, repeat(seq(',', rule)))
}
