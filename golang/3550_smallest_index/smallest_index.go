// Package main ...
package main

import "fmt"

func smallestIndex(nums []int) int {
	for i, num := range nums {
		digitSum := 0

		for num > 0 {
			digitSum += num % 10
			num /= 10
		}

		if digitSum == i {
			return i
		}
	}

	return -1
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 3, 2}, 2},
		{[]int{1, 10, 11}, 1},
		{[]int{1, 2, 3}, -1},
	}

	for index, test := range tests {
		if smallestIndex(test.nums) != test.ans {
			fmt.Println(index)
		}
	}
}
