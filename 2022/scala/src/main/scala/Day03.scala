object Day03 {
  def solve = {
    val input = Util.readResource("Day03/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : Seq[String]) : Int = {
    input.map(knapsack => knapsack.grouped(knapsack.size / 2))
      .map(_.reduce((a, b) => a.intersect(b)).last)
      .map(toPriority)
      .sum
  }

  def part2(input : Seq[String]) : Int = {
    input.grouped(3)
      .map(_.reduce((a, b) => a.intersect(b)).last)
      .map(toPriority)
      .sum
  }

  // 'a' == 97
  // 'A' == 65
  def toPriority(char : Char) : Int = {
    if (char > 96)
      char - 96
    else
      char - 38
  }
}
