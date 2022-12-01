import scala.io.Source

object Util {
  def readResource(resource: String): Seq[String] =
    Source.fromInputStream(getClass.getClassLoader.getResourceAsStream(resource)).getLines().toSeq
}
