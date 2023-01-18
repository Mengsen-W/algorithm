/*
 * @Date: 2022-04-10 09:53:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-10 10:02:45
 * @FilePath: /algorithm/804_unique_morse_representations/unique_morse_representations.go
 */

package main

import "strings"

var morse = []string{
	".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
	"....", "..", ".---", "-.-", ".-..", "--", "-.",
	"---", ".--.", "--.-", ".-.", "...", "-", "..-",
	"...-", ".--", "-..-", "-.--", "--..",
}

func uniqueMorseRepresentations(words []string) int {
	set := map[string]struct{}{}
	for _, word := range words {
		trans := &strings.Builder{}
		for _, ch := range word {
			trans.WriteString(morse[ch-'a'])
		}
		set[trans.String()] = struct{}{}
	}
	return len(set)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(uniqueMorseRepresentations([]string{"gin", "zen", "gig", "msg"}) == 2)
	assert(uniqueMorseRepresentations([]string{"a"}) == 1)
}
