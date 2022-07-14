/*
 * @Date: 2022-07-14
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-14
 * @FilePath: /algorithm/745_word_filter/word_filter.go
 */

package main

type WordFilter map[string]int

func Constructor(words []string) WordFilter {
	wf := WordFilter{}
	for i, word := range words {
		for j, n := 1, len(word); j <= n; j++ {
			for k := 0; k < n; k++ {
				wf[word[:j]+"#"+word[k:]] = i
			}
		}
	}
	return wf
}

func (wf WordFilter) F(pref, suff string) int {
	if i, ok := wf[pref+"#"+suff]; ok {
		return i
	}
	return -1
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	w := Constructor([]string{"apple"})
	assert(w.F("a", "e") == 0)
}
