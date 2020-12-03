class PasswordPolicy
  def self.parse(policy_str)
    temp = policy_str.split(" ")
    range = temp[0]
    character = temp[1]
    temp2 = range.split("-")
    new(min: temp2[0].to_i, max: temp2[1].to_i, character: character)
  end

  private getter min : Int32
  private getter max : Int32
  private getter character : String

  def initialize(@min, @max, @character)
  end

  def valid?(password)
    occurrence_count = password.count(character)
    occurrence_count >= min && occurrence_count <= max
  end
end

content = File.read("./input.txt")
puts content.lines.map { |line| line.split(":") }.select { |entry| PasswordPolicy.parse(entry[0]).valid?(entry[1]) }.size
