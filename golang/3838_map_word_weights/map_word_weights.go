// Package main ...
package main

import "fmt"

func mapWordWeights(words []string, weights []int) string {
	ans := make([]byte, 0, len(words))
	for _, word := range words {
		s := 0
		for _, c := range word {
			s += weights[c-'a']
		}
		ans = append(ans, byte('z'-s%26))
	}
	return string(ans)
}

func main() {
	tests := []struct {
		words   []string
		weights []int
		ans     string
	}{
		{
			[]string{"abcd", "def", "xyz"},
			[]int{5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2},
			"rij",
		},
		{
			[]string{"a", "b", "c"},
			[]int{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
			"yyy",
		},
	}

	for _, test := range tests {
		ans := mapWordWeights(test.words, test.weights)
		if ans != test.ans {
			fmt.Printf("test failed: %v, expected: %v, got: %v\n", test, test.ans, ans)
		}
	}
}
