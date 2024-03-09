class Day6
{
    private void Part1(List<string> inputLines)
    {
        string[] groups = String.Join("\n", inputLines).Split("\n\n");
        int sum = 0;

        foreach (string group in groups)
        {
            var uniqueChars = group.Replace("\n", "").ToHashSet();
            sum += uniqueChars.Count;
        }

        Console.WriteLine(sum);
    }

    private void Part2(List<string> inputLines)
    {
        string[] groups = String.Join("\n", inputLines).Split("\n\n");
        int sum = 0;

        foreach (string group in groups)
        {
            var splitGroups = group.Split("\n");
            var possible = splitGroups[0].ToHashSet();

            if (splitGroups.Length > 1)
            {
                foreach (var splitGroup in splitGroups.Skip(1))
                {
                    var possibleThisGroup = splitGroup.ToHashSet();
                    possible.IntersectWith(possibleThisGroup);
                }
            }

            sum += possible.Count;
        }

        Console.WriteLine(sum);
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day6/in.txt");
        Part1(inputLines);
        Part2(inputLines);
    }
}
