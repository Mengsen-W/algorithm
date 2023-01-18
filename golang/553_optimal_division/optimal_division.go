/*
 * @Date: 2022-02-27 00:38:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-27 00:57:43
 * @FilePath: /algorithm/553_optimal_division/optimal_division.go
 * @Description: file content
 */

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func optimalDivision1(nums []int) string {
	type node struct {
		minVal, maxVal float64
		minStr, maxStr string
	}
	n := len(nums)
	dp := make([][]node, n)
	for i := range dp {
		dp[i] = make([]node, n)
		for j := range dp[i] {
			dp[i][j].minVal = 1e4
		}
	}

	for i, num := range nums {
		dp[i][i].minVal = float64(num)
		dp[i][i].maxVal = float64(num)
		dp[i][i].minStr = strconv.Itoa(num)
		dp[i][i].maxStr = strconv.Itoa(num)
	}
	for i := 1; i < n; i++ {
		for j := 0; j+i < n; j++ {
			for k := j; k < j+i; k++ {
				if dp[j][j+i].maxVal < dp[j][k].maxVal/dp[k+1][j+i].minVal {
					dp[j][j+i].maxVal = dp[j][k].maxVal / dp[k+1][j+i].minVal
					if k+1 == j+i {
						dp[j][j+i].maxStr = dp[j][k].maxStr + "/" + dp[k+1][j+i].minStr
					} else {
						dp[j][j+i].maxStr = dp[j][k].maxStr + "/(" + dp[k+1][j+i].minStr + ")"
					}
				}
				if dp[j][j+i].minVal > dp[j][k].minVal/dp[k+1][j+i].maxVal {
					dp[j][j+i].minVal = dp[j][k].minVal / dp[k+1][j+i].maxVal
					if k+1 == j+i {
						dp[j][j+i].minStr = dp[j][k].minStr + "/" + dp[k+1][j+i].maxStr
					} else {
						dp[j][j+i].minStr = dp[j][k].minStr + "/(" + dp[k+1][j+i].maxStr + ")"
					}
				}
			}
		}
	}
	return dp[0][n-1].maxStr
}

func optimalDivision2(nums []int) string {
	n := len(nums)
	if n == 1 {
		return strconv.Itoa(nums[0])
	}
	if n == 2 {
		return fmt.Sprintf("%d/%d", nums[0], nums[1])
	}
	ans := &strings.Builder{}
	ans.WriteString(fmt.Sprintf("%d/(%d", nums[0], nums[1]))
	for _, num := range nums[2:] {
		ans.WriteByte('/')
		ans.WriteString(strconv.Itoa(num))
	}
	ans.WriteByte(')')
	return ans.String()
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(optimalDivision1([]int{1000, 100, 10, 2}) == "1000/(100/10/2)")
	assert(optimalDivision2([]int{1000, 100, 10, 2}) == "1000/(100/10/2)")
}
