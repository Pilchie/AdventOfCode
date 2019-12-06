package main

import (
	"fmt"
	"strconv"
)

// IsValidPassword returns true if input is a valid password, false otherwise
func IsValidPassword(input int) bool {
	if input < 111111 {
		return false
	}

	if input > 999999 {
		return false
	}

	hasDouble := false
	s := strconv.Itoa(input)
	prev, _ := strconv.Atoi(s[:1])
	for i := 1; i < 6; i++ {
		cur, _ := strconv.Atoi(s[i : i+1])
		if cur < prev {
			return false
		}

		if cur == prev {
			hasDouble = true
		}
		prev = cur
	}

	return hasDouble
}

// IsValidPasswordPart2 returns true if input is a valid password with the extra rule from Part2, false otherwise
func IsValidPasswordPart2(input int) bool {
	if input < 111111 {
		return false
	}

	if input > 999999 {
		return false
	}

	s := strconv.Itoa(input)

	digits := make([]int, 6)
	for i := 0; i < 6; i++ {
		digits[i], _ = strconv.Atoi(s[i : i+1])
	}

	prev := digits[0]
	for i := 1; i < 6; i++ {
		cur := digits[i]
		if cur < prev {
			return false
		} 
		
		prev = cur
	}

	if digits[0] == digits[1] && digits[1] != digits[2] {
		return true
	} else if digits[3] != digits[4] && digits[4] == digits[5] {
		return true
	} else {
		for i := 1; i < 4; i++ {
			if digits[i-1] != digits[i] && digits[i] == digits[i+1] && digits[i+1] != digits[i+2] {
				return true
			}
		}
	}

	return false
}

func main() {
	count := 0
	for i := 146810; i < 612564; i++ {
		if IsValidPasswordPart2(i) {
			count++
		}
	}

	fmt.Println(count)
}
