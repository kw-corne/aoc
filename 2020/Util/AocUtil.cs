class AocUtil
{
    public static List<string> GetLines(string filename)
    {
        List<string> lines = new List<string>();

        using (StreamReader sr = new StreamReader(filename))
        {
            string? line;

            while ((line = sr.ReadLine()) != null)
            {
                lines.Add(line);
            }
        }

        return lines;
    }

    public static List<List<char>> LinesToGrid(IEnumerable<string> lines)
    {
        var grid = new List<List<char>>();

        foreach (string line in lines)
        {
            grid.Add(new List<char>());
            foreach (char ch in line)
            {
                grid.Last().Add(ch);
            }
        }

        return grid;
    }
}
