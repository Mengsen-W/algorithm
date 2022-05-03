/*
 * @Date: 2022-05-03 07:51:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-03 08:01:08
 * @FilePath: /algorithm/937_reorder_log_files/reorder_log_files.go
 */

package main

import (
	"reflect"
	"sort"
	"strings"
	"unicode"
)

func reorderLogFiles(logs []string) []string {
	sort.SliceStable(logs, func(i, j int) bool {
		s, t := logs[i], logs[j]
		s1 := strings.SplitN(s, " ", 2)[1]
		s2 := strings.SplitN(t, " ", 2)[1]
		isDigit1 := unicode.IsDigit(rune(s1[0]))
		isDigit2 := unicode.IsDigit(rune(s2[0]))
		if isDigit1 && isDigit2 {
			return false
		}
		if !isDigit1 && !isDigit2 {
			return s1 < s2 || s1 == s2 && s < t
		}
		return !isDigit1
	})
	return logs
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(reorderLogFiles(
		[]string{"dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"}),
		[]string{"let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"})
	assert(reorderLogFiles(
		[]string{"a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"}),
		[]string{"g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"})
}
