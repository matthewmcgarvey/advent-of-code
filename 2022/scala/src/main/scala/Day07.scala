import scala.collection.mutable.ListBuffer
import scala.collection.mutable.ListBuffer;

abstract class TerminalLine
case class ChangeDirectory(location : String) extends TerminalLine
case class ListContents() extends TerminalLine
case class DirectoryListing(name : String) extends TerminalLine
case class FileListing(name : String, size : Int) extends TerminalLine

abstract class Node {
  def size : Int
  def name : String
}
case class DirectoryNode(name : String, contents : ListBuffer[Node] = ListBuffer(), parent : Option[DirectoryNode] = None) extends Node {
  override def size : Int = contents.map(_.size).sum

  def getDirectory(name : String) : DirectoryNode = {
    if (name == "..") {
      return parent.getOrElse(this)
    }
    contents.find(node => node.name == name).get.asInstanceOf[DirectoryNode]
  }
}
case class FileNode(name : String, size : Int) extends Node

object Day07 {
  def solve = {
    val input = Util.readResource("Day07/input.txt").toList
    println(part1(input))
    println(part2(input))
  }

  def part1(input : List[String]) : Int = {
    val lines = input.map(wrapLine)
    val tree = buildTree(lines)
    getFlattenedDirs(tree)
      .map(_.size)
      .filter(_ <= 100_000)
      .sum
  }

  def part2(input : List[String]) : Int = {
    val lines = input.map(wrapLine)
    val tree = buildTree(lines)
    val neededSpace = 30_000_000 -(70_000_000 - tree.size)
    getFlattenedDirs(tree)
      .sortBy(_.size)
      .find(_.size >= neededSpace)
      .map(_.size)
      .get
  }

  def getFlattenedDirs(dir : DirectoryNode) : List[DirectoryNode] = {
    dir :: dir.contents.collect { case t: DirectoryNode => t }.flatMap(getFlattenedDirs).toList
  }

  def buildTree(lines : List[TerminalLine]) : DirectoryNode = {
    val rootDir = DirectoryNode("/")
    var currentDir = rootDir
    val rest = lines.tail
    rest.foreach(line => {
      line match {
        case DirectoryListing(name) => currentDir.contents += DirectoryNode(name, parent = Some(currentDir))
        case FileListing(name, size) => currentDir.contents += FileNode(name, size)
        case ChangeDirectory(location) => currentDir = currentDir.getDirectory(location)
        case _ => // don't care at all about ListContents
      }
    });
    return rootDir
  }

  def wrapLine(line : String) : TerminalLine = {
    val cd_pattern = """\$ cd (.+)""".r
    val dir_listing_pattern = "dir (.+)".r
    val file_listing_pattern = "(\\d+) (.+)".r
    line match {
      case cd_pattern(dir) => ChangeDirectory(dir.toString())
      case "$ ls" => ListContents()
      case dir_listing_pattern(name) => DirectoryListing(name.toString())
      case file_listing_pattern(size, name) => FileListing(name.toString(), size.toInt)
      case _ => throw new Exception(s"unable to handle $line")
    }
  }
}
