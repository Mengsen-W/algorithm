/*
 * @Date: 2023-02-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-10
 * @FilePath: /algorithm/golang/1223_die_simulator/die_simulator.go
 */

package main

func dieSimulator(n int, rollMax []int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	const mod = 1e9 + 7
	d := make([][6]int, n+1)
	sum := make([]int, n+1)
	sum[0] = 1
	for i := 1; i <= n; i++ {
		for j := 0; j < 6; j++ {
			pos := max(i-rollMax[j]-1, 0)
			sub := ((sum[pos]-d[pos][j])%mod + mod) % mod
			d[i][j] = ((sum[i-1]-sub)%mod + mod) % mod
			if i <= rollMax[j] {
				d[i][j] = (d[i][j] + 1) % mod
			}
			sum[i] = (sum[i] + d[i][j]) % mod
		}
	}
	return sum[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 2
		rollMax := []int{1, 1, 2, 2, 2, 3}
		ans := 34
		assert(dieSimulator(n, rollMax) == ans)
	}

	{
		n := 2
		rollMax := []int{1, 1, 1, 1, 1, 1}
		ans := 30
		assert(dieSimulator(n, rollMax) == ans)
	}

	{
		n := 3
		rollMax := []int{1, 1, 1, 2, 2, 3}
		ans := 181
		assert(dieSimulator(n, rollMax) == ans)
	}
}
