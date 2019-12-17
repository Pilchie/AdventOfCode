import asteroids
import unittest

class Day10Tests(unittest.TestCase):
    def __init__(self, methodName):
        super().__init__(methodName)

    def test_1(self):
        map = asteroids.Map(""".#..#
.....
#####
....#
...##""")
        #actual_point, actual_count = map.max_visible()
        #self.verify_point_count(asteroids.Point(3,4), 8, actual_point, actual_count)
        point = asteroids.Point(3,4)
        self.assertEqual(8, map.count_visible(point))

    def verify_point_count(self, expected_point, expected_count, actual_point, actual_count):
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

def main():
    unittest.main()

if __name__ == "__main__":
    main()