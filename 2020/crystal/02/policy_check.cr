class PasswordPolicy
  def self.parse(policy_str)
    temp = policy_str.split(" ")
    range = temp[0]
    character = temp[1].chars[0]
    temp2 = range.split("-")
    new(pos1: temp2[0].to_i, pos2: temp2[1].to_i, character: character)
  end

  private getter pos1 : Int32
  private getter pos2 : Int32
  private getter character : Char

  def initialize(@pos1, @pos2, @character)
  end

  def valid?(password)
    (password[pos1] == character) ^ (password[pos2] == character)
  end
end

content = File.read("./input.txt")
puts content.lines.map { |line| line.split(":") }.select { |entry| PasswordPolicy.parse(entry[0]).valid?(entry[1]) }.size
