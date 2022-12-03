import scala.collection.mutable.ListBuffer

object Day03 {
  def solve = {
    val input = Util.readResource("Day03/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : Seq[String]) : Int = {
    val priorityLists = input.map(_.chars().toArray().map(toPriority))
    priorityLists.map(priorityList => {
      val left = priorityList.take(priorityList.size / 2)
      val right = priorityList.takeRight(priorityList.size / 2)
      left.intersect(right).last
    }).sum
  }

  def part2(input : Seq[String]) : Int = {
    input.grouped(3)
      .map(_.reduce((a, b) => a.intersect(b)).last)
      .map(char => toPriority(char.toInt))
      .sum
  }

  // 'a' == 97
  // 'A' == 65
  def toPriority(char : Int) : Int = {
    if (char > 96)
      char - 96
    else
      char - 38
  }
}
