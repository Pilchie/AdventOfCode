import asteroids
import unittest

class Day10Tests(unittest.TestCase):
    large_map = """.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"""

    def __init__(self, methodName):
        super().__init__(methodName)

    def test_1(self):
        map = asteroids.Map(""".#..#
.....
#####
....#
...##""")
        self.verify_point_count(asteroids.Point(3,4), 8, map)

    def test_2(self):
        map = asteroids.Map("""......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####""")
        self.verify_point_count(asteroids.Point(5, 8), 33, map)

    def test_3(self):
        map = asteroids.Map("""#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.""")
        self.verify_point_count(asteroids.Point(1, 2), 35, map)

    def test_4(self):
        map = asteroids.Map(""".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..""")
        self.verify_point_count(asteroids.Point(6, 3), 41, map)

    def test_5(self):
        map = asteroids.Map(Day10Tests.large_map)
        self.verify_point_count(asteroids.Point(11, 13), 210, map)

    def verify_point_count(self, expected_point, expected_count, map):
        actual_point, actual_count = map.max_visible()
        self.assertEqual(expected_point.x(), actual_point.x())
        self.assertEqual(expected_point.y(), actual_point.y())
        self.assertEqual(expected_count, actual_count)

    def test_all_points_on_small_map(self):
        map = asteroids.Map(""".#..#
.....
#####
....#
...##""")
        answers = """.7..7
.....
67775
....7
...87""".splitlines()

        for y in range(0, len(answers)):
            for x in range(0, len(answers[y])):
                origin = asteroids.Point(x,y)
                if map.is_asteroid(origin):
                    self.assertEqual(int(answers[y][x]), map.count_visible(origin), f"At ({x},{y})")

    def test_part2_large_map(self):
        points = [
            (1, asteroids.Point(11, 12)),
            (2, asteroids.Point(12, 1)),
            (3, asteroids.Point(12, 2)),
            (10, asteroids.Point(12, 8)),
            (20, asteroids.Point(16, 0)),
            (50, asteroids.Point(16, 9)),
            (100, asteroids.Point(10, 16)),
            (199, asteroids.Point(9, 6)),
            (200, asteroids.Point(8, 2)),
            (201, asteroids.Point(10, 9)),
            (299, asteroids.Point(11, 1))
        ]
        self.verify_vaporized(points)

    def verify_vaporized(self, points):
        map = asteroids.Map(Day10Tests.large_map)
        order_vaporized = map.order_vaporized(asteroids.Point(11, 13))
        for i, expected in points:
            actual = order_vaporized[i - 1]
            self.assertEqual(expected, actual, f"Expected: ({expected.x()}, {expected.y()}), Actual: ({actual.x()}, {actual.y()})")

def main():
    unittest.main()

if __name__ == "__main__":
    main()