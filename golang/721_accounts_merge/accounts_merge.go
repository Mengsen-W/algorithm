// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func accountsMerge(accounts [][]string) (ans [][]string) {
	emailToIndex := map[string]int{}
	emailToName := map[string]string{}
	for _, account := range accounts {
		name := account[0]
		for _, email := range account[1:] {
			if _, has := emailToIndex[email]; !has {
				emailToIndex[email] = len(emailToIndex)
				emailToName[email] = name
			}
		}
	}

	parent := make([]int, len(emailToIndex))
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	union := func(from, to int) {
		parent[find(from)] = find(to)
	}

	for _, account := range accounts {
		firstIndex := emailToIndex[account[1]]
		for _, email := range account[2:] {
			union(emailToIndex[email], firstIndex)
		}
	}

	indexToEmails := map[int][]string{}
	for email, index := range emailToIndex {
		index = find(index)
		indexToEmails[index] = append(indexToEmails[index], email)
	}

	for _, emails := range indexToEmails {
		sort.Strings(emails)
		account := append([]string{emailToName[emails[0]]}, emails...)
		ans = append(ans, account)
	}
	return
}

func main() {
	tests := []struct {
		accounts [][]string
		ans      [][]string
	}{
		{
			[][]string{
				{"John", "johnsmith@mail.com", "john00@mail.com"},
				{"John", "johnnybravo@mail.com"},
				{"John", "johnsmith@mail.com", "john_newyork@mail.com"},
				{"Mary", "mary@mail.com"},
			},
			[][]string{
				{"John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"},
				{"John", "johnnybravo@mail.com"},
				{"Mary", "mary@mail.com"},
			},
		},
		{
			[][]string{
				{"Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"},
				{"Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"},
				{"Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"},
				{"Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"},
				{"Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"},
			},
			[][]string{
				{"Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"},
				{"Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"},
				{"Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"},
				{"Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"},
				{"Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"},
			},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, accountsMerge(test.accounts), index)
	}
}
