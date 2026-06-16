// Package main ...
package main

import "fmt"

func processStr(s string) string {
	var result []rune
	for _, ch := range s {
		switch ch {
		case '*':
			if len(result) > 0 {
				result = result[:len(result)-1]
			}
		case '#':
			copyPart := make([]rune, len(result))
			copy(copyPart, result)
			result = append(result, copyPart...)
		case '%':
			for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
				result[i], result[j] = result[j], result[i]
			}
		default:
			result = append(result, ch)
		}
	}
	return string(result)
}

func main() {
	tests := []struct {
		s   string
		ans string
	}{
		{"a#b%*", "ba"},
		{"z*#", ""},
	}

	for _, test := range tests {
		result := processStr(test.s)
		if result != test.ans {
			fmt.Printf("Failed: %s, expected: %s, got: %s\n", test.s, test.ans, result)
		}
	}
}
