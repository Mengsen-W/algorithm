/*
 * @Date: 2023-01-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-04
 * @FilePath: /algorithm/1802_max_value/max_value.go
 */

package main

import "math"

func maxValue(n, index, maxSum int) int {
	left := index
	right := n - index - 1
	if left > right {
		left, right = right, left
	}

	upper := ((left+1)*(left+1)-3*(left+1))/2 + left + 1 + (left + 1) + ((left+1)*(left+1)-3*(left+1))/2 + right + 1
	if upper >= maxSum {
		a := 1.0
		b := -2.0
		c := float64(left + right + 2 - maxSum)
		return int((-b + math.Sqrt(b*b-4*a*c)) / (2 * a))
	}

	upper = (2*(right+1)-left-1)*left/2 + (right + 1) + ((right+1)*(right+1)-3*(right+1))/2 + right + 1
	if upper >= maxSum {
		a := 1.0 / 2
		b := float64(left) + 1 - 3.0/2
		c := float64(right + 1 + (-left-1)*left/2.0 - maxSum)
		return int((-b + math.Sqrt(b*b-4*a*c)) / (2 * a))
	} else {
		a := float64(left + right + 1)
		b := float64(-left*left-left-right*right-right)/2 - float64(maxSum)
		return int(-b / a)
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 4
		index := 2
		maxSum := 6
		ans := 2
		assert(maxValue(n, index, maxSum) == ans)
	}

	{
		n := 6
		index := 1
		maxSum := 10
		ans := 3
		assert(maxValue(n, index, maxSum) == ans)
	}
}
