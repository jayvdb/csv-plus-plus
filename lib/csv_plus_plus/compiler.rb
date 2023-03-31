# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  # Encapsulates the parsing and building of objects (+Template+ -> +Row+ -> +Cell+). Variable resolution is delegated
  # to the +Scope+
  #
  # @attr_reader options [Options] The +Options+ to compile with
  # @attr_reader runtime [Runtime] The runtime execution
  class Compiler
    attr_reader :options, :runtime

    # Create a compiler and make sure it gets cleaned up
    #
    # @param runtime [Runtime] The initial +Runtime+ for the compiler
    # @param options [Options]
    def self.with_compiler(runtime:, options:, &block)
      compiler = new(options:, runtime:)
      if options.verbose
        ::CSVPlusPlus::BenchmarkedCompiler.with_benchmarks(compiler) do |c|
          block.call(c)
        end
      else
        yield(compiler)
      end
    ensure
      runtime.cleanup!
    end

    # @param runtime [Runtime]
    # @param options [Options]
    def initialize(runtime:, options:)
      @options = options
      @runtime = runtime

      # TODO: infer a type
      # allow user-supplied key/values to override anything global or from the code section
      @runtime.def_variables(
        options.key_values.transform_values { |v| ::CSVPlusPlus::Entities::String.new(v.to_s) }
      )
    end

    # Write the compiled results
    def outputting!
      @runtime.start_at_csv!
      yield(@runtime)
    end

    # Compile a template and return a +::CSVPlusPlus::Template+ instance ready to be written with a +Writer+
    #
    # @return [Template]
    def compile_template
      parse_code_section!
      rows = parse_csv_section!

      ::CSVPlusPlus::Template.new(rows:, runtime: @runtime).tap do |t|
        t.validate_infinite_expands(@runtime)
        expanding! { t.expand_rows! }
        bind_all_vars! { t.bind_all_vars!(@runtime) }
        resolve_all_cells!(t)
      end
    end

    protected

    # Parses the input file and sets variables on +@runtime+ as necessary
    def parse_code_section!
      @runtime.start!

      # TODO: this flow can probably be refactored, it used to have more needs back when we had to
      # parse and save the code_section
      parsing_code_section do |input|
        csv_section = ::CSVPlusPlus::Parser::CodeSection.new().yyparse(input, @runtime)

        # return the csv_section to the caller because they're gonna re-write input with it
        next csv_section
      end
    end

    # Parse the CSV section and return an array of +Row+s
    #
    # @return [Array<Row>]
    def parse_csv_section!
      @runtime.start_at_csv!
      @runtime.map_rows(::CSV.new(runtime.input)) do |csv_row|
        parse_row(csv_row)
      end
    ensure
      # we're done with the file and everything is in memory
      @runtime.cleanup!
    end

    # Iterates through each cell of each row and resolves it's variable and function references.
    #
    # @param template [Template]
    # @return [Array<Entity>]
    def resolve_all_cells!(template)
      @runtime.start_at_csv!
      @runtime.map_rows(template.rows, cells_too: true) do |cell|
        cell.ast = @runtime.resolve_cell_value if cell.ast
      end
    end

    # Expanding rows
    def expanding!
      @runtime.start_at_csv!
      yield
    end

    # Binding all [[var=]] directives
    def bind_all_vars!
      @runtime.start_at_csv!
      yield
    end

    private

    def parsing_code_section
      csv_section = yield(@runtime.input.read)
      @runtime.rewrite_input!(csv_section)
    end

    # Using the current +@runtime+ and the given +csv_row+ parse it into a +Row+ of +Cell+s
    # +csv_row+ should have already been run through a CSV parser and is an array of strings
    #
    # @param csv_row [Array<Array<String>>]
    # @return [Row]
    def parse_row(csv_row)
      row_modifier = ::CSVPlusPlus::Modifier.new(row_level: true)

      cells = @runtime.map_row(csv_row) { |value, _cell_index| parse_cell(value, row_modifier) }

      ::CSVPlusPlus::Row.new(@runtime.row_index, cells, row_modifier)
    end

    def parse_cell(value, row_modifier)
      cell_modifier = ::CSVPlusPlus::Modifier.new
      parsed_value = ::CSVPlusPlus::Parser::Modifier.new().yyparse(value, @runtime)

      ::CSVPlusPlus::Cell.parse(parsed_value, runtime:, modifier: cell_modifier)
    end
  end
end
