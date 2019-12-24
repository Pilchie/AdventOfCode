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

    # Return all the points in the map, sorted by angle and then distance
    def sort_points(self, base):
        points = list(filter(lambda p: p != base, self.all_asteroids()))
        points.sort(key=lambda p: 1000000*p.angle_from(base) + p.distance_from(base))
        return points

    def order_vaporized(self, base):
        result = []
        points = self.sort_points(base)
        while len(points) > 0:
            i = 0
            while i < len(points):
                angle = points[i].angle_from(base)
                result.append(points[i])
                points = points[:i] + points[i+1:]
                while i < len(points) and angle == points[i].angle_from(base):
                    i += 1
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

    def angle_from(self, other):
        dy = -1*(self.y() - other.y())
        dx = self.x() - other.x()
        if dx == 0:
            if dy < 0:
                angle = -1 * math.atan(math.inf)
            else:
                angle = math.atan(math.inf)
        else:
            angle = math.atan(dy/dx)
        angle = math.degrees(angle)
        if dx >= 0:
            if dy > 0:
                angle =  90 - angle # Q1
            else:
                angle = 90 - angle # Q4
        else:
            if dy > 0:
                angle = 270 - angle # Q2
            else:
                angle = 270 - angle # Q3
        return angle

    def distance_from(self, other):
        dx = self.x() - other.x()
        dy = self.y() - other.y()
        return math.sqrt(dx**2 + dy**2)

def solve_part1():
    with open("input.txt") as f:
        content = f.read()
    map = Map(content)
    point, count = map.max_visible()
    print(count)

def solve_part2():
    with open("input.txt") as f:
        content = f.read()
    map = Map(content)
    point, count = map.max_visible()
    order = map.order_vaporized(point)
    print(100 * order[199].x() + order[199].y())

if __name__ == "__main__":
    #solve_part1()
    solve_part2()
