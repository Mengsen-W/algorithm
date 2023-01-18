/*
 * @Date: 2022-07-04
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-04
 * @FilePath: /algorithm/556_next_greater_element/next_greater_element.go
 */

package main

import "math"

func nextGreaterElement(n int) int {
	x, cnt := n, 1
	for ; x >= 10 && x/10%10 >= x%10; x /= 10 {
		cnt++
	}
	x /= 10
	if x == 0 {
		return -1
	}

	targetDigit := x % 10
	x2, cnt2 := n, 0
	for ; x2%10 <= targetDigit; x2 /= 10 {
		cnt2++
	}
	x += x2%10 - targetDigit // 把 x2%10 换到 targetDigit 上

	for i := 0; i < cnt; i++ { // 反转 n 末尾的 cnt 个数字拼到 x 后
		d := targetDigit
		if i != cnt2 {
			d = n % 10
		}
		if x > math.MaxInt32/10 || x == math.MaxInt32/10 && d > 7 {
			return -1
		}
		x = x*10 + d
		n /= 10
	}
	return x
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(nextGreaterElement(12) == 21)
	assert(nextGreaterElement(21) == -1)
}
