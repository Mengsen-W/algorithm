/*
 * @Date: 2023-03-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-25
 * @FilePath: /algorithm/golang/1574_find_length_of_shortest_subarray/find_length_of_shortest_subarray.go
 */

// Package main ...
package main

func findLengthOfShortestSubarray(arr []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	n := len(arr)
	j := n - 1
	for j > 0 && arr[j-1] <= arr[j] {
		j--
	}
	if j == 0 {
		return 0
	}
	res := j
	for i := 0; i < n; i++ {
		for j < n && arr[j] < arr[i] {
			j++
		}
		res = min(res, j-i-1)
		if i+1 < n && arr[i] > arr[i+1] {
			break
		}
	}
	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		arr := []int{1, 2, 3, 10, 4, 2, 3, 5}
		ans := 3
		assert(findLengthOfShortestSubarray(arr) == ans)
	}

	{
		arr := []int{5, 4, 3, 2, 1}
		ans := 4
		assert(findLengthOfShortestSubarray(arr) == ans)
	}

	{
		arr := []int{1, 2, 3}
		ans := 0
		assert(findLengthOfShortestSubarray(arr) == ans)
	}

	{
		arr := []int{1}
		ans := 0
		assert(findLengthOfShortestSubarray(arr) == ans)
	}
}
