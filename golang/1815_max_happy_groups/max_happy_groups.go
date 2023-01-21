/*
 * @Date: 2023-01-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-22
 * @FilePath: /algorithm/golang/1815_max_happy_groups/max_happy_groups.go
 */

package main

func maxHappyGroups(batchSize int, groups []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	const Width = 5
	const WidthMask = 1<<Width - 1

	cnt := make([]int, batchSize)
	for _, x := range groups {
		cnt[x%batchSize]++
	}

	start := 0
	for i := batchSize - 1; i > 0; i-- {
		start = start<<Width | cnt[i]
	}

	memo := map[int]int{}
	var dfs func(int) int
	dfs = func(mask int) (best int) {
		if mask == 0 {
			return
		}
		if res, ok := memo[mask]; ok {
			return res
		}

		total := 0
		for i := 1; i < batchSize; i++ {
			amount := mask >> ((i - 1) * Width) & WidthMask
			total += i * amount
		}

		for i := 1; i < batchSize; i++ {
			amount := mask >> ((i - 1) * Width) & WidthMask
			if amount > 0 {
				result := dfs(mask - 1<<((i-1)*Width))
				if (total-i)%batchSize == 0 {
					result++
				}
				best = max(best, result)
			}
		}
		memo[mask] = best
		return
	}
	return dfs(start) + cnt[0]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		batchSize := 3
		groups := []int{1, 2, 3, 4, 5, 6}
		ans := 4
		assert(maxHappyGroups(batchSize, groups) == ans)
	}

	{
		batchSize := 4
		groups := []int{1, 3, 2, 5, 2, 2, 1, 6}
		ans := 4
		assert(maxHappyGroups(batchSize, groups) == ans)
	}
}
