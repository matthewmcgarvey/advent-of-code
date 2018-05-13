module AdventOfCode
  class Day4
    def self.valid_passphrase?(passphrase)
      words = passphrase.split
      words.uniq.length == words.length
    end

    def self.part_1
      File.open("resources/day4.txt")
        .map { |line| line[0..-2] }
        .map { |passphrase| Day4.valid_passphrase?(passphrase) }
        .select(&:itself)
        .length
    end

    def self.valid_non_anagram?(passphrase)
      words = passphrase.split
      unless words.uniq.length == words.length
        return false
      end

      letters = words.map { |word| word.split("").sort }
      letters.uniq.length == letters.length
    end

    def self.part_2
      File.open("resources/day4.txt")
        .map { |line| line[0..-2] }
        .map { |passphrase| Day4.valid_non_anagram?(passphrase) }
        .select(&:itself)
        .length
    end
  end
end