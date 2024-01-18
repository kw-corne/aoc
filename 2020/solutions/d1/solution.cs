class Day1 {
  private const int TARGET = 2020;

  private void Part1(List<string> inputLines) {
    var seen = new HashSet<int>();

    foreach (string num in inputLines) {
      int number = int.Parse(num);
      int need = TARGET - number;

      if (seen.Contains(need)) {
        Console.WriteLine(number * need);
        break;
      }

      seen.Add(number);
    }
  }

  private void Part2(List<string> inputLines) {
    var seen = new HashSet<int>();

    for (int i = 0; i < inputLines.Count; i++) {
      for (int j = i + 1; j < inputLines.Count; j++) {
        var n1 = int.Parse(inputLines[i]);
        var n2 = int.Parse(inputLines[j]);
        var need = TARGET - n1 - n2;

        if (seen.Contains(need)) {
          Console.WriteLine(need * n1 * n2);
          break;
        }

        seen.Add(n1);
      }
    }
  }

  public void solve() {
    var inputLines = AocUtil.GetLines("solutions/d1/in.txt");
    Part1(inputLines);
    Part2(inputLines);
  }
}
