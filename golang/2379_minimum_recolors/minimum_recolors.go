/*
 * @Date: 2023-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-09
 * @FilePath: /algorithm/golang/2379_minimum_recolors/minimum_recolors.go
 */

package main

func minimumRecolors(blocks string, k int) int {
	res := k
	left, whites := 0, 0
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	for right := 0; right < len(blocks); right++ {
		if blocks[right] == 'W' {
			whites++
		}
		if right < k-1 {
			continue
		}
		res = min(res, whites)
		if blocks[left] == 'W' {
			whites--
		}
		left++
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
		blocks := "WBBWWBBWBW"
		k := 7
		ans := 3
		assert(minimumRecolors(blocks, k) == ans)
	}

	{
		blocks := "WBWBBBW"
		k := 2
		ans := 0
		assert(minimumRecolors(blocks, k) == ans)
	}
}
