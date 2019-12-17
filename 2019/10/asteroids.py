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
        if origin.x() == destination.x() and origin.y() == destination.y():
            return False

        dx = destination.x() - origin.x()
        dy = destination.y() - origin.y()
        if dx == 0:
            for i in range(1, abs(dy)):
                if self.is_asteroid(Point(origin.x(), origin.y() + int(i*dy/abs(dy)))):
                    return False
            return True
        elif dy == 0:
            for i in range(1, abs(dx)):
                if self.is_asteroid(Point(origin.x() + int(i*dx/abs(dx)), origin.y())):
                    return False
            return True
        else:
            i = 1
            gcd = greatest_common_denominator(dx,dy)
            while True:
                new  = Point(origin.x() + i * int(dx/gcd), origin.y() + i * int(dy/gcd))
                if new.x() == destination.x() and new.y() == destination.y():
                    return True
                if self.is_asteroid(new):
                    return False
                i = i + 1
            raise Exception()

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

class Point:
    def __init__(self, x, y):
        self._x = x
        self._y = y

    def x(self):
        return self._x

    def y(self):
        return self._y
