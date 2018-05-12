module AdventOfCode
  class Day2
    def self.high_low(spreadsheet)
      Day2.checksum(spreadsheet) do |row|
        sorted = row.sort
        sorted.last - sorted.first
      end
    end

    def self.even_division(spreadsheet)
      Day2.checksum(spreadsheet) do |row|
        high_to_low = row.sort.reverse

        high_to_low.combination(2)
          .to_a
          .select { |a,b| a % b == 0 }
          .map { |a,b| a / b }
          .first
      end
    end

    private
    def self.checksum(spreadsheet)
      spreadsheet.map { |row| yield row }.sum
    end
  end
end