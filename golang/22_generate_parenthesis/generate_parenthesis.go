// Package main ...
package main

import (
	"bytes"
	"fmt"
	"reflect"
)

func generateParenthesis(n int) (ans []string) {
	path := []int{} // 记录左括号的下标

	// 目前填了 i 个括号
	// 这 i 个括号中的左括号个数 - 右括号个数 = balance
	var dfs func(int, int)
	dfs = func(i, balance int) {
		if len(path) == n {
			s := bytes.Repeat([]byte{')'}, n*2)
			for _, j := range path {
				s[j] = '('
			}
			ans = append(ans, string(s))
			return
		}
		// 枚举填 right=0,1,2,...,balance 个右括号
		for right := range balance + 1 {
			// 先填 right 个右括号，然后填 1 个左括号，记录左括号的下标 i+right
			path = append(path, i+right)
			dfs(i+right+1, balance-right+1)
			path = path[:len(path)-1] // 恢复现场
		}
	}

	dfs(0, 0)
	return
}

func main() {
	tests := []struct {
		n   int
		ans []string
	}{
		{3, []string{"((()))", "(()())", "(())()", "()(())", "()()()"}},
		{1, []string{"()"}},
	}

	for _, test := range tests {
		if !reflect.DeepEqual(generateParenthesis(test.n), test.ans) {
			fmt.Println("Test failed:", test.n)
		}
	}
}
