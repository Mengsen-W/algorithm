/*
 * @Date: 2023-02-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-09
 * @FilePath: /algorithm/golang/1797_AuthenticationManager/AuthenticationManager.go
 */

package main

type AuthenticationManager struct {
	mp  map[string]int
	ttl int
}

func Constructor(timeToLive int) AuthenticationManager {
	return AuthenticationManager{map[string]int{}, timeToLive}
}

func (m *AuthenticationManager) Generate(tokenId string, currentTime int) {
	m.mp[tokenId] = currentTime
}

func (m *AuthenticationManager) Renew(tokenId string, currentTime int) {
	if v, ok := m.mp[tokenId]; ok && v+m.ttl > currentTime {
		m.mp[tokenId] = currentTime
	}
}

func (m *AuthenticationManager) CountUnexpiredTokens(currentTime int) (ans int) {
	for _, t := range m.mp {
		if t+m.ttl > currentTime {
			ans++
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
	a := Constructor(5)
	a.Renew("aaa", 1)
	a.Generate("aaa", 2)
	assert(a.CountUnexpiredTokens(6) == 1)
	a.Generate("bbb", 7)
	a.Renew("aaa", 8)
	a.Renew("bbb", 10)
	assert(a.CountUnexpiredTokens(15) == 0)
}
