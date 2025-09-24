#include <cassert>
#include <string>
#include <stack>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    string clearStars(string s) {
        vector<stack<int>> cnt(26);
        for (int i = 0; i < s.size(); i++) {
            if (s[i] != '*') {
                cnt[s[i] - 'a'].push(i);
            } else {
                for (int j = 0; j < 26; j++) {
                    if (!cnt[j].empty()) {
                        s[cnt[j].top()] = '*';
                        cnt[j].pop();
                        break;
                    }
                }
            }
        }

        string ans;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] != '*') {
                ans.push_back(s[i]);
            }
        }
        return ans;
    }
};

int main() {
  vector<tuple<string, string>> tests {
    {"aaba*", "aab"},
    {"abc", "abc"},
  };

  for (auto &[s, ans] : tests) {
    assert(Solution().clearStars(s) == ans);
  }
}