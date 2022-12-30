#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.0
# from Racc grammar file "".
#

require 'racc/parser.rb'

require 'strscan'

module CSVPlusPlus
  class CellValueParser < Racc::Parser

module_eval(<<'...end cell_value_parser.y/module_eval...', 'cell_value_parser.y', 30)
  attr_accessor :ast

  def parse(text)
    return nil unless text.strip.start_with?('=')
    tokens = []

    s = StringScanner.new text
    until s.empty?
      case
      when s.scan(/\s+/)
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
      when s.scan(/[\(\)\/\*\+\-,=]/) 
        tokens << [s.matched, s.matched]
      else
        raise "Unable to parse starting at: <#{s.peek 100}>"
      end 
    end
    tokens << [:EOL, :EOL]

    define_singleton_method(:next_token) { tokens.shift }

    do_parse
 
    @ast
  end
...end cell_value_parser.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
    28,    14,    15,    16,    17,     6,    13,    14,    15,    16,
    17,     5,    29,     9,     8,    10,    11,     6,    27,    14,
    15,    16,    17,     5,     6,     9,     8,    10,    11,     2,
     5,     6,     9,     8,    10,    11,     3,     5,     6,     9,
     8,    10,    11,    12,     5,     6,     9,     8,    10,    11,
    18,     5,   nil,     9,     8,    10,    11,     6,    25,    14,
    15,    16,    17,     5,     6,     9,     8,    10,    11,   nil,
     5,   nil,     9,     8,    10,    11,    14,    15,    14,    15 ]

racc_action_check = [
    24,     4,     4,     4,     4,     2,     4,    26,    26,    26,
    26,     2,    24,     2,     2,     2,     2,     6,    19,    19,
    19,    19,    19,     6,    14,     6,     6,     6,     6,     0,
    14,    15,    14,    14,    14,    14,     1,    15,    16,    15,
    15,    15,    15,     3,    16,    17,    16,    16,    16,    16,
     5,    17,   nil,    17,    17,    17,    17,    18,    18,    30,
    30,    30,    30,    18,    29,    18,    18,    18,    18,   nil,
    29,   nil,    29,    29,    29,    29,    22,    22,    23,    23 ]

racc_action_pointer = [
    15,    36,     3,    43,    -3,    48,    15,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,    22,    29,    36,    43,    55,    15,
   nil,   nil,    72,    74,    -3,   nil,     3,   nil,   nil,    62,
    55 ]

racc_action_default = [
   -17,   -17,   -17,   -17,   -17,   -16,   -17,    -9,   -12,   -13,
   -14,   -15,    31,    -1,   -17,   -17,   -17,   -17,   -17,   -17,
    -4,    -5,    -6,    -7,   -17,    -3,   -11,    -8,    -2,   -17,
   -10 ]

racc_goto_table = [
     4,     1,    24,   nil,    19,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,    20,    21,    22,    23,    26,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,    30 ]

racc_goto_check = [
     2,     1,     3,   nil,     2,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,     2,     2,     2,     2,     2,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,     2 ]

racc_goto_pointer = [
   nil,     1,    -2,   -16,   nil ]

racc_goto_default = [
   nil,   nil,   nil,   nil,     7 ]

racc_reduce_table = [
  0, 0, :racc_error,
  3, 17, :_reduce_1,
  4, 18, :_reduce_2,
  3, 18, :_reduce_3,
  3, 18, :_reduce_4,
  3, 18, :_reduce_5,
  3, 18, :_reduce_6,
  3, 18, :_reduce_7,
  3, 18, :_reduce_8,
  1, 18, :_reduce_9,
  3, 19, :_reduce_10,
  1, 19, :_reduce_11,
  1, 20, :_reduce_none,
  1, 20, :_reduce_none,
  1, 20, :_reduce_none,
  1, 20, :_reduce_none,
  1, 20, :_reduce_none ]

racc_reduce_n = 17

racc_shift_n = 31

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
  "=" => 14,
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
  "\"=\"",
  "\",\"",
  "$start",
  "cell_value",
  "exp",
  "fn_call_args",
  "literal" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

module_eval(<<'.,.,', 'cell_value_parser.y', 8)
  def _reduce_1(val, _values, result)
     @ast = val[1]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 10)
  def _reduce_2(val, _values, result)
     result = [[:fn, val[0]], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 11)
  def _reduce_3(val, _values, result)
     result = [[:fn, val[0]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 12)
  def _reduce_4(val, _values, result)
     result = [[:fn, "MULTIPLY"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 13)
  def _reduce_5(val, _values, result)
     result = [[:fn, "DIVIDE"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 14)
  def _reduce_6(val, _values, result)
     result = [[:fn, "ADD"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 15)
  def _reduce_7(val, _values, result)
     result = [[:fn, "MINUS"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 16)
  def _reduce_8(val, _values, result)
     result = [:group, [val[1]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 17)
  def _reduce_9(val, _values, result)
     result = [:literal, val[0]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 19)
  def _reduce_10(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value_parser.y', 20)
  def _reduce_11(val, _values, result)
     result = val[0]
    result
  end
.,.,

# reduce 12 omitted

# reduce 13 omitted

# reduce 14 omitted

# reduce 15 omitted

# reduce 16 omitted

def _reduce_none(val, _values, result)
  val[0]
end

  end   # class CellValueParser
end   # module CSVPlusPlus