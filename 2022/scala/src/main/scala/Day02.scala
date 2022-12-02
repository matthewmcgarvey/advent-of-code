import scala.collection.mutable.ListBuffer

object Day02 {
  def solve = {
    val input = Util.readResource("Day02/example.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : Seq[String]) : Int = {
    input.map(line => (line.charAt(0) - 'A', line.charAt(2) - 'X'))
      .map(game => (game._1 + 1, game._1 + 1))
      .map(game => (play(game) + game._2))
      .sum
  }

  def part2(input : Seq[String]) : Int = {
    input.map(line => (line.charAt(0) - 'A', line.charAt(2) - 'X'))
      .map(game => (game._1 + 1, toMove(game._1 + 1, game._2)))
      .map(game => (play(game) + game._2))
      .sum
  }

  // rock is 1
  // paper is 2
  // scissors is 3
  // we want right side to win
  def play(game : (Int, Int)) : Int = {
    val left = game._1
    val right = game._2
    if (left == right)
      3
    else if ((left % 3 + 1) == right)
      6
    else
      0
  }

  def toMove(left : Int, right : Int) : Int = {
    if (right == 0)
      (left - 1) % 3
    else if (right == 1)
      left
    else
      left % 3 + 1
  }
}
