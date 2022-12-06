object Day06 {
  def solve = {
    val input = Util.readResource("Day06/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : Seq[String]) : Int = {
    val buffer = input.head
    var idx = 3
    buffer.sliding(4).find(temp => {
      idx += 1
      temp.distinct.size == 4
    })
    idx
  }

  def part2(input : Seq[String]) : Int = {
    val buffer = input.head
    var idx = 13
    buffer.sliding(14).find(temp => {
      idx += 1
      temp.distinct.size == 14
    })
    idx
  }
}
