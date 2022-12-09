enum Direction:
  case Left, Right, Up, Down

object Day09 {
  def solve = {
    val input = Util.readResource("Day09/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : List[String]) : Int = {
    val commands = parseCommands(input)
    applyCommands(commands,  List.fill(2)((0, 0)))
      .map(_.last)
      .distinct
      .size
  }

  def part2(input : List[String]) : Int = {
    val commands = parseCommands(input)
    applyCommands(commands,  List.fill(10)((0, 0)))
      .map(_.last)
      .distinct
      .size
  }

  def applyCommands(commands : List[Direction], positions : List[(Int, Int)]) : List[List[(Int, Int)]] = {
    commands.foldLeft(List(positions)) { (movements, direction) =>
      val currentPosition = movements.head
      val head = currentPosition.head
      val newHeadPosition = move(direction, head)
      val newPositions = currentPosition.tail.foldLeft(List(newHeadPosition))((rope, curr) => {
        rope :+ moveKnot(rope.last, curr)
      })
      newPositions :: movements
    }
  }

  def moveKnot(head : (Int, Int), knot : (Int, Int)) : (Int, Int) = {
    val xDis = (head._1 - knot._1).abs
    val yDis = (head._2 - knot._2).abs
    // not far enough away to move the knot
    // Chebyshev distance
    if (xDis.max(yDis) <= 1) {
      return knot
    }

    val newX = compareAndMove(knot._1, head._1)
    val newY = compareAndMove(knot._2, head._2)
    (newX, newY)
  }

  def compareAndMove(a : Int, b : Int) : Int = {
    if (a > b) {
      a - 1
    } else if (a < b) {
      a + 1
    } else {
      a
    }
  }

  def move(direction : Direction, point : (Int, Int)) : (Int, Int) = {
    direction match {
      case Direction.Left => (point._1 - 1, point._2)
      case Direction.Right => (point._1 + 1, point._2)
      case Direction.Up => (point._1, point._2 + 1)
      case Direction.Down => (point._1, point._2 - 1)
    }
  }

  def parseCommands(input : List[String]) : List[Direction] = {
    input.flatMap(line => {
      val (direction, amount) = line.splitAt(1)
      1.to(amount.trim.toInt).map(idx => {
        direction.trim match {
          case "R" => Direction.Right
          case "U" => Direction.Up
          case "L" => Direction.Left
          case "D" => Direction.Down
          case _ => throw new Exception(s"what? $direction $amount")
        }
      })
    })
  }
}
