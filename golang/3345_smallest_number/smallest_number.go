// Package main ...
package main

func smallestNumber(n int, t int) int {
	getMultiply := func(a int) int {
		ans := 1
		for a > 0 {
			tmp := a % 10
			if a == 0 {
				return 0
			}
			ans *= tmp % 10
			a /= 10
		}
		return ans
	}

	for n > 0 {
		tmp := getMultiply(n)
		if tmp%t == 0 {
			return n
		}
		n++
	}

	return -1
}

func main() {
	tests := []struct {
		n   int
		t   int
		ans int
	}{
		{10, 2, 10},
		{15, 3, 16},
	}

	for _, test := range tests {
		if smallestNumber(test.n, test.t) == test.ans {
			println("Test passed:", test.n, test.t, test.ans)
		} else {
			println("Test failed:", test.n, test.t, test.ans)
		}
	}
}
