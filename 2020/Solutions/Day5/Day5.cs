class Day5
{
    private const int N_ROWS = 128;
    private const int N_COLS = 8;

    private int BinaryPartition(string partition, int lowerBound, int upperBound, char lh, char uh)
    {
        for (int i = 0; i < partition.Length - 1; i++)
        {
            char ch = partition[i];

            if (ch == lh)
            {
                upperBound -= (upperBound - lowerBound) / 2 + 1;
            }
            else if (ch == uh)
            {
                lowerBound += (upperBound - lowerBound) / 2 + 1;
            }
        }

        return partition[partition.Length - 1] == lh ? lowerBound : upperBound;
    }

    private void Part1(List<string> inputLines)
    {
        int maxSeatId = 0;

        foreach (string partition in inputLines)
        {
            string rowPartition = partition.Substring(0, 7);
            string columnPartition = partition.Substring(7);

            int row = BinaryPartition(rowPartition, 0, N_ROWS - 1, 'F', 'B');
            int column = BinaryPartition(columnPartition, 0, N_COLS - 1, 'L', 'R');

            int seatId = row * 8 + column;
            maxSeatId = Math.Max(seatId, maxSeatId);
        }

        Console.WriteLine(maxSeatId);
    }

    private void Part2(List<string> inputLines)
    {
        var seatIds = new SortedSet<int>();

        foreach (string partition in inputLines)
        {
            string rowPartition = partition.Substring(0, 7);
            string columnPartition = partition.Substring(7);

            int row = BinaryPartition(rowPartition, 0, N_ROWS - 1, 'F', 'B');
            int column = BinaryPartition(columnPartition, 0, N_COLS - 1, 'L', 'R');

            int seatId = row * 8 + column;
            seatIds.Add(seatId);
        }

        int prev = -1;
        foreach (int seatId in seatIds)
        {
            if (prev != -1 && seatId != prev + 1)
            {
                Console.WriteLine(seatId - 1);
            }
            prev = seatId;
        }
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day5/in.txt");
        Part1(inputLines);
        Part2(inputLines);
    }
}
