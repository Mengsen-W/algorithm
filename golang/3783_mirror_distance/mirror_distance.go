// Package main ...
package main

func mirrorDistance(n int) int {
	reverse := func(n int) int {
		res := 0
		for n > 0 {
			res = res*10 + n%10
			n /= 10
		}
		return res
	}

	diff := n - reverse(n)
	if diff < 0 {
		return -diff
	}
	return diff
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{25, 27},
		{10, 9},
		{7, 0},
	}

	for _, test := range tests {
		if mirrorDistance(test.n) != test.ans {
			panic("wrong")
		}
	}
}
