/*
 * @Date: 2023-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-18
 * @FilePath: /algorithm/golang/1073_add_negabinary/add_negabinary.go
 */

// Package main ...
package main

import "reflect"

func addNegabinary(arr1 []int, arr2 []int) (ans []int) {
	i := len(arr1) - 1
	j := len(arr2) - 1
	carry := 0
	for i >= 0 || j >= 0 || carry != 0 {
		x := carry
		if i >= 0 {
			x += arr1[i]
		}
		if j >= 0 {
			x += arr2[j]
		}

		if x >= 2 {
			ans = append(ans, x-2)
			carry = -1
		} else if x >= 0 {
			ans = append(ans, x)
			carry = 0
		} else {
			ans = append(ans, 1)
			carry = 1
		}
		i--
		j--
	}
	for len(ans) > 1 && ans[len(ans)-1] == 0 {
		ans = ans[:len(ans)-1]
	}
	for i, n := 0, len(ans); i < n/2; i++ {
		ans[i], ans[n-1-i] = ans[n-1-i], ans[i]
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
		arr1 := []int{1, 1, 1, 1, 1}
		arr2 := []int{1, 0, 1}
		ans := []int{1, 0, 0, 0, 0}
		assert(addNegabinary(arr1, arr2), ans)
	}

	{
		arr1 := []int{0}
		arr2 := []int{0}
		ans := []int{0}
		assert(addNegabinary(arr1, arr2), ans)
	}

	{
		arr1 := []int{0}
		arr2 := []int{1}
		ans := []int{1}
		assert(addNegabinary(arr1, arr2), ans)
	}
}
