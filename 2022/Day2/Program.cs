void PartOne()
{
     var scores = new int[][] {
         new int[] { 1 + 3, 2 + 6, 3 + 0 },
         new int[] { 1 + 0, 2 + 3, 3 + 6 },
         new int[] { 1 + 6, 2 + 0, 3 + 3 },
};

    var total = 0;
    foreach (var line in File.ReadAllLines("input.txt"))
    {
        var players = line.Split(' ');
        var them = players[0][0] - 'A';
        var me = players[1][0] - 'X';
        var round = scores[them][me];
        total += round;
    }

    Console.WriteLine(total);
}

void PartTwo()
{
    var lookup = new int[][] {
                                //  Lose,  Draw,  Win
         /* Rock */     new int[] { 0 + 3, 3 + 1, 6 + 2 },
         /* Paper */    new int[] { 0 + 1, 3 + 2, 6 + 3 },
         /* Scissors */ new int[] { 0 + 2, 3 + 3, 6 + 1 },
    };

    var total = 0;
    foreach (var line in File.ReadAllLines("input.txt"))
    {
        var round = line.Split(' ');
        var them = round[0][0] - 'A';
        var endState = round[1][0] - 'X';
        total += lookup[them][endState];
    }

    Console.WriteLine(total);

}

PartTwo();