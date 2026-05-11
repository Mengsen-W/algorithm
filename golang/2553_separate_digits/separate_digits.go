// Package main ...
package main

import "reflect"

func separateDigits(nums []int) []int {
	res := []int{}
	for i := len(nums) - 1; i >= 0; i-- {
		x := nums[i]
		for x > 0 {
			res = append(res, x%10)
			x /= 10
		}
	}

	for i, j := 0, len(res)-1; i < j; i, j = i+1, j-1 {
		res[i], res[j] = res[j], res[i]
	}

	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{13, 25, 83, 77}, []int{1, 3, 2, 5, 8, 3, 7, 7}},
		{[]int{7, 1, 3, 9}, []int{7, 1, 3, 9}},
	}

	for _, test := range tests {
		ans := separateDigits(test.nums)
		if !reflect.DeepEqual(ans, test.ans) {
			panic("ans")
		}
	}
}
