class Day3
{
    private void Part1(List<string> inputLines)
    {
        List<List<char>> grid = AocUtil.LinesToGrid(inputLines);
        int xLen = grid[0].Count;
        int yLen = grid.Count;

        var pos = (0, 0);
        int treeCount = 0;

        int dx = 3;
        int dy = 1;

        while (pos.Item2 < yLen)
        {
            pos.Item1 += dx;
            pos.Item2 += dy;

            int relX = pos.Item1 % xLen;
            int relY = pos.Item2 % yLen;
            char gridItem = grid[relY][relX];

            if (gridItem == '#')
            {
                treeCount++;
            }
        }

        Console.WriteLine(treeCount);
    }

    private void Part2(List<string> inputLines)
    {
        List<List<char>> grid = AocUtil.LinesToGrid(inputLines);
        int xLen = grid[0].Count;
        int yLen = grid.Count;
        var steps = new List<ValueTuple<int, int>> { (1, 1), (3, 1), (5, 1), (7, 1),
                                                 (1, 2) };
        int product = 1;

        foreach (var step in steps)
        {
            var pos = (0, 0);
            int treeCount = 0;

            while (pos.Item2 < yLen)
            {
                pos.Item1 += step.Item1;
                pos.Item2 += step.Item2;

                int relX = pos.Item1 % xLen;
                int relY = pos.Item2 % yLen;
                char gridItem = grid[relY][relX];

                if (gridItem == '#')
                {
                    treeCount++;
                }
            }

            product *= treeCount;
        }

        Console.WriteLine(product);
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day3/in.txt");
        Part1(inputLines);
        Part2(inputLines);
    }
}
