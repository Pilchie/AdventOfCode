class Image:
    def __init__(self, image_values, width, height):
        super().__init__()
        self.layers = []
        self.width = width
        self.height = height
        layer_size = width * height
        print(f"layer size: {layer_size}")
        layer_count = int(len(image_values) / layer_size)
        print(f"layer_count: {layer_count}")
        for i in range (0, layer_count):
            self.layers.append(image_values[layer_size*i:layer_size*(i+1)])

    def layer_with_fewest_zeros(self):
        min = None
        min_layer = None
        for i in range(0, len(self.layers)):
            count =  self.count(self.layers[i], 0)
            if min == None or count < min:
                min = count
                min_layer = i
        return self.layers[min_layer]

    def count(self, layer, value):
        result = 0
        for x in layer:
            if x == value:
                result = result + 1

        print(f"count for '{value}' returning '{result}'")
        return result

    def part1(self):
        layer = self.layer_with_fewest_zeros()
        ones = self.count(layer, 1)
        twos = self.count(layer, 2)
        return ones * twos

    def value_at_position(self, x, y):
        index = y * self.width + x
        for layer in self.layers:
            if layer[index] != 2:
                return layer[index]
        raise Exception

    def part2(self):
        image = []
        for y in range(0, self.height):
            for x in range(0, self.width):
                image.append(self.value_at_position(x, y))
        return image

def solve_part1():
    with open("input.txt") as f:
        content = f.read()
    image_values = list(map(lambda x: int(x), content))
    image = Image(image_values, 25, 6)
    answer = image.part1()
    print(answer)

def solve_part2():
    with open("input.txt") as f:
        content = f.read()
    image_values = list(map(lambda x: int(x), content))
    image = Image(image_values, 25, 6)
    answer = image.part2()
    for y in range(0, 6):
        for x in range(0, 25):
            val = answer[y * 25 + x]
            if val == 0:
                print("  ", end='')
            else:
                print("\u2591\u2591", end='')
        print()
    print()

def main():
    #solve_part1()
    solve_part2()

if __name__ == "__main__":
    main()
