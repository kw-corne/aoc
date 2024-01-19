class Day2
{
    private void Part1(List<string> inputLines)
    {
        int validCount = 0;

        foreach (string line in inputLines)
        {
            string[] words = line.Split();
            string[] range = words[0].Split('-');

            int minOccurence = int.Parse(range[0]);
            int maxOccurence = int.Parse(range[1]);
            char letter = words[1][0];
            string password = words[2];

            int charCount = 0;
            foreach (char ch in password)
            {
                if (ch == letter)
                {
                    charCount++;
                }
            }

            if (charCount >= minOccurence && charCount <= maxOccurence)
            {
                validCount++;
            }
        }

        Console.WriteLine(validCount);
    }

    private void Part2(List<string> inputLines)
    {
        int validCount = 0;

        foreach (string line in inputLines)
        {
            string[] words = line.Split();
            string[] range = words[0].Split('-');

            int index1 = int.Parse(range[0]);
            int index2 = int.Parse(range[1]);
            char letter = words[1][0];
            string password = words[2];

            int seen = 0;
            if (password[index1 - 1] == letter)
            {
                seen++;
            }

            if (password[index2 - 1] == letter)
            {
                seen++;
            }

            if (seen == 1)
            {
                validCount++;
            }
        }

        Console.WriteLine(validCount);
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day2/in.txt");
        Part1(inputLines);
        Part2(inputLines);
    }
}
