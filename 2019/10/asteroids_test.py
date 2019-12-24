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

    def test_part2_entire_small_map(self):
        map = asteroids.Map(""".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##""")
        order_vaporized = map.order_vaporized(asteroids.Point(8, 3))
        points = [
            (1, asteroids.Point(8, 1)),
            (2, asteroids.Point(9, 0)),
            (3, asteroids.Point(9, 1)),
            (4, asteroids.Point(10, 0)),
            (5, asteroids.Point(9, 2)),
            (6, asteroids.Point(11, 1)),
            (7, asteroids.Point(12, 1)),
            (8, asteroids.Point(11, 2)),
            (9, asteroids.Point(15, 1)),
            (10, asteroids.Point(12, 2)),
            (11, asteroids.Point(13, 2)),
            (12, asteroids.Point(14, 2)),
            (13, asteroids.Point(15, 2)),
            (14, asteroids.Point(12, 3)),
            (15, asteroids.Point(16, 4)),
            (16, asteroids.Point(15, 4)),
            (17, asteroids.Point(10, 4)),
            (18, asteroids.Point(4, 4)),
            (19, asteroids.Point(2, 4)),
            (20, asteroids.Point(2, 3)),
            (21, asteroids.Point(0, 2)),
            (22, asteroids.Point(1, 2)),
            (23, asteroids.Point(0, 1)),
            (24, asteroids.Point(1, 1)),
            (25, asteroids.Point(5, 2)),
            (26, asteroids.Point(1, 0)),
            (27, asteroids.Point(5, 1)),
            (28, asteroids.Point(6, 1)),
            (29, asteroids.Point(6, 0)),
            (30, asteroids.Point(7, 0)),
            (31, asteroids.Point(8, 0)),
            (32, asteroids.Point(10, 1)),
            (33, asteroids.Point(14, 0)),
            (34, asteroids.Point(16, 1)),
            (35, asteroids.Point(13, 3)),
            (36, asteroids.Point(14, 3)),
        ]
        self.verify_points(points, order_vaporized)

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
        self.verify_points(points, order_vaporized)
    
    def verify_points(self, points, order_vaporized):
        for i, expected in points:
            actual = order_vaporized[i - 1]
            self.assertEqual(expected, actual, f"Position {i} - Expected: ({expected.x()}, {expected.y()}), Actual: ({actual.x()}, {actual.y()})")

def main():
    unittest.main()

if __name__ == "__main__":
    main()