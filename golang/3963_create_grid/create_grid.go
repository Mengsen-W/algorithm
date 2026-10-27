// Package main ...
package main

import (
	"fmt"
	"reflect"
	"strings"
)

func createGrid(m, n int) []string {
	ans := make([]string, m)
	ans[0] = strings.Repeat(".", n)
	row := strings.Repeat("#", n-1) + "." // 避免在循环中反复创建字符串
	for i := 1; i < m; i++ {
		ans[i] = row
	}
	return ans
}

func main() {
	tests := []struct {
		m   int
		n   int
		ans []string
	}{
		{2, 3, []string{"..#", "#.."}},
		{3, 3, []string{"..#", "#..", "##."}},
		{1, 4, []string{"...."}},
	}

	for _, tt := range tests {
		if got := createGrid(tt.m, tt.n); !reflect.DeepEqual(got, tt.ans) {
			fmt.Printf("got: %v, want: %v", got, tt.ans)
		}
	}
}
