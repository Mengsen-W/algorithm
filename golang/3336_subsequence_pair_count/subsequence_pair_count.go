// Package main ...
package main

import "fmt"

func subsequencePairCount(nums []int) int {
	const MOD = 1000000007
	gcd := func(a, b int) int {
		for b != 0 {
			a, b = b, a%b
		}
		return a
	}

	m := 0
	for _, num := range nums {
		m = max(m, num)
	}

	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, m+1)
	}
	dp[0][0] = 1

	for _, num := range nums {
		ndp := make([][]int, m+1)
		for i := range ndp {
			ndp[i] = make([]int, m+1)
		}

		for j := 0; j <= m; j++ {
			divisor1 := gcd(j, num)
			for k := 0; k <= m; k++ {
				val := dp[j][k]
				if val == 0 {
					continue
				}

				divisor2 := gcd(k, num)
				ndp[j][k] = (ndp[j][k] + val) % MOD
				ndp[divisor1][k] = (ndp[divisor1][k] + val) % MOD
				ndp[j][divisor2] = (ndp[j][divisor2] + val) % MOD
			}
		}
		dp = ndp
	}

	ans := 0
	for j := 1; j <= m; j++ {
		ans = (ans + dp[j][j]) % MOD
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3, 4}, 10},
		{[]int{10, 20, 30}, 2},
		{[]int{1, 1, 1, 1}, 50},
	}

	for index, test := range tests {
		if subsequencePairCount(test.nums) != test.ans {
			fmt.Println("test failed", index)
		}
	}
}
