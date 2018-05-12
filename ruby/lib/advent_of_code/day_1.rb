module AdventOfCode
  class Day1
    def self.captcha(captcha)
      total = 0
      digits = captcha.chars
      digits << digits.first
      digits.each_cons(2).to_a
        .select { |a, b| a == b }
        .map(&:first)
        .map(&:to_i)
        .reduce(0, &:+)
    end

    # checking for equality half way around is same as comparing each element
    # of array to itself after it has been split in half and the two pieces swapped
    # 1221 -> 2112
    def self.captcha_two(captcha)
      digits = captcha.chars
      second = digits.each_slice( (digits.size/2).round )
        .to_a
        .reverse
        .flatten
      digits.zip(second)
        .select { |a, b| a == b}
        .map(&:first)
        .map(&:to_i)
        .reduce(0, &:+)
    end
  end
end