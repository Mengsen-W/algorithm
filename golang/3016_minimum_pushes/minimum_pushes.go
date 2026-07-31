// Package main ...
package main

import (
	"fmt"
	"sort"
)

func minimumPushes(word string) int {
	freq := make([]int, 26)
	for _, c := range word {
		freq[c-'a']++
	}

	sort.Slice(freq, func(i, j int) bool { return freq[i] > freq[j] })

	ans := 0

	for i := 0; i < 26 && freq[i] > 0; i++ {
		ans += (i/8 + 1) * freq[i]
	}

	return ans
}

func main() {
	tests := []struct {
		word string
		ans  int
	}{
		{"abcde", 5},
		{"xyzxyzxyzxyz", 12},
		{"aabbccddeeffgghhiiiiii", 24},
	}

	for index, test := range tests {
		if minimumPushes(test.word) != test.ans {
			fmt.Print(index)
		}
	}
}
