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
    
  end
end