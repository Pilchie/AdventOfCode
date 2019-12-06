package main

import "testing"

func Test1(t *testing.T) {
	verify(t, 111111, true, false)
}

func Test2(t *testing.T) {
	verify(t, 223450, false, false)
}

func Test2_2(t *testing.T) {
	verify(t, 223450, false, true)
}

func Test3(t *testing.T) {
	verify(t, 123789, false, false)
}

func Test3_2(t *testing.T) {
	verify(t, 123789, false, true)
}

func Test4(t *testing.T) {
	verify(t, 112233, true, true)
}

func Test5(t *testing.T) {
	verify(t, 123444, false, true)
}

func Test6(t *testing.T) {
	verify(t, 111122, true, true)
}

func Test7(t *testing.T) {
	verify(t, 112222, true, true)
}

func Test8(t *testing.T) {
	verify(t, 111125, false, true)
}

func Test9(t *testing.T) {
	verify(t, 123356, true, true)
}


func verify(t *testing.T, input int, expectedValid bool, part2 bool) {
	var actual bool
	if part2 {
		actual = IsValidPasswordPart2(input)
	} else {
		actual = IsValidPassword(input)
	}

	if actual != expectedValid {
		t.Fatalf("Input: '%d', Expected '%v', Actual: '%v'", input, expectedValid, actual)
	}
}
