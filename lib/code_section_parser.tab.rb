#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.0
# from Racc grammar file "".
#

require 'racc/parser.rb'

require 'strscan'

module CSVPlusPlus
  class CodeSectionParser < Racc::Parser

module_eval(<<'...end code_section_parser.y/module_eval...', 'code_section_parser.y', 34)
  attr_accessor :variables

  def parse(text)
    tokens = []
    @variables = {}

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
      when s.scan(/TRUE/)
        tokens << [:TRUE, s.matched]
      when s.scan(/FALSE/) 
        tokens << [:FALSE, s.matched]
      when s.scan(/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/)
        tokens << [:STRING, s.matched]
      when s.scan(/-?[\d.]+/)
        tokens << [:NUMBER, s.matched]
      when s.scan(/[\$\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=]/) 
        tokens << [s.matched, s.matched]
      else
        raise "Unable to parse starting at: <#{s.peek 100}>"
      end 
    end
    return @variables if tokens.empty?

    define_singleton_method(:next_token) { tokens.shift }

    do_parse
 
    @variables
  end
...end code_section_parser.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
    30,     4,    16,    17,     3,    10,    16,    17,     6,     3,
     7,     9,    31,    13,    12,    14,    15,    10,    29,    16,
    17,    18,    19,     9,    10,    13,    12,    14,    15,    20,
     9,    10,    13,    12,    14,    15,   nil,     9,    10,    13,
    12,    14,    15,   nil,     9,    10,    13,    12,    14,    15,
   nil,     9,   nil,    13,    12,    14,    15,    10,    27,    16,
    17,    18,    19,     9,    10,    13,    12,    14,    15,   nil,
     9,   nil,    13,    12,    14,    15,    16,    17,    18,    19,
    16,    17,    18,    19 ]

racc_action_check = [
    26,     1,    24,    24,     0,     6,    25,    25,     3,     1,
     4,     6,    26,     6,     6,     6,     6,    10,    21,    21,
    21,    21,    21,    10,    16,    10,    10,    10,    10,     9,
    16,    17,    16,    16,    16,    16,   nil,    17,    18,    17,
    17,    17,    17,   nil,    18,    19,    18,    18,    18,    18,
   nil,    19,   nil,    19,    19,    19,    19,    20,    20,     8,
     8,     8,     8,    20,    31,    20,    20,    20,    20,   nil,
    31,   nil,    31,    31,    31,    31,    28,    28,    28,    28,
    32,    32,    32,    32 ]

racc_action_pointer = [
    -4,     1,   nil,    -6,    10,   nil,     3,   nil,    55,    27,
    15,   nil,   nil,   nil,   nil,   nil,    22,    29,    36,    43,
    55,    15,   nil,   nil,    -2,     2,    -3,   nil,    72,   nil,
   nil,    62,    76 ]

racc_action_default = [
   -19,   -19,    -2,   -19,   -19,    -1,   -19,    33,    -3,   -18,
   -19,   -11,   -14,   -15,   -16,   -17,   -19,   -19,   -19,   -19,
   -19,   -19,    -6,    -7,    -8,    -9,   -19,    -5,   -13,   -10,
    -4,   -19,   -12 ]

racc_goto_table = [
     8,     2,     5,     1,    21,    26,   nil,   nil,   nil,   nil,
    22,    23,    24,    25,    28,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,    32 ]

racc_goto_check = [
     3,     2,     2,     1,     3,     4,   nil,   nil,   nil,   nil,
     3,     3,     3,     3,     3,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,     3 ]

racc_goto_pointer = [
   nil,     3,     1,    -6,   -15,   nil ]

racc_goto_default = [
   nil,   nil,   nil,   nil,   nil,    11 ]

racc_reduce_table = [
  0, 0, :racc_error,
  2, 17, :_reduce_none,
  1, 17, :_reduce_none,
  3, 18, :_reduce_3,
  4, 19, :_reduce_4,
  3, 19, :_reduce_5,
  3, 19, :_reduce_6,
  3, 19, :_reduce_7,
  3, 19, :_reduce_8,
  3, 19, :_reduce_9,
  3, 19, :_reduce_10,
  1, 19, :_reduce_11,
  3, 20, :_reduce_12,
  1, 20, :_reduce_13,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none ]

racc_reduce_n = 19

racc_shift_n = 33

racc_token_table = {
  false => 0,
  :error => 1,
  "(" => 2,
  ")" => 3,
  "*" => 4,
  "/" => 5,
  "+" => 6,
  "-" => 7,
  :ID => 8,
  :EOL => 9,
  :NUMBER => 10,
  :STRING => 11,
  :TRUE => 12,
  :FALSE => 13,
  :ASSIGN => 14,
  "," => 15 }

racc_nt_base = 16

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
  "\"(\"",
  "\")\"",
  "\"*\"",
  "\"/\"",
  "\"+\"",
  "\"-\"",
  "ID",
  "EOL",
  "NUMBER",
  "STRING",
  "TRUE",
  "FALSE",
  "ASSIGN",
  "\",\"",
  "$start",
  "code",
  "var",
  "exp",
  "fn_call_args",
  "literal" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

# reduce 1 omitted

# reduce 2 omitted

module_eval(<<'.,.,', 'code_section_parser.y', 12)
  def _reduce_3(val, _values, result)
     @variables[val[0]] = val[2]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 14)
  def _reduce_4(val, _values, result)
     result = [[:fn, val[0]], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 15)
  def _reduce_5(val, _values, result)
     result = [[:fn, val[0]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 16)
  def _reduce_6(val, _values, result)
     result = [[:fn, "MULTIPLY"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 17)
  def _reduce_7(val, _values, result)
     result = [[:fn, "DIVIDE"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 18)
  def _reduce_8(val, _values, result)
     result = [[:fn, "ADD"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 19)
  def _reduce_9(val, _values, result)
     result = [[:fn, "MINUS"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 20)
  def _reduce_10(val, _values, result)
     result = [:group, [val[1]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 21)
  def _reduce_11(val, _values, result)
     result = [:literal, val[0]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 23)
  def _reduce_12(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section_parser.y', 24)
  def _reduce_13(val, _values, result)
     result = val[0]
    result
  end
.,.,

# reduce 14 omitted

# reduce 15 omitted

# reduce 16 omitted

# reduce 17 omitted

# reduce 18 omitted

def _reduce_none(val, _values, result)
  val[0]
end

  end   # class CodeSectionParser
end   # module CSVPlusPlus