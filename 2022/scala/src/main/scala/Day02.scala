import scala.collection.mutable.ListBuffer

object Day02 {
  def solve = {
    val input = Util.readResource("Day02/input.txt")
    println(part2(input))
  }

  def part1(input : Seq[String]) : Int = {
    input.map(line => (line.charAt(0), line.charAt(2)))
      .map(game => (toPoint(game._1.toInt), toPoint(game._2.toInt - 23)))
      .map(game => (play(game) + game._2))
      .sum
  }

  def part2(input : Seq[String]) : Int = {
    input.map(line => (line.charAt(0), line.charAt(2)))
      .map(game => (game._1.toInt, game._2))
      .map(game => (toPoint(game._1), game._2))
      .map(game => (game._1, toMove(game)))
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
      return 3
    else if ((left % 3 + 1) == right)
      return 6
    else
      return 0
  }

  def toMove(game : (Int, Char)) : Int = {
    if (game._2 == 'X')
      return (game._1 + 1) % 3 + 1
    else if (game._2 == 'Y')
      return game._1
    else
      return game._1 % 3 + 1
  }

  def toPoint(x : Int) : Int = {
    if (x == 65)
      1
    else if (x == 66)
      2
    else if (x == 67)
      3
    else
      -100
  }
}
