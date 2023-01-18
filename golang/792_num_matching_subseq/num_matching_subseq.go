/*
 * @Date: 2022-11-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-17
 * @FilePath: /algorithm/792_num_matching_subseq/num_matching_subseq.go
 */

package main

func numMatchingSubseq(s string, words []string) (ans int) {
	type pair struct{ i, j int }
	ps := [26][]pair{}
	for i, w := range words {
		ps[w[0]-'a'] = append(ps[w[0]-'a'], pair{i, 0})
	}
	for _, c := range s {
		q := ps[c-'a']
		ps[c-'a'] = nil
		for _, p := range q {
			p.j++
			if p.j == len(words[p.i]) {
				ans++
			} else {
				w := words[p.i][p.j] - 'a'
				ps[w] = append(ps[w], p)
			}
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		s := "abcde"
		words := []string{"a", "bb", "acd", "ace"}
		ans := 3
		assert(numMatchingSubseq(s, words) == ans)
	}

	{
		s := "dsahjpjauf"
		words := []string{"ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"}
		ans := 2
		assert(numMatchingSubseq(s, words) == ans)
	}
}
