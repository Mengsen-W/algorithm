/*
 * @Date: 2021-03-07 09:30:21
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-07 09:44:02
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

vector<vector<int>> hw;
vector<vector<string>> res;
vector<string> tmp;
void dfs(int x, string &s, int &n) {
  if (x == n) res.push_back(tmp);
  for (int i = 0; i < hw[x].size(); i++) {
    tmp.push_back(s.substr(x, hw[x][i]));
    dfs(x + hw[x][i], s, n);
    tmp.pop_back();
  }
}

vector<vector<string>> partition(string &&s) {
  int n = s.size();
  hw.resize(n + 1);
  for (int i = n - 1; i >= 0; i--) {
    hw[i].push_back(1);
    if (i + 1 < n && s[i] == s[i + 1]) hw[i].push_back(2);
    for (int j = 0; j < hw[i + 1].size(); j++) {
      int index = i + hw[i + 1][j] + 1;
      if (index < n && s[i] == s[index]) hw[i].push_back(index - i + 1);
    }
  }
  dfs(0, s, n);
  return res;
}

int main() {
  assert(partition(string("aab")) ==
         move(vector<vector<string>>{{"a", "a", "b"}, {"aa", "b"}}));
  return 0;
}