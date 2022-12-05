object Day05 {
  def solve = {
    val input = Util.readResource("Day05/input.txt")
    println(part1(input))
    println(part2(input))
  }

  def part1(input : Seq[String]) : String = {
    val splitIdx = input.indexOf("")
    val state = parseState(input.take(splitIdx))
    val commands = parseCommands(input.drop(splitIdx + 1))
    val temp = commands.foldLeft(state) ((state, command) => {
      val amount = command._1
      val from = command._2
      val to = command._3
      applyCommand(state, amount, from, to)
    })

    temp.flatten(_.headOption).mkString
  }

  def part2(input : Seq[String]) : String = {
    val splitIdx = input.indexOf("")
    val state = parseState(input.take(splitIdx))
    val commands = parseCommands(input.drop(splitIdx + 1))
    val temp = commands.foldLeft(state) ((state, command) => {
      val amount = command._1
      val from = command._2
      val to = command._3
      applyCommand2(state, amount, from, to)
    })

    temp.flatten(_.headOption).mkString
  }

  def parseState(input : Seq[String]) : List[List[Char]] = {
    input
      .transpose
      .filter(_.last.isDigit)
      .map(_.filterNot(_.isWhitespace).dropRight(1).toList)
      .toList
  }

  def parseCommands(input : Seq[String]) : List[(Int, Int, Int)] = {
    val regex = "move (\\d+) from (\\d+) to (\\d+)".r
    input.toList
      .flatten(line => regex.findFirstMatchIn(line))
      .map(matches => (matches.group(1).toInt, matches.group(2).toInt - 1, matches.group(3).toInt - 1))
  }

  def applyCommand(state : List[List[Char]], quantity : Int, from : Int, to : Int) : List[List[Char]] = {
    val stack = state(from)
    if (stack.isEmpty) {
      return state;
    }
    val head = stack.head
    val tail = stack.tail
    var idx = -1
    val newState = state.map(stack => {
      idx += 1
      if (idx == from) {
        tail
      } else if (idx == to) {
        head :: stack
      } else {
        stack
      }
    })
    if (quantity == 1) {
      newState
    } else {
      applyCommand(newState, quantity - 1, from, to)
    }
  }

  def applyCommand2(state : List[List[Char]], quantity : Int, from : Int, to : Int) : List[List[Char]] = {
    val stack = state(from)
    if (stack.isEmpty) {
      return state;
    }
    val head = stack.take(quantity)
    val tail = stack.drop(quantity)
    var idx = -1
    state.map(stack => {
      idx += 1
      if (idx == from) {
        tail
      } else if (idx == to) {
        head ++ stack
      } else {
        stack
      }
    })
  }
}
