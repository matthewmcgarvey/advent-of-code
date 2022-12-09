import scala.io.Source

object Util {
  def readResource(resource: String): List[String] =
    Source.fromInputStream(getClass.getClassLoader.getResourceAsStream(resource))
      .getLines()
      .toList
}
