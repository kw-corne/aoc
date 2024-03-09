class Day7
{
    private Dictionary<string, List<(string, int)>> GetBagsFromInput(List<string> inputLines)
    {
        var bags = new Dictionary<string, List<(string, int)>>();

        foreach (string line in inputLines)
        {
            var words = line.Split();
            string bag = $"{words[0]} {words[1]}";

            if (!bags.ContainsKey(bag))
            {
                bags.Add(bag, new List<(string, int)>());
            }

            for (int i = 4; i < words.Length; i += 4)
            {
                if (words[i] == "no")
                {
                    continue;
                }

                int amount = int.Parse(words[i]);
                string targetBag = words[i + 1] + " " + words[i + 2];

                bags[bag].Add((targetBag, amount));
            }
        }

        return bags;
    }

    private bool ReducesToShinyGold(string bag, Dictionary<string, List<(string, int)>> bags)
    {
        foreach (var bagAmount in bags[bag])
        {
            if (bagAmount.Item1 == "shiny gold")
            {
                return true;
            }
            else
            {
                if (ReducesToShinyGold(bagAmount.Item1, bags))
                {
                    return true;
                }
            }
        }

        return false;
    }

    private void Part2(List<string> inputLines) { }

    private void Part1(List<string> inputLines)
    {
        var bags = GetBagsFromInput(inputLines);
        int count = 0;

        foreach (string bag in bags.Keys)
        {
            if (ReducesToShinyGold(bag, bags))
            {
                count++;
            }
        }

        Console.WriteLine(count);
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day7/in.txt");
        Part1(inputLines);
        // Part2(inputLines);
    }
}
