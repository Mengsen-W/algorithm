// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func resultArray(nums []int) []int {
	arr1 := []int{nums[0]}
	arr2 := []int{nums[1]}
	for i := 2; i < len(nums); i++ {
		if arr1[len(arr1)-1] > arr2[len(arr2)-1] {
			arr1 = append(arr1, nums[i])
		} else {
			arr2 = append(arr2, nums[i])
		}
	}
	return append(arr1, arr2...)
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{2, 1, 3}, []int{2, 3, 1}},
		{[]int{5, 4, 3, 8}, []int{5, 3, 4, 8}},
	}

	for _, test := range tests {
		if reflect.DeepEqual(resultArray(test.nums), test.ans) {
			fmt.Println("Test passed")
		} else {
			fmt.Println("Test failed")
		}
	}
}
