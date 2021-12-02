def part_one(lines)
  horiz = 0
  depth = 0

  lines.each do |line|
    temp = line.split(" ")
    command = temp[0]
    amount = temp[1].to_i

    case command
    when "forward"
      horiz += amount
    when "up"
      depth -= amount
    when "down"
      depth += amount
    else
    end
  end

  puts "Part one: #{horiz * depth}"
end

def part_two(lines)
  horiz = 0
  depth = 0
  aim = 0

  lines.each do |line|
    temp = line.split(" ")
    command = temp[0]
    amount = temp[1].to_i

    case command
    when "forward"
      horiz += amount
      depth += amount * aim
    when "up"
      aim -= amount
    when "down"
      aim += amount
    else
    end
  end

  puts "Part two: #{horiz * depth}"
end

lines = File.read_lines(File.join(__DIR__, "input.txt"))
part_one(lines)
part_two(lines)
