/*
 * @Date: 2021-07-18 16:52:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-18 19:17:13
 */

package main

import "fmt"

func groupAnagrams(strs []string) [][]string {
	mp := map[[26]int][]string{}
	for _, str := range strs {
		cnt := [26]int{}
		for _, b := range str {
			cnt[b-'a']++
		}
		mp[cnt] = append(mp[cnt], str)
	}
	ans := make([][]string, 0, len(mp))
	for _, v := range mp {
		ans = append(ans, v)
	}
	return ans
}

func groupAnagrams_2(strs []string) [][]string {
	m := map[uint64][]string{}
	pf := [26]uint64{}
	for i := 0; i < 26; i++ {
		pf[i] = 1
	}
	var b uint64 = 97755331
	for i := 1; i < 26; i++ {
		pf[i] = pf[i-1] * b
	}

	for _, t := range strs {
		var hash uint64 = 0
		for _, c := range t {
			hash += pf[c-'a'] * uint64(c)
		}
		m[hash] = append(m[hash], t)
	}
	ans := [][]string{}
	for _, v := range m {
		ans = append(ans, v)
	}
	return ans
}

func main() {
	strs := []string{"eat", "tea", "tan", "ate", "nat", "bat"}
	fmt.Printf("%v\n", groupAnagrams(strs))
	fmt.Printf("%v\n", groupAnagrams_2(strs))
}
