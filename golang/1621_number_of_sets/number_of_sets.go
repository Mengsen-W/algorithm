// Package main ...
package main

import "strconv"

const mod1621 int64 = 1000000007

func pow1621(a, e int64) int64 {
	result := int64(1)
	for e > 0 {
		if e&1 == 1 {
			result = result * a % mod1621
		}
		a = a * a % mod1621
		e >>= 1
	}
	return result
}

func numberOfSets(n int, k int) int {
	m := 2 * k
	numerator, denominator := int64(1), int64(1)
	for i := 1; i <= m; i++ {
		numerator = numerator * int64(n+k-i) % mod1621
		denominator = denominator * int64(i) % mod1621
	}
	return int(numerator * pow1621(denominator, mod1621-2) % mod1621)
}

func main() {
	tests := []struct {
		n        int
		k        int
		expected int
	}{
		{4, 2, 5}, {3, 1, 3}, {30, 7, 796297179}, {5, 3, 7}, {3, 2, 1},
	}

	for index, test := range tests {
		if numberOfSets(test.n, test.k) != test.expected {
			panic("test" + strconv.Itoa(index) + " failed")
		}
	}
}
