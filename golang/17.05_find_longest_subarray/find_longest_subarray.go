/*
 * @Date: 2023-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-11
 * @FilePath: /algorithm/golang/17.05_find_longest_subarray/find_longest_subarray.go
 */

// Package main ...
package main

import "reflect"

func findLongestSubarray(array []string) []string {
	indices := map[int]int{}
	sum := 0
	startIndex := -1
	maxLength := 0
	indices[0] = -1
	for i, s := range array {
		if s[0] >= '0' && s[0] <= '9' {
			sum++
		} else {
			sum--
		}
		if firstIndex, ok := indices[sum]; ok {
			if i-firstIndex > maxLength {
				maxLength = i - firstIndex
				startIndex = firstIndex + 1
			}
		} else {
			indices[sum] = i
		}
	}
	if maxLength == 0 {
		return []string{}
	}
	return array[startIndex : startIndex+maxLength]
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		array := []string{"A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I", "J", "K", "L", "M"}
		ans := []string{"A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7"}
		assert(findLongestSubarray(array), ans)
	}

	{
		array := []string{"A", "A"}
		ans := []string{}
		assert(findLongestSubarray(array), ans)
	}
}
