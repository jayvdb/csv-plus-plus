require 'ast'
require 'syntax_error'

describe CSVPlusPlus::AST do
  describe "::variable_references" do
    let(:ast) { [[:fn, "MULTIPLY$$"], [[:literal, "C$$rownum"], [:literal, "$$foo"]]] }

    subject { CSVPlusPlus::AST::variable_references ast }

    it { should eq(["foo"]) }
  end

  describe "::interpolate_variables" do
    let(:ast) { [[:fn, "MULTIPLY$$"], [[:literal, "$$rownum"], [:literal, "$$foo"]]] }
    let(:variables) {
      {
        "rownum" => [:literal, "1"],
        "foo" => [[:fn, "ADD"], [[:literal, "5"], [:literal, "42"]]],
        "dep" => [[:fn, "ADD"], [[:literal, "$$rownum"], [:literal, "42"]]],
      }
    }

    subject { CSVPlusPlus::AST::interpolate_variables(ast, variables) }

    it { should eq([[:fn, "MULTIPLY$$"], [variables["rownum"], variables["foo"]]]) }

    context "with undefined variables" do
      let(:variables) {
        {
          "foo" => [:literal, "$$thisdoesnotexist"],
        }
      }

      it "should raise a SyntaxError" do
        expect { subject }.to raise_error(CSVPlusPlus::SyntaxError)
      end
    end
  end

  describe "::interpolate_variable" do
    let(:ast) { [[:fn, "MULTIPLY$$"], [[:literal, "$$rownum"], [:literal, "$$foo"]]] }

    subject { CSVPlusPlus::AST::interpolate_variable(ast, "rownum", [:literal, "1"]) }

    it { should eq([[:fn, "MULTIPLY$$"], [[:literal, "1"], [:literal, "$$foo"]]]) }
  end

  describe "::copy_tree" do
    let(:ast) { [[:fn, "MULTIPLY$$"], [[:literal, "$$rownum"], [:literal, "$$foo"]]] }

    subject { CSVPlusPlus::AST::copy_tree(ast) {|n| n} }

    it { should eq(ast) }
  end

  describe "::depth_first_search" do
    let(:ast) { [[:fn, "MULTIPLY"], [[:literal, "5"], [:literal, "5"]]] }

    subject { AST::dfs ast }

    it "accumulates each value returned by the block" do
      expect(CSVPlusPlus::AST::depth_first_search(ast) {|n| 1 }).to eq([1, 1, 1, 1, 1])
    end

    context "with a literal" do
      let(:ast) { [:literal, "5"] }

      it "yields the literal" do
        expect {|block|
          CSVPlusPlus::AST::depth_first_search(ast, &block)
        }.to yield_successive_args([:literal, "5"])
      end
    end

    context "a function call" do
      let(:ast) { [[:fn, "MULTIPLY"], [[:literal, "5"], [:literal, "5"]]] }

      it "yields the function and arguments in order" do
        expect {|block|
          CSVPlusPlus::AST::depth_first_search(ast, &block)
        }.to yield_successive_args(
          [:before_fn],
          [:fn, "MULTIPLY"],
          [:literal, "5"],
          [:literal, "5"],
          [:after_fn],
        )
      end
    end
  end

  describe "::topological_sort" do
    let(:dependencies) { CSVPlusPlus::AST::GraphHash[[
      ["a", ["b", "c"]],
      ["b", ["c"]],
      ["c", ["d"]],
      ["d", []]
    ]] }

    subject { CSVPlusPlus::AST::topological_sort(dependencies) }

    it "orders the keys by their required resolution order" do
      expect(subject).to eq(["d", "c", "b", "a"])
    end

    context "when it can't be resolved" do
      let(:dependencies) { CSVPlusPlus::AST::GraphHash[[
        ["a", ["b"]],
        ["b", ["a"]],
      ]] }

      it "orders the keys by their required resolution order" do
        expect { subject }.to raise_error(TSort::Cyclic)
      end
    end
  end
end