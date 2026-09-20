// Package main ...
package main

import "fmt"

func reverseDegree(s string) int {
	ans := 0
	for i := 1; i <= len(s); i++ {
		ans += (26 - int(s[i-1]-'a')) * i
	}
	return ans
}

func main() {
	tests := []struct {
		s   string
		ans int
	}{
		{"abc", 148},
		{"zaza", 160},
	}

	for index, test := range tests {
		if reverseDegree(test.s) != test.ans {
			fmt.Println(index)
		}
	}
}
