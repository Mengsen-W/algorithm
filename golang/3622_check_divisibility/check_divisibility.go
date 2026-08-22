// Package main ...
package main

func checkDivisibility(n int) bool {
	digitSum := 0
	digitProduct := 1
	original := n

	for n > 0 {
		digit := n % 10
		n /= 10

		digitSum += digit
		digitProduct *= digit
	}

	return original%(digitSum+digitProduct) == 0
}

func main() {
	tests := []struct {
		n   int
		ans bool
	}{
		{99, true},
		{23, false},
	}

	for _, test := range tests {
		if checkDivisibility(test.n) != test.ans {
			panic("Not Passed")
		}
	}
}
