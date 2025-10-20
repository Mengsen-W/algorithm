// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findLexSmallestString(s string, a int, b int) string {
	q := []string{s}
	vis := map[string]bool{s: true}
	ans := s
	n := len(s)
	for len(q) > 0 {
		s = q[0]
		q = q[1:]
		if ans > s {
			ans = s
		}
		t1 := []byte(s)
		for i := 1; i < n; i += 2 {
			t1[i] = byte((int(t1[i]-'0')+a)%10 + '0')
		}
		t2 := s[n-b:] + s[:n-b]
		for _, t := range []string{string(t1), t2} {
			if !vis[t] {
				vis[t] = true
				q = append(q, t)
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		s    string
		a    int
		b    int
		want string
	}{
		{"5525", 9, 2, "2050"},
		{"74", 5, 1, "24"},
		{"0011", 4, 2, "0011"},
		{"43987654", 7, 3, "00553311"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.want, findLexSmallestString(test.s, test.a, test.b), index)
	}
}
