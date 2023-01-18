/*
 * @Date: 2021-12-10 09:28:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-10 09:41:21
 */

package main

import "unicode"

func shortestCompletingWord(licensePlate string, words []string) (ans string) {
	cnt := [26]int{}
	for _, ch := range licensePlate {
		if unicode.IsLetter(ch) {
			cnt[unicode.ToLower(ch)-'a']++
		}
	}

next:
	for _, word := range words {
		c := [26]int{}
		for _, ch := range word {
			c[ch-'a']++
		}
		for i := 0; i < 26; i++ {
			if c[i] < cnt[i] {
				continue next
			}
		}
		if ans == "" || len(word) < len(ans) {
			ans = word
		}
	}
	return
}

func main() {
	assert := func(a, b string) {
		if a != b {
			panic("assert failed")
		}
	}

	assert(shortestCompletingWord(
		"1s3 PSt", []string{"step", "steps", "stripe", "stepple"}),
		"steps")
	assert(shortestCompletingWord(
		"1s3 456", []string{"looks", "pest", "stew", "show"}),
		"pest")
	assert(shortestCompletingWord(
		"Ah71752", []string{"suggest", "letter", "of", "husband",
			"easy", "education", "drug", "prevent",
			"writer", "old"}), "husband")
	assert(shortestCompletingWord(
		"OgEu755",
		[]string{"enough", "these", "play", "wide", "wonder", "box",
			"arrive", "money", "tax", "thus"}), "enough")
	assert(shortestCompletingWord(
		"iMSlpe4", []string{"claim", "consumer", "student", "camera",
			"public", "never", "wonder", "simple",
			"thought", "use"}), "simple")
}
