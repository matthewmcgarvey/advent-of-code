require "spec_helper"
include AdventOfCode

RSpec.describe Day4 do
  it 'is a class' do
    expect(described_class).to be_a Class
  end

  describe '.valid_passphrase?' do
    it 'returns true for `aa bb cc dd ee`' do
      expect(Day4.valid_passphrase?("aa bb cc dd ee")).to be true
    end

    it 'returns false for `aa bb cc dd aa`' do
      expect(Day4.valid_passphrase?("aa bb cc dd aa")).to be false
    end

    it 'returns true for `aa bb cc dd aaa`' do
      expect(Day4.valid_passphrase?("aa bb cc dd aaa")).to be true
    end

    it 'can solve can solve the part 1 problem' do
      expect(Day4.part_1).to eq(477)
    end
  end

  describe '.valid_non_anagram?' do
    it 'returns true for `abcde fghij`' do
      expect(Day4.valid_non_anagram?("abcde fghij")).to be true
    end

    it 'returns false for `abcde xyz ecdab`' do
      expect(Day4.valid_non_anagram?("abcde xyz ecdab")).to be false
    end

    it 'can solve can solve the part 2 problem' do
      expect(Day4.part_2).to eq(167)
    end
  end
end