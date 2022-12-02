/*
 * @Date: 2022-12-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-02
 * @FilePath: /algorithm/1769_min_operations/min_operations.go
 */

package main

import "reflect"

func minOperations(boxes string) []int {
	left := int(boxes[0] - '0')
	right := 0
	operations := 0
	n := len(boxes)
	for i := 1; i < n; i++ {
		if boxes[i] == '1' {
			right++
			operations += i
		}
	}
	ans := make([]int, n)
	ans[0] = operations
	for i := 1; i < n; i++ {
		operations += left - right
		if boxes[i] == '1' {
			left++
			right--
		}
		ans[i] = operations
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		box := "110"
		ans := []int{1, 1, 3}
		assert(minOperations(box), ans)
	}

	{
		box := "001011"
		ans := []int{11, 8, 5, 4, 3, 4}
		assert(minOperations(box), ans)
	}
}
