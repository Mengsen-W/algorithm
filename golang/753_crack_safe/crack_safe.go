/*
 * @Date: 2023-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-10
 * @FilePath: /algorithm/753_crack_safe/crack_safe.go
 */

package main

import (
	"math"
	"strconv"
)

func crackSafe(n int, k int) string {
	seen := map[int]bool{}
	ans := ""
	highest := int(math.Pow(10, float64(n-1)))

	var dfs func(int)
	dfs = func(node int) {
		for x := 0; x < k; x++ {
			nei := node*10 + x
			if !seen[nei] {
				seen[nei] = true
				dfs(nei % highest)
				ans += strconv.Itoa(x)
			}
		}
	}
	dfs(0)
	for i := 1; i < n; i++ {
		ans += "0"
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
		n := 1
		k := 2
		ans := "10"
		assert(crackSafe(n, k) == ans)
	}

	{
		n := 2
		k := 2
		ans := "01100"
		assert(crackSafe(n, k) == ans)
	}
}
