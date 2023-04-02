# typed: strict

module CSVPlusPlus
  module Entities
    class Entity
      sig { returns(T::Boolean) }
      def boolean?; end

      sig { returns(T::Boolean) }
      def cell_reference?; end

      sig { returns(T::Boolean) }
      def date?; end

      sig { returns(T::Boolean) }
      def function?; end

      sig { returns(T::Boolean) }
      def function_call?; end

      sig { returns(T::Boolean) }
      def number?; end

      sig { returns(T::Boolean) }
      def runtime_value?; end

      sig { returns(T::Boolean) }
      def string?; end

      sig { returns(T::Boolean) }
      def variable?; end
    end
  end

  module Parser
    class CellValue
      sig { void }
      def initialize; end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end

    class CodeSection
      sig { void }
      def initialize; end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end

    class Modifier
      sig { params(cell_modifier: CSVPlusPlus::Modifier::Modifier, row_modifier: CSVPlusPlus::Modifier::Modifier).void }
      def initialize(cell_modifier:, row_modifier:); end

      sig { params(input: ::String, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      def parse(input, runtime); end
    end
  end
end

