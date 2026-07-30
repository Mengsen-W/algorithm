// Package main ...
package main

import "fmt"

func minimumPushes(word string) int {
	n := len(word)
	// 向上取整
	m := (n-1)/8 + 1
	return (m-1+1)*(m-1)/2*8 + (n-(m-1)*8)*m
}

func main() {
	tests := []struct{
		word string
		ans int
	}{
      {"abcde", 5},
      {"xycdefghij", 12},
	}

	for index, test := range tests {
		if minimumPushes(test.word) != test.ans {
			fmt.Println(index)
		}
	}
}
