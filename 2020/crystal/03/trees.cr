class World
  private getter rows : Array(String)
  private property x = 0
  private property y = 0

  def initialize(@rows)
  end

  def move(move_x, move_y)
    temp_x = self.x + move_x
    if temp_x >= x_width
      temp_x -= x_width
    end
    self.x = temp_x

    self.y += move_y
  end

  def bottom?
    self.y == (y_height - 1)
  end

  def tree?
    object_at_pos == '#'
  end

  private def object_at_pos
    rows[y][x]
  end

  private def x_width
    rows.first.size
  end

  private def y_height
    rows.size
  end
end

content = File.read("./input.txt")

world = World.new(content.lines)

tree_count = 0

loop do
  world.move(move_x: 3, move_y: 1)
  tree_count += 1 if world.tree?

  break if world.bottom?
end

puts tree_count
