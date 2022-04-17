/*
 * @Date: 2022-04-18 07:14:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-18 07:16:27
 * @FilePath: /algorithm/386_lexical_order/lexical_order.go
 */

package main

import "reflect"

func lexicalOrder(n int) []int {
	ans := make([]int, n)
	num := 1
	for i := range ans {
		ans[i] = num
		if num*10 <= n {
			num *= 10
		} else {
			for num%10 == 9 || num+1 > n {
				num /= 10
			}
			num++
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(lexicalOrder(13), []int{1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9})
	assert(lexicalOrder(2), []int{1, 2})
}
