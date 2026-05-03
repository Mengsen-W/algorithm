// Package main...
package main

import "strings"

func rotateString(s, goal string) bool {
	return len(s) == len(goal) && strings.Contains(s+s, goal)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(rotateString("abcde", "cdeab") == true)
	assert(rotateString("abcde", "abced") == false)
}
