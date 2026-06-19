// Package main ...
package main

func largestAltitude(gain []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	total := 0
	for _, x := range gain {
		total += x
		ans = max(ans, total)
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	tests := []struct {
		gain []int
		ans  int
	}{
		{[]int{-5, 1, 5, 0, -7}, 1},
		{[]int{-4, -3, -2, -1, 4, 3, 2}, 0},
	}
	for _, test := range tests {
		assert(largestAltitude(test.gain) == test.ans)
	}
}
