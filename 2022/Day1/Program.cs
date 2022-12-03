var elves = new List<int>();
elves.Add(0);

foreach (var line in File.ReadAllLines("input.txt"))
{
    if (string.IsNullOrEmpty(line))
    {
        elves.Add(0);
        continue;
    }

    elves[^1] += int.Parse(line);
}

// Part 1:
////Console.WriteLine(elves.Max());

// Part 2:
elves.Sort();
Console.WriteLine(elves.ToArray()[^3..].Sum());
