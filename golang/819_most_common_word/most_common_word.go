/*
 * @Date: 2022-04-17 09:05:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-17 09:25:47
 * @FilePath: /algorithm/819_most_common_word/most_common_word.go
 */

package main

import (
	"reflect"
	"unicode"
)

func mostCommonWord(paragraph string, banned []string) string {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	ban := map[string]bool{}
	for _, s := range banned {
		ban[s] = true
	}
	freq := map[string]int{}
	maxFreq := 0
	var word []byte
	for i, n := 0, len(paragraph); i <= n; i++ {
		if i < n && unicode.IsLetter(rune(paragraph[i])) {
			word = append(word, byte(unicode.ToLower(rune(paragraph[i]))))
		} else if word != nil {
			s := string(word)
			if !ban[s] {
				freq[s]++
				maxFreq = max(maxFreq, freq[s])
			}
			word = nil
		}
	}
	for s, f := range freq {
		if f == maxFreq {
			return s
		}
	}
	return ""
}

func main() {
	func(a, b string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}(mostCommonWord("Bob hit a ball, the hit BALL flew far after it was hit.", []string{"hit"}), "ball")
}
