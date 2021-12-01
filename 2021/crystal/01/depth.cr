require "file"

def part_one(lines : Array(Int32))
  variations = lines.map_with_index do |item, idx|
    if idx == 0
      0
    else
      item - lines[idx - 1]
    end
  end

  puts "PART ONE: #{variations.select(&.positive?).size}"
end

def part_two(lines : Array(Int32))
  a = lines.clone
  b = a.clone
  b.shift
  c = b.clone
  c.shift
  a.pop(2)
  b.pop

  windowed = a.zip(b, c).map(&.sum)
  variations = windowed.map_with_index do |item, idx|
    if idx == 0
      0
    else
      item - windowed[idx - 1]
    end
  end

  puts "PART TWO: #{variations.select(&.positive?).size}"
end

lines = File.read_lines(File.join(__DIR__, "input.txt")).map(&.strip.to_i)
part_two(lines)
