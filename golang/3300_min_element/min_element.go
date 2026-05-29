// Package main ...
package main

func minElement(nums []int) int {
	ans := 37
	for _, num := range nums {
		dig := 0
		n := num
		for n > 0 {
			dig += n % 10
			n /= 10
		}
		if dig < ans {
			ans = dig
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{10, 12, 13, 14}, 1},
		{[]int{1, 2, 3, 4}, 1},
		{[]int{999, 19, 199}, 10},
	}

	for _, test := range tests {
		if minElement(test.nums) != test.ans {
			panic("test failed")
		}
	}
}
