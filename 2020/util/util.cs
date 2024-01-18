class AocUtil {
  public static List<string> GetLines(string filename) {
    List<string> lines = new List<string>();

    using (StreamReader sr = new StreamReader(filename)) {
      string ? line;

      while ((line = sr.ReadLine()) != null) {
        lines.Add(line);
      }
    }

    return lines;
  }
}
