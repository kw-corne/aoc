class Day8
{
    private enum TerminationState
    {
        Looped,
        ReachedEnd,
    }

    private class Instruction
    {
        public string Operation { get; set; }
        public int Argument { get; set; }

        public Instruction(string instructionLine)
        {
            var instructionParts = instructionLine.Split();

            Operation = instructionParts[0];
            Argument = int.Parse(instructionParts[1]);
        }
    }

    private static int ExecuteProgram(List<String> program, out TerminationState terminateState)
    {
        var seenIdx = new HashSet<int>();
        int accumulator = 0;
        int currIdx = 0;

        while (!seenIdx.Contains(currIdx) && currIdx < program.Count)
        {
            seenIdx.Add(currIdx);
            Instruction instr = new Instruction(program[currIdx]);

            switch (instr.Operation)
            {
                case "nop":
                    {
                        currIdx++;
                        break;
                    }
                case "acc":
                    {
                        accumulator += instr.Argument;
                        currIdx++;
                        break;
                    }
                case "jmp":
                    {
                        currIdx += instr.Argument;
                        break;
                    }
                default:
                    throw new NotImplementedException();
            }
        }

        terminateState = (currIdx == program.Count) ?
                TerminationState.ReachedEnd :
                TerminationState.Looped;

        return accumulator;
    }

    private static IEnumerable<List<string>> GeneratePrograms(List<string> program)
    {
        for (int i = 0; i < program.Count; i++)
        {
            var programCopy = new List<string>(program);
            Instruction instr = new Instruction(program[i]);

            if (instr.Operation == "nop")
            {
                programCopy[i] = $"jmp {instr.Argument}";
            }
            else if (instr.Operation == "jmp")
            {
                programCopy[i] = $"nop {instr.Argument}";
            }

            yield return programCopy;
        }

        yield break;
    }

    private static void Part2(List<string> inputLines)
    {
        foreach (List<string> program in GeneratePrograms(inputLines))
        {
            int result = ExecuteProgram(program, out TerminationState termState);

            if (termState == TerminationState.ReachedEnd)
            {
                Console.WriteLine($"Accumulator: {result} {termState.ToString()}");
            }
        }
    }

    private static void Part1(List<string> inputLines)
    {
        int result = ExecuteProgram(inputLines, out TerminationState termState);
        Console.WriteLine($"Accumulator: {result} {termState.ToString()}");
    }

    public static void solve()
    {
        var inputLines = AocUtil.GetLines("Solutions/Day8/in.txt");
        // Part1(inputLines);
        Part2(inputLines);
    }
}
