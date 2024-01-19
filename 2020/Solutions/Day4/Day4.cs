class Day4
{
    private void Part1(List<string> inputLines)
    {
        string[] batches = String.Join("\n", inputLines).Split("\n\n");
        int validCount = 0;

        foreach (string batch in batches)
        {
            var fieldsSeen = new HashSet<string>();
            foreach (string field in batch.Split())
            {
                string[] parts = field.Split(":", 2);
                fieldsSeen.Add(parts[0]);
            }

            bool goodLen = (fieldsSeen.Count() == 8);
            bool goodLenAndCid = (fieldsSeen.Count() == 7 && !fieldsSeen.Contains("cid"));

            if (goodLen || goodLenAndCid)
            {
                validCount++;
            }
        }
        Console.WriteLine(validCount);
    }

    private bool ValidateField(string field, string value)
    {
        switch (field)
        {
            case "byr":
                int birthYear = int.Parse(value);
                return birthYear >= 1920 && birthYear <= 2002;
            case "iyr":
                int issueYear = int.Parse(value);
                return issueYear >= 2010 && issueYear <= 2020;
            case "eyr":
                int expirationYear = int.Parse(value);
                return expirationYear >= 2020 && expirationYear <= 2030;
            case "hgt":
                int i = 0;
                while (i < value.Length && char.IsDigit(value[i]))
                {
                    i++;
                }

                if (i == value.Length)
                {
                    return false;
                }

                var height = int.Parse(value.Substring(0, i));
                var heightUnit = value.Substring(i, 2);
                if (heightUnit == "cm")
                {
                    return height >= 150 && height <= 193;
                }
                else if (heightUnit == "in")
                {
                    return height >= 59 && height <= 76;
                }

                return false;
            case "hcl":
                if (value[0] != '#')
                {
                    return false;
                }

                for (i = 1; i < value.Length; i++)
                {
                    bool isValidChar = (value[i] >= 'a' && value[i] <= 'f');
                    if (!(isValidChar || char.IsDigit(value[i])))
                    {
                        return false;
                    }
                }

                return true;
            case "ecl":
                string[] valid = { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
                return valid.Contains(value);
            case "pid":
                if (value.Length != 9)
                {
                    return false;
                }

                for (i = 0; i < value.Length; i++)
                {
                    if (!char.IsDigit(value[i]))
                    {
                        return false;
                    }
                }

                return true;
            case "cid":
                return true;
        }

        throw new Exception("Invalid field?");
    }

    private void Part2(List<string> inputLines)
    {
        string[] batches = String.Join("\n", inputLines).Split("\n\n");
        int validCount = 0;

        foreach (string batch in batches)
        {
            var fieldsSeen = new HashSet<string>();
            bool allFieldsValid = true;

            foreach (string field in batch.Split())
            {
                string[] parts = field.Split(":", 2);

                if (!ValidateField(parts[0], parts[1]))
                {
                    allFieldsValid = false;
                    break;
                }

                fieldsSeen.Add(parts[0]);
            }

            bool goodLen = (fieldsSeen.Count() == 8);
            bool goodLenAndCid = (fieldsSeen.Count() == 7 && !fieldsSeen.Contains("cid"));

            if ((goodLen || goodLenAndCid) && allFieldsValid)
            {
                validCount++;
            }
        }

        Console.WriteLine(validCount);
    }

    public void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day4/in.txt");
        Part1(inputLines);
        Part2(inputLines);
    }
}
