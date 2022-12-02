import scala.collection.mutable.ListBuffer

object Day01 {
  def solve = {
    val temp = Util.readResource("Day01/input.txt")
    val arrs = ListBuffer(ListBuffer.empty[Int])
    temp.foreach(item => {
      if (item.isBlank()) {
        arrs += ListBuffer.empty[Int]
      } else {
        arrs.last += item.toInt
      }
    })
    val calsPerElf = arrs.toList.map(_.sum).sorted.reverse.take(3).sum
    println(calsPerElf)
  }
}
