/*
 * @Date: 2022-04-21 09:52:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-21 09:53:20
 * @FilePath: /algorithm/824_to_goat_latin/to_goat_latin.go
 */

package main

import "strings"

var vowels = map[byte]struct{}{'a': {}, 'e': {}, 'i': {}, 'o': {}, 'u': {}, 'A': {}, 'E': {}, 'I': {}, 'O': {}, 'U': {}}

func toGoatLatin(sentence string) string {
	ans := &strings.Builder{}
	for i, cnt, n := 0, 1, len(sentence); i < n; i++ {
		if cnt > 1 {
			ans.WriteByte(' ')
		}
		start := i
		for i++; i < n && sentence[i] != ' '; i++ {
		}
		cnt++
		if _, ok := vowels[sentence[start]]; ok {
			ans.WriteString(sentence[start:i])
		} else {
			ans.WriteString(sentence[start+1 : i])
			ans.WriteByte(sentence[start])
		}
		ans.WriteByte('m')
		ans.WriteString(strings.Repeat("a", cnt))
	}
	return ans.String()
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(toGoatLatin("I speak Goat Latin") == "Imaa peaksmaaa oatGmaaaa atinLmaaaaa")
	assert(toGoatLatin("The quick brown fox jumped over the lazy dog") == "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa")
}
