#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.2
# from Racc grammar file "".
#

require 'racc/parser.rb'

require 'strscan'
require_relative './global_scope'
require_relative './syntax_error'
require_relative '../code_section'

module CSVPlusPlus
  module Language
    class CodeSectionParser < Racc::Parser

module_eval(<<'...end code_section.y/module_eval...', 'code_section.y', 55)
  def parse(execution_context)
    rest = nil
    execution_context.parsing_code_section! do |input|
      text = input.read.strip
      @code_section = CodeSection.new

      eoc_index = text.index(Language::END_OF_CODE_SECTION)
      next text if eoc_index.nil?

      tokens = []

      s = StringScanner.new text
      until s.empty?
        case
        when s.scan(/\s+/)
        when s.scan(/\#[^\n]+\n/)
        when s.scan(/---/)
          break
        when s.scan(/\n/)
          tokens << [:EOL, s.matched]
        when s.scan(/:=/)
          tokens << [:ASSIGN, s.matched]
        when s.scan(/\bdef\b/)
          tokens << [:FUNCTION_DEF, s.matched]
        when s.scan(/TRUE/)
          tokens << [:TRUE, s.matched]
        when s.scan(/FALSE/)
          tokens << [:FALSE, s.matched]
        when s.scan(/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/)
          tokens << [:STRING, s.matched]
        when s.scan(/-?[\d.]+/)
          tokens << [:NUMBER, s.matched]
        when s.scan(/\$\$/)
          tokens << [:VAR_REF, s.matched]
        when s.scan(/[\w_]+/)
          tokens << [:ID, s.matched]
        when s.scan(/[\(\)\{\}\/\*\+\-,=&]/)
          tokens << [s.matched, s.matched]
        else
          raise SyntaxError.new("Unable to parse starting at", s.rest, execution_context)
        end
      end

      next text if tokens.empty?

      define_singleton_method(:next_token) { tokens.shift }

      begin
        do_parse
      rescue Racc::ParseError => e
        raise SyntaxError.new("Error parsing code section", e.message, execution_context,
                              wrapped_error: e)
      end

      # return the rest of the file (the CSV part) to the execution_context because they're
      # going to use it to rewrite the input file and further parse
      s.rest
    end

    @code_section
  end
...end code_section.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
     7,     5,     5,    25,    26,    33,    34,    29,     6,     6,
    19,    14,    17,    16,    18,    15,    19,    14,    17,    16,
    18,    15,    19,    14,    17,    16,    18,    15,    19,    14,
    17,    16,    18,    15,    19,    14,    17,    16,    18,    15,
    21,     9,    10,    11,    22,    12,    23,    24,    32,    35 ]

racc_action_check = [
     1,     0,     1,    20,    20,    28,    28,    23,     0,     1,
    23,    23,    23,    23,    23,    23,    10,    10,    10,    10,
    10,    10,    21,    21,    21,    21,    21,    21,    25,    25,
    25,    25,    25,    25,    34,    34,    34,    34,    34,    34,
    12,     5,     6,     7,    12,     9,    14,    15,    26,    30 ]

racc_action_pointer = [
    -1,     0,   nil,   nil,   nil,    32,    39,    43,   nil,    41,
     8,   nil,    35,   nil,    42,    38,   nil,   nil,   nil,   nil,
    -2,    14,   nil,     2,   nil,    20,    39,   nil,     0,   nil,
    44,   nil,   nil,   nil,    26,   nil,   nil ]

racc_action_default = [
   -21,   -21,    -2,    -3,    -4,   -21,   -21,   -21,    -1,   -21,
   -21,    37,   -21,    -9,   -18,   -21,   -14,   -15,   -16,   -17,
   -21,   -21,    -8,   -21,   -13,   -21,   -21,    -6,   -21,   -11,
   -20,    -5,    -7,   -10,   -21,   -12,   -19 ]

racc_goto_table = [
    13,     2,     8,     1,    20,    28,   nil,   nil,   nil,   nil,
   nil,    27,   nil,    30,   nil,    31,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,    36 ]

racc_goto_check = [
     6,     2,     2,     1,     5,     7,   nil,   nil,   nil,   nil,
   nil,     6,   nil,     6,   nil,     6,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,     6 ]

