/*
 * @Date: 2022-10-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-17
 * @FilePath: /algorithm/904_total_fruit/total_fruit.go
 */

package main

func totalFruit(fruits []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	cnt := map[int]int{}
	left := 0
	for right, x := range fruits {
		cnt[x]++
		for len(cnt) > 2 {
			y := fruits[left]
			cnt[y]--
			if cnt[y] == 0 {
				delete(cnt, y)
			}
			left++
		}
		ans = max(ans, right-left+1)
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		fruits := []int{1, 2, 1}
		ans := 3
		assert(totalFruit(fruits) == ans)
	}

	{
		fruits := []int{0, 1, 2, 2}
		ans := 3
		assert(totalFruit(fruits) == ans)
	}

	{
		fruits := []int{1, 2, 3, 2, 2}
		ans := 4
		assert(totalFruit(fruits) == ans)
	}

	{
		fruits := []int{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4}
		ans := 5
		assert(totalFruit(fruits) == ans)
	}
}
