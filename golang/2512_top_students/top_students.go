/*
 * @Date: 2023-10-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-11
 * @FilePath: /algorithm/golang/2512_top_students/top_students.go
 */

// Package main ...
package main

import (
	"sort"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func topStudents(positiveFeedback, negativeFeedback, report []string, studentId []int, k int) []int {
	words := map[string]int{}
	for _, w := range positiveFeedback {
		words[w] = 3
	}
	for _, w := range negativeFeedback {
		words[w] = -1
	}
	type pair struct{ score, id int }
	A := make([]pair, len(report))
	for i, r := range report {
		score := 0
		for _, w := range strings.Split(r, " ") {
			score += words[w]
		}
		A[i] = pair{score, studentId[i]}
	}
	sort.Slice(A, func(i, j int) bool {
		a, b := A[i], A[j]
		return a.score > b.score || a.score == b.score && a.id < b.id
	})
	res := make([]int, k)
	for i, p := range A[:k] {
		res[i] = p.id
	}
	return res
}

func main() {
	tests := []struct {
		positiveFeedback []string
		negativeFeedback []string
		report           []string
		studentId        []int
		k                int
		ans              []int
	}{
		{
			[]string{"smart", "brilliant", "studious"},
			[]string{"not"},
			[]string{"this student is studious", "the student is smart"},
			[]int{1, 2},
			2,
			[]int{1, 2},
		},
		{
			[]string{"smart", "brilliant", "studious"},
			[]string{"not"},
			[]string{"this student is not studious", "the student is smart"},
			[]int{1, 2},
			2,
			[]int{2, 1},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, topStudents(test.positiveFeedback, test.negativeFeedback, test.report, test.studentId, test.k), index)
	}
}
