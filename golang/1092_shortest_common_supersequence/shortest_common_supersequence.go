/*
 * @Date: 2023-03-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-28
 * @FilePath: /algorithm/golang/1092_shortest_common_supersequence/shortest_common_supersequence.go
 */

// Package main ...
package main

import "reflect"

func shortestCommonSupersequence(s, t string) string {
	min := func(a, b int) int {
		if b < a {
			return b
		}
		return a
	}
	n, m := len(s), len(t)
	// f[i+1][j+1] 表示 s 的前 i 个字母和 t 的前 j 个字母的最短公共超序列的长度
	f := make([][]int, n+1)
	for i := range f {
		f[i] = make([]int, m+1)
	}
	for j := 1; j < m; j++ {
		f[0][j] = j // 递归边界
	}
	for i := 1; i < n; i++ {
		f[i][0] = i // 递归边界
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if s[i] == t[j] { // 最短公共超序列一定包含 s[i]
				f[i+1][j+1] = f[i][j] + 1
			} else { // 取更短的组成答案
				f[i+1][j+1] = min(f[i][j+1], f[i+1][j]) + 1
			}
		}
	}

	ans := []byte{}
	i, j := n-1, m-1
	for i >= 0 && j >= 0 {
		if s[i] == t[j] { // 公共超序列一定包含 s[i]
			ans = append(ans, s[i])
			i--
			j-- // 相当于继续递归 makeAns(i - 1, j - 1)
		} else if f[i+1][j+1] == f[i][j+1]+1 {
			ans = append(ans, s[i])
			i-- // 相当于继续递归 makeAns(i - 1, j)
		} else {
			ans = append(ans, t[j])
			j-- // 相当于继续递归 makeAns(i, j - 1)
		}
	}
	for i, n := 0, len(ans); i < n/2; i++ {
		ans[i], ans[n-1-i] = ans[n-1-i], ans[i]
	}
	// 补上前面的递归边界
	return s[:i+1] + t[:j+1] + string(ans)
}

func main() {
	assert := func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	str1 := "abac"
	str2 := "cab"
	ans := "cabac"
	assert(shortestCommonSupersequence(str1, str2), ans)
}
