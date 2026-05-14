// Package main ...
package main

func isGood(nums []int) bool {
	n := len(nums)
	count := make([]int, n)
	for _, a := range nums {
		if a < 1 || a >= n {
			return false
		}
		if a < n-1 && count[a] > 0 {
			return false
		}
		if a == n-1 && count[a] > 1 {
			return false
		}
		count[a]++
	}
	return true
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{2, 1, 3}, false},
		{[]int{1, 3, 3, 2}, true},
		{[]int{1, 1}, true},
		{[]int{1, 2, 2, 3, 3, 4}, false},
	}

	for _, test := range tests {
		if got := isGood(test.nums); got != test.ans {
			panic(test)
		}
	}
}
