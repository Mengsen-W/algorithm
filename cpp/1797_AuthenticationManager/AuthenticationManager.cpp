/*
 * @Date: 2023-02-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-09
 * @FilePath: /algorithm/cpp/1797_AuthenticationManager/AuthenticationManager.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>

using namespace std;

class AuthenticationManager {
 private:
  int timeToLive;
  unordered_map<string, int> mp;

 public:
  AuthenticationManager(int timeToLive) { this->timeToLive = timeToLive; }

  void generate(string tokenId, int currentTime) { mp[tokenId] = currentTime + timeToLive; }

  void renew(string tokenId, int currentTime) {
    if (mp.count(tokenId) && mp[tokenId] > currentTime) {
      mp[tokenId] = currentTime + timeToLive;
    }
  }

  int countUnexpiredTokens(int currentTime) {
    int res = 0;
    for (auto &[_, time] : mp) {
      if (time > currentTime) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  AuthenticationManager a{5};
  a.renew("aaa", 1);
  a.generate("aaa", 2);
  assert(a.countUnexpiredTokens(6) == 1);
  a.generate("bbb", 7);
  a.renew("aaa", 8);
  a.renew("bbb", 10);
  assert(a.countUnexpiredTokens(15) == 0);
}