require 'spec_helper'
include AdventOfCode

RSpec.describe Day3 do
  it 'is a class' do
    expect(described_class).to be_a Class
  end

  describe '.steps' do
    it 'returns 0 for 1' do
      expect(Day3.steps(1)).to eq(0)
    end

    it 'returns 3 for 12' do
      expect(Day3.steps(12)).to eq(3)
    end

    it 'returns 2 for 23' do
      expect(Day3.steps(23)).to eq(2)
    end

    it 'returns 31 for 1024' do
      expect(Day3.steps(1024)).to eq(31)
    end

    it 'can solve the part 1 challenge' do
      expect(Day3.steps(361_527)).to eq(326)
    end
  end

  describe '.square_sum' do
    it 'returns 2 for 1' do
      expect(Day3.square_sum(1)).to eq(2)
    end

    it 'returns 23 for 11' do
      expect(Day3.square_sum(11)).to eq(23)
    end

    it 'returns 122 for 59' do
      expect(Day3.square_sum(59)).to eq(122)
    end

    it 'can solve the part 2 challenge' do
      expect(Day3.square_sum(361_527)).to eq(363_010)
    end
  end
end
