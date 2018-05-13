module AdventOfCode
  class Day3
    points = []

    def self.steps(location)
      point = [0, 0]
      add = true
      points = [[0, 0]]
      length = 1

      while points.length < location
        (1..length).each do |_i|
          point[0] = add ? point[0] + 1 : point[0] - 1
          points << Array.new(point)
        end

        (1..length).each do |_i|
          point[1] = add ? point[1] + 1 : point[1] - 1
          points << Array.new(point)
        end

        add = !add
        length += 1
      end
      target = points[location - 1]
      target.first.abs + target.last.abs
    end

    def self.square_sum(target)
      first = {
        x: 0,
        y: 0,
        sum: 1
      }
      points = [first]
      previous = first
      length = 1
      current_length = 0
      do_repeat = true
      direction = :right
      while previous[:sum] <= target
        # create current
        current = {
          x: previous[:x],
          y: previous[:y]
        }

        # update x or y based off of direction
        if direction == :right
          current[:x] += 1
        elsif direction == :left
          current[:x] -= 1
        elsif direction == :up
          current[:y] += 1
        elsif direction == :down
          current[:y] -= 1
        end

        # build array of points that surround current
        surrounding = [
          [current[:x] + 1, current[:y]],
          [current[:x] - 1, current[:y]],
          [current[:x], current[:y] + 1],
          [current[:x], current[:y] - 1],
          [current[:x] + 1, current[:y] + 1],
          [current[:x] + 1, current[:y] - 1],
          [current[:x] - 1, current[:y] + 1],
          [current[:x] - 1, current[:y] - 1]
        ]

        # find any touching points to create the sum
        sum = points.select { |point| surrounding.include? [point[:x], point[:y]] }
                    .map { |point| point[:sum] }
                    .sum
        current[:sum] = sum

        # add current to points
        points << current

        # set current to previous
        previous = current

        # add 1 to current length
        current_length += 1

        # if current length equals length, update direction and reset
        next unless current_length == length
        if direction == :right
          direction = :up
        elsif direction == :up
          direction = :left
        elsif direction == :left
          direction = :down
        elsif direction == :down
          direction = :right
        end
        current_length = 0
        length += 1 unless do_repeat
        do_repeat = !do_repeat
      end
      previous[:sum]
    end
  end
end
