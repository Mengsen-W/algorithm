// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func validSequence(s, t string) []int {
	n, m := len(s), len(t)
	suf := make([]int, n+1)
	suf[n] = m
	for i, j := n-1, m-1; i >= 0; i-- {
		if j >= 0 && s[i] == t[j] {
			j--
		}
		suf[i] = j + 1
	}

	ans := make([]int, m)
	changed := false // 是否修改过
	j := 0
	for i := range s {
		if s[i] == t[j] || !changed && suf[i+1] <= j+1 {
			if s[i] != t[j] {
				changed = true
			}
			ans[j] = i
			j++
			if j == m {
				return ans
			}
		}
	}
	return nil
}

func main() {
	tests := []struct {
		s   string
		t   string
		ans []int
	}{
		{"vbcca", "abc", []int{0, 1, 2}},
		{"bacdc", "abc", []int{1, 2, 4}},
		{"aaaaaa", "aaabc", nil},
		{"abc", "ab", []int{0, 1}},
	}

	for _, test := range tests {
		ans := validSequence(test.s, test.t)
		if !reflect.DeepEqual(ans, test.ans) {
			fmt.Println("error:", test.s, test.t, ans, test.ans)
		} else {
			fmt.Println("success:", test.s, test.t, ans)
		}
	}
}
