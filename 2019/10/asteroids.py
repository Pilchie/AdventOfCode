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
        elif abs(dx/dy) >= 1:
            for i in range(1, abs(dy)):
                if self.is_asteroid(Point(origin.x() + int(abs(dx/dy)*dx/abs(dx)*i), origin.y() + int(i*dy/abs(dy)))):
                    return False
            return True
        elif abs(dy/dx) >= 1:
            for i in range(1, abs(dx)):
                if self.is_asteroid(Point(origin.x() + int(i*dx/abs(dx)), origin.y() + int(abs(dy/dx)*dy/abs(dy)*i))):
                    return False
            return True
        else:
            raise Exception

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
