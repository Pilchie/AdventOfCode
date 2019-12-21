import math

def greatest_common_denominator(i1,i2):
    if abs(i1) < abs(i2):
        min = i1
    else:
        min = i2

    for i in range(int(abs(min)), 0, -1):
        if i1 % i == 0 and i2 % i == 0:
            return i
    raise Exception()

class Map:
    def __init__(self, lines):
        self._lines = lines.splitlines()

    def can_see(self, origin, destination):
        return self.first_visible(origin, destination) == destination

    def first_visible(self, origin, destination):
        if origin.x() == destination.x() and origin.y() == destination.y():
            return None

        dx = destination.x() - origin.x()
        dy = destination.y() - origin.y()
        if dx == 0:
            for i in range(1, abs(dy) + 1):
                p = Point(origin.x(), origin.y() + int(i*dy/abs(dy)))
                if self.is_asteroid(p):
                    return p
            return None
        elif dy == 0:
            for i in range(1, abs(dx) + 1):
                p = Point(origin.x() + int(i*dx/abs(dx)), origin.y())
                if self.is_asteroid(p):
                    return p
            return None
        else:
            i = 1
            gcd = greatest_common_denominator(dx,dy)
            while True:
                new  = Point(origin.x() + i * int(dx/gcd), origin.y() + i * int(dy/gcd))
                if new.x() < 0 or new.y() < 0 or new.y() >= len(self._lines) or new.x() >= len(self._lines[new.y()]):
                    return None
                if self.is_asteroid(new):
                    return new
                i = i + 1

    def is_asteroid(self, point):
        if self._lines[point.y()][point.x()] == "#":
            return True
        return False

    def all_asteroids(self):
        asteroids = []
        for y in range(0, len(self._lines)):
            for x in range (0, len(self._lines[y])):
                point = Point(x, y)
                if self.is_asteroid(point):
                    asteroids.append(point)
        return asteroids

    def count_visible(self, origin):
        visible = 0
        for a in self.all_asteroids():
            if self.can_see(origin, a):
                visible = visible + 1
        return visible

    def max_visible(self):
        point = None
        max_visible = 0
        for a in self.all_asteroids():
            count = self.count_visible(a)
            if count > max_visible:
                max_visible = count
                point = a
        return point, max_visible

    def check(self, base, point, result):
        first = self.first_visible(base, point)
        if first != None:
            line = self._lines[first.y()]
            line = line[:first.x()] + "." + line[first.x() + 1:]
            self._lines[first.y()] = line
            result.append(first)

    def order_vaporized(self, base):
        result = []
        count = None
        while count != len(result):
            count = len(result)
            for x in range(base.x(), len(self._lines[0])):
                self.check(base, Point(x, 0), result)
            for y in range(0, len(self._lines)):
                self.check(base, Point(len(self._lines[0]) - 1, y), result)
            for x in range(len(self._lines[0]), 0, -1):
                self.check(base, Point(x - 1, len(self._lines) - 1), result)
            for y in range(len(self._lines), 0, -1):
                self.check(base, Point(0, y), result)
            for x in range(0, base.x()):
                self.check(base, Point(x, 0), result)
        return result

class Point:
    def __init__(self, x, y):
        self._x = x
        self._y = y

    def x(self):
        return self._x

    def y(self):
        return self._y

    def __eq__(self, value):
        if value == None:
            return False
        return self._x == value._x and self._y == value._y

def solve_part1():
    with open("input.txt") as f:
        content = f.read()
    map = Map(content)
    point, count = map.max_visible()
    print(count)

if __name__ == "__main__":
    solve_part1()