racc_goto_pointer = [
   nil,     3,     1,   nil,   nil,    -8,   -10,   -18 ]

racc_goto_default = [
   nil,   nil,   nil,     3,     4,   nil,   nil,   nil ]

racc_reduce_table = [
  0, 0, :racc_error,
  2, 15, :_reduce_none,
  1, 15, :_reduce_none,
  1, 16, :_reduce_none,
  1, 16, :_reduce_none,
  6, 17, :_reduce_5,
  5, 17, :_reduce_6,
  3, 19, :_reduce_7,
  1, 19, :_reduce_8,
  3, 18, :_reduce_9,
  4, 20, :_reduce_10,
  3, 20, :_reduce_11,
  4, 20, :_reduce_12,
  2, 20, :_reduce_13,
  1, 20, :_reduce_14,
  1, 20, :_reduce_15,
  1, 20, :_reduce_16,
  1, 20, :_reduce_17,
  1, 20, :_reduce_18,
  3, 21, :_reduce_19,
  1, 21, :_reduce_20 ]

racc_reduce_n = 21

racc_shift_n = 37

racc_token_table = {
  false => 0,
  :error => 1,
  :FN_DEF => 2,
  :ASSIGN => 3,
  "(" => 4,
  ")" => 5,
  "," => 6,
  :EOL => 7,
  :FALSE => 8,
  :ID => 9,
  :NUMBER => 10,
  :STRING => 11,
  :TRUE => 12,
  :VAR_REF => 13 }

racc_nt_base = 14

racc_use_result_var = true

Racc_arg = [
  racc_action_table,
  racc_action_check,
  racc_action_default,
  racc_action_pointer,
  racc_goto_table,
  racc_goto_check,
  racc_goto_default,
  racc_goto_pointer,
  racc_nt_base,
  racc_reduce_table,
  racc_token_table,
  racc_shift_n,
  racc_reduce_n,
  racc_use_result_var ]

Racc_token_to_s_table = [
  "$end",
  "error",
  "FN_DEF",
  "ASSIGN",
  "\"(\"",
  "\")\"",
  "\",\"",
  "EOL",
  "FALSE",
  "ID",
  "NUMBER",
  "STRING",
  "TRUE",
  "VAR_REF",
  "$start",
  "code",
  "def",
  "fn_def",
  "var_def",
  "fn_def_args",
  "exp",
  "fn_call_args" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

# reduce 1 omitted

# reduce 2 omitted

# reduce 3 omitted

# reduce 4 omitted

module_eval(<<'.,.,', 'code_section.y', 24)
  def _reduce_5(val, _values, result)
     @code_section.def_function(val[0], val[2], val[3])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 25)
  def _reduce_6(val, _values, result)
     @code_section.def_function(val[0], [], val[3])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 27)
  def _reduce_7(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 28)
  def _reduce_8(val, _values, result)
     result = val[0]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 30)
  def _reduce_9(val, _values, result)
     @code_section.def_variable(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 32)
  def _reduce_10(val, _values, result)
     result = Language::FunctionCall.new(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 33)
  def _reduce_11(val, _values, result)
     result = Language::FunctionCall.new(val[0], [])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 34)
  def _reduce_12(val, _values, result)
     result = Language::FunctionCall.new(val[0], [val[2]])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 35)
  def _reduce_13(val, _values, result)
     result = Language::Variable.new(val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 36)
  def _reduce_14(val, _values, result)
     result = Language::String.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 37)
  def _reduce_15(val, _values, result)
     result = Language::Number.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 38)
  def _reduce_16(val, _values, result)
     result = Language::Boolean.new(true)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 39)
  def _reduce_17(val, _values, result)
     result = Language::Boolean.new(false)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 40)
  def _reduce_18(val, _values, result)
     result = val[0]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 42)
  def _reduce_19(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 43)
  def _reduce_20(val, _values, result)
     result = val[0]
    result
  end
.,.,

def _reduce_none(val, _values, result)
  val[0]
end

    end   # class CodeSectionParser
  end   # module Language
end   # module CSVPlusPlus
