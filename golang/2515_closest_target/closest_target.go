// package main ...
package main

import "fmt"

func closestTarget(words []string, target string, startIndex int) int {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	ans := len(words)
	n := len(words)

	for i, word := range words {
		if word == target {
			dist := abs(i - startIndex)
			ans = min(ans, min(dist, n-dist))
		}
	}

	if ans < n {
		return ans
	}
	return -1
}

func main() {
	tests := []struct {
		words      []string
		target     string
		startIndex int
		ans        int
	}{
		{[]string{"hello", "i", "am", "leetcode", "hello"}, "hello", 1, 1},
		{[]string{"a", "b", "leetcode"}, "leetcode", 0, 1},
		{[]string{"i", "eat", "leetcode"}, "ate", 0, -1},
	}

	for index, test := range tests {
		if closestTarget(test.words, test.target, test.startIndex) != test.ans {
			fmt.Printf("%d\n", index)
		}
	}
}
