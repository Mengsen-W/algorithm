/*
 * @Date: 2022-10-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-15
 * @FilePath: /algorithm/1441_build_array/build_array.go
 */

package main

import "reflect"

func buildArray(target []int, n int) (ans []string) {
	prev := 0
	for _, number := range target {
		for i := 0; i < number-prev-1; i++ {
			ans = append(ans, "Push", "Pop")
		}
		ans = append(ans, "Push")
		prev = number
	}
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		target := []int{1, 3}
		n := 3
		ans := []string{"Push", "Push", "Pop", "Push"}
		assert(buildArray(target, n), ans)
	}

	{
		target := []int{1, 2, 3}
		n := 3
		ans := []string{"Push", "Push", "Push"}
		assert(buildArray(target, n), ans)
	}

	{
		target := []int{1, 2}
		n := 4
		ans := []string{"Push", "Push"}
		assert(buildArray(target, n), ans)
	}
}
