# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Boolean do
  subject(:entity) { described_class.new('true') }

  describe '#initialize' do
    it 'converts to a boolean' do
      expect(subject.value).to(be(true))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:boolean_true))) }

    it { is_expected.not_to(eq(build(:boolean_false))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#boolean?' do
    it { is_expected.to(be_boolean) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('TRUE')) }
  end
end