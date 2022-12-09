import scala.collection.immutable.Range.Inclusive
object Day04 {
  def solve = {
    val input = Util.readResource("Day04/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : List[String]) : Int = {
    input.map(_.split(",").map(toRange))
      .filter(pair => completeOverlap(pair(0), pair(1)))
      .size
  }

  def part2(input : List[String]) : Int = {
    input.map(_.split(",").map(toRange))
      .filter(pair => anyOverlap(pair(0), pair(1)))
      .size
  }

  def toRange(str : String) : Inclusive = {
    val list = str.split("-").map(_.toInt)
    list(0).to(list(1))
  }

  def completeOverlap(range1 : Inclusive, range2 : Inclusive) : Boolean = {
   val intersection = range1.intersect(range2)
   intersection == range1 || intersection == range2
  }

  def anyOverlap(range1 : Inclusive, range2 : Inclusive) : Boolean = {
   range1.intersect(range2).nonEmpty
  }
}
