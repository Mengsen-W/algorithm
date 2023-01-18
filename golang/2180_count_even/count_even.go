/*
 * @Date: 2023-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-06
 * @FilePath: /algorithm/2180_count_even/count_even.go
 */

package main

func countEven(num int) int {
	y, x := num/10, num%10
	ans := y * 5
	ySum := 0
	for ; y > 0; y /= 10 {
		ySum += y % 10
	}
	if ySum%2 == 0 {
		ans += x / 2
	} else {
		ans += (x+1)/2 - 1
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		num := 4
		ans := 2
		assert(countEven(num) == ans)
	}

	{
		num := 30
		ans := 14
		assert(countEven(num) == ans)
	}
}
