# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::CellReference do
  subject(:cell_reference) { described_class.new('A1') }

  describe '.from_index' do
    subject { described_class.from_index(cell_index: 1, row_index: 1) }

    it 'returns a CellReference at that index' do
      expect(subject.cell_reference).to(eq('B2'))
    end
  end

  describe '.valid_cell_reference?' do
    let(:cell_reference_string) { 'A1:B2' }

    subject { described_class.valid_cell_reference?(cell_reference_string) }

    it { is_expected.to(be(true)) }

    context 'with a sheet name' do
      let(:cell_reference_string) { 'Sheet1!A1:B2' }

      it { is_expected.to(be(true)) }
    end

    context 'with a sheet name with quotes' do
      let(:cell_reference_string) { "'Test Sheet'!A1:B2" }

      it { is_expected.to(be(true)) }
    end

    context 'not a cell reference' do
      let(:cell_reference_string) { 'foo' }

      it { is_expected.not_to(be(true)) }
    end
  end

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.cell_reference).to(eq('A1'))
    end
  end

  describe '#to_s' do
    subject { cell_reference.to_s }

    it { is_expected.to(eq('A1')) }
  end

  describe '#cell_reference?' do
    it { is_expected.to(be_cell_reference) }
  end

  describe '#variable?' do
    it { is_expected.not_to(be_variable) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:cell_reference, ref: 'A1'))) }

    it { is_expected.not_to(eq(build(:cell_reference, ref: 'Z5'))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end
