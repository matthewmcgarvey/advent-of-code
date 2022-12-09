type Trees = List[List[Int]]

object Day08 {
  def solve = {
    val input = Util.readResource("Day08/input.txt").toList
    println(part1(input))
    println(part2(input))
  }

  def part1(input : List[String]) : Int = {
    val trees = toTrees(input)
    visibleTrees(trees).size
  }

  def part2(input : List[String]) : Int = {
    val trees = toTrees(input)
    val visibleTreesList = visibleTrees(trees)
    scenicScores(visibleTreesList, trees).max
  }

  def scenicScores(visibleTreesList : List[(Int, Int)], trees : Trees) : List[Int] = {
    visibleTreesList.map(scenicScore(_, trees))
  }

  def scenicScore(tree : (Int, Int), trees : Trees) : Int = {
    val left = trees(tree._1).take(tree._2).reverse
    val right = trees(tree._1).drop(tree._2 + 1)
    val top = trees.map(_(tree._2)).take(tree._1).reverse
    val bottom = trees.map(_(tree._2)).drop(tree._1 + 1)
    val treeHeight = trees(tree._1)(tree._2)

    val leftScore = directionalScenicScore(treeHeight, left)
    val rightScore = directionalScenicScore(treeHeight, right)
    val topScore = directionalScenicScore(treeHeight, top)
    val bottomScore = directionalScenicScore(treeHeight, bottom)

    leftScore * rightScore * topScore * bottomScore
  }

  def directionalScenicScore(height : Int, treeLine : List[Int]) : Int = {
    var score = 0
    treeLine.takeWhile(treeHeight => {
      score += 1
      treeHeight < height
    })
    score
  }

  def toTrees(lines : List[String]) : Trees = {
    lines.map(_.split("").toList.map(_.toInt))
  }

  def visibleTrees(trees : Trees) : List[(Int, Int)] = {
    var idX = -1
    var idY = -1
    trees.flatMap(row => {
      idX += 1
      idY = -1
      row.flatMap(col => {
        idY +=1
        val location = (idX, idY)
        val isVisible = checkLeft(trees, location, col)
          || checkRight(trees, location, col)
          || checkTop(trees, location, col)
          || checkBottom(trees, location, col)
        if isVisible then Some(location) else None
      })
    })
  }

  def checkLeft(trees : Trees, point : (Int, Int), height : Int) : Boolean = {
    trees(point._1)
      .take(point._2)
      .forall(otherHeight => otherHeight < height)
  }

  def checkRight(trees : Trees, point : (Int, Int), height : Int) : Boolean = {
    trees(point._1)
      .drop(point._2 + 1)
      .forall(otherHeight => otherHeight < height)
  }

  def checkTop(trees : Trees, point : (Int, Int), height : Int) : Boolean = {
    trees.map(row => row(point._2))
      .take(point._1)
      .forall(otherHeight => otherHeight < height)
  }

  def checkBottom(trees : Trees, point : (Int, Int), height : Int) : Boolean = {
    trees.map(row => row(point._2))
      .drop(point._1 + 1)
      .forall(otherHeight => otherHeight < height)
  }
}
