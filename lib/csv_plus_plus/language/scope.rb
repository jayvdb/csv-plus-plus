# frozen_string_literal: true

require_relative '../code_section'
require_relative '../graph'
require_relative './entities'
require_relative './references'
require_relative './syntax_error'

BUILTIN_FUNCTIONS = {
  # =CELLREF(C) === =INDIRECT(CONCAT($$C, $$rownum))
  cellref: ::CSVPlusPlus::Language::Entities::Function.new(
    :cellref,
    [:cell],
    ::CSVPlusPlus::Language::Entities::FunctionCall.new(
      :indirect,
      [
        ::CSVPlusPlus::Language::Entities::FunctionCall.new(
          :concat,
          [
            ::CSVPlusPlus::Language::Entities::Variable.new(:cell),
            ::CSVPlusPlus::Language::Entities::Variable.new(:rownum)
          ]
        )
      ]
    )
  )
}.freeze

module CSVPlusPlus
  module Language
    # A class representing the scope of the current Template and responsible for resolving variables
    #
    # @attr_reader code_section [CodeSection] The CodeSection containing variables and functions to be resolved
    # @attr_reader runtime [Runtime] The compiler's current runtime
    #
    # rubocop:disable Metrics/ClassLength
    class Scope
      attr_reader :code_section, :runtime

      # initialize with a +Runtime+ and optional +CodeSection+
      # @param runtime [Runtime]
      # @param code_section [Runtime, nil]
      def initialize(runtime:, code_section: nil)
        @code_section = code_section if code_section
        @runtime = runtime
      end

      # Resolve all values in the ast of the current cell being processed
      #
      # @return [Entity]
      def resolve_cell_value
        return unless (ast = @runtime.cell&.ast)

        last_round = nil
        loop do
          refs = ::CSVPlusPlus::Language::References.extract(ast, @code_section)
          return ast if refs.empty?

          # TODO: throw an error here instead I think - basically we did a round and didn't make progress
          return ast if last_round == refs

          ast = resolve_functions(resolve_variables(ast, refs.variables), refs.functions)
        end
      end

      # Set the +code_section+ and resolve all inner dependencies in it's variables and functions.
      #
      # @param code_section [CodeSection] The code_section to be resolved
      def code_section=(code_section)
        @code_section = code_section

        resolve_static_variables!
        resolve_static_functions!
      end

      # @return [String]
      def to_s
        "Scope(code_section: #{@code_section}, runtime: #{@runtime})"
      end

      private

      # Resolve all variable references defined statically in the code section
      def resolve_static_variables!
        variables = @code_section.variables
        last_var_dependencies = {}
        # TODO: might not need the infinite loop wrap
        loop do
          var_dependencies, resolution_order = variable_resolution_order(only_static_vars(variables))
          return if var_dependencies == last_var_dependencies

          # TODO: make the contract better here where we're not seting the variables of another class
          @code_section.variables = resolve_dependencies(var_dependencies, resolution_order, variables)
          last_var_dependencies = var_dependencies.clone
        end
      end

      def only_static_vars(var_dependencies)
        var_dependencies.reject { |k| @runtime.runtime_variable?(k) }
      end

      # Resolve all functions defined in the code section
      def resolve_static_functions!
        # TODO: I'm still torn if it's worth replacing function references
        #
        # my current theory is that if we resolve static functions before processing each cell,
        # overall compile time will be improved because there will be less to do for each cell
        #
        # though I don't think we'll ever be able to fully resolve them because they can be a mix
        # of global functions and defined functions?
      end

      def resolve_functions(ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          function_replace(acc, elem.id, resolve_function(elem.id))
        end
      end

      def resolve_variables(ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          variable_replace(acc, elem.id, resolve_variable(elem.id))
        end
      end

      # Make a copy of the AST represented by +node+ and replace +fn_id+ with +replacement+ throughout
      def function_replace(node, fn_id, replacement)
        if node.function_call? && node.id == fn_id
          apply_arguments(replacement, node)
        elsif node.function_call?
          arguments = node.arguments.map { |n| function_replace(n, fn_id, replacement) }
          ::CSVPlusPlus::Language::Entities::FunctionCall.new(node.id, arguments)
        else
          node
        end
      end

      def resolve_function(fn_id)
        id = fn_id.to_sym
        return @code_section.functions[id] if @code_section.defined_function?(id)

        # this will throw a syntax error if it doesn't exist (which is what we want)
        return ::BUILTIN_FUNCTIONS[id] if ::BUILTIN_FUNCTIONS.key?(id)
      end

      def apply_arguments(function, function_call)
        i = 0
        function.arguments.reduce(function.body.dup) do |ast, argument|
          variable_replace(ast, argument, function_call.arguments[i]).tap do
            i += 1
          end
        end
      end

      # Make a copy of the AST represented by +node+ and replace +var_id+ with +replacement+ throughout
      def variable_replace(node, var_id, replacement)
        if node.function_call?
          arguments = node.arguments.map { |n| variable_replace(n, var_id, replacement) }
          ::CSVPlusPlus::Language::Entities::FunctionCall.new(node.id, arguments)
        elsif node.variable? && node.id == var_id
          replacement
        else
          node
        end
      end

      def resolve_variable(var_id)
        id = var_id.to_sym
        return @code_section.variables[id] if @code_section.defined_variable?(id)

        # this will throw a syntax error if it doesn't exist (which is what we want)
        @runtime.runtime_value(id)
      end

      def check_unbound_vars(dependencies, variables)
        unbound_vars = dependencies.values.flatten - variables.keys
        return if unbound_vars.empty?

        @runtime.raise_syntax_error('Undefined variables', unbound_vars.map(&:to_s).join(', '))
      end

      def variable_resolution_order(variables)
        # we have a hash of variables => ASTs but they might have references to each other, so
        # we need to interpolate them first (before interpolating the cell values)
        var_dependencies = ::CSVPlusPlus::Graph.dependency_graph(variables, @runtime)
        # are there any references that we don't have variables for? (undefined variable)
        check_unbound_vars(var_dependencies, variables)

        # a topological sort will give us the order of dependencies
        [var_dependencies, ::CSVPlusPlus::Graph.topological_sort(var_dependencies)]
        # TODO: don't expose this exception directly to the caller
      rescue ::TSort::Cyclic
        @runtime.raise_syntax_error('Cyclic variable dependency detected', var_refs.keys)
      end

      def resolve_dependencies(var_dependencies, resolution_order, variables)
        {}.tap do |resolved_vars|
          # for each var and each dependency it has, build up and mutate resolved_vars
          resolution_order.each do |var|
            resolved_vars[var] = variables[var].dup

            var_dependencies[var].each do |dependency|
              resolved_vars[var] = variable_replace(resolved_vars[var], dependency, variables[dependency])
            end
          end
        end
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end
