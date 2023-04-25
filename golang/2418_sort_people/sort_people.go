/*
 * @Date: 2023-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-25
 * @FilePath: /algorithm/golang/2418_sort_people/sort_people.go
 */

// Package main ...
package main

import (
	"reflect"
	"sort"
)

func sortPeople(names []string, heights []int) []string {
	n := len(names)
	arr := make([][2]int, n)
	for i, h := range heights {
		arr[i] = [2]int{h, i}
	}
	sort.Slice(arr, func(i, j int) bool { return arr[i][0] > arr[j][0] })
	ans := make([]string, n)
	for i, x := range arr {
		ans[i] = names[x[1]]
	}
	return ans
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		names := []string{"Mary", "John", "Emma"}
		heights := []int{180, 165, 170}
		ans := []string{"Mary", "Emma", "John"}
		assert(sortPeople(names, heights), ans)
	}

	{
		names := []string{"Alice", "Bob", "Bob"}
		heights := []int{155, 185, 150}
		ans := []string{"Bob", "Alice", "Bob"}
		assert(sortPeople(names, heights), ans)
	}
}
