#include <cassert>
#include <string>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  using ll = long long;

  struct State {
    int prev, curr, tight, lead;
    ll cnt, sum;
  };

  ll solve(ll num) {
    // 数字小于 3 位波动值为 0
    if (num < 100) {
      return 0;
    }
    string s = to_string(num);
    int n = s.size();

    vector<State> currStates;
    // 数位 10 表示存在前导零时的无效无效状态
    currStates.push_back({10, 10, 1, 1, 1, 0});
    for (int pos = 0; pos < n; ++pos) {
      int limit = s[pos] - '0';
      ll cnt[2][2][11][11] = {0};
      ll sum[2][2][11][11] = {0};

      for (const auto& st : currStates) {
        int maxDigit = st.tight ? limit : 9;
        for (int digit = 0; digit <= maxDigit; ++digit) {
          int newLead = st.lead && (digit == 0);
          int newPrev = st.curr;
          int newCurr = newLead ? 10 : digit;
          int newTight = st.tight && (digit == maxDigit);

          ll add = 0;
          // 已有三位有效数字时才计算波动（prev和curr都有效，且不是前导零）
          if (!newLead && st.prev != 10 && st.curr != 10) {
            if ((st.prev < st.curr && st.curr > digit) || (st.prev > st.curr && st.curr < digit)) {
              add = st.cnt;
            }
          }

          cnt[newTight][newLead][newPrev][newCurr] += st.cnt;
          sum[newTight][newLead][newPrev][newCurr] += st.sum + add;
        }
      }

      // 收集合法状态
      vector<State> nextStates;
      for (int tight = 0; tight < 2; ++tight) {
        for (int lead = 0; lead < 2; ++lead) {
          for (int prev = 0; prev <= 10; ++prev) {
            for (int curr = 0; curr <= 10; ++curr) {
              ll c = cnt[tight][lead][prev][curr];
              ll s = sum[tight][lead][prev][curr];
              // 如何当前为有效状态，则进入下一轮计算
              if (c != 0) {
                nextStates.push_back({prev, curr, tight, lead, c, s});
              }
            }
          }
        }
      }

      currStates.swap(nextStates);
    }

    // 累加所有合法状态的波动值之和
    ll ans = 0;
    for (const auto& st : currStates) {
      ans += st.sum;
    }
    return ans;
  }

  long long totalWaviness(long long num1, long long num2) { return solve(num2) - solve(num1 - 1); }
};

int main() {
  vector<tuple<long, long, long>> tests{
      {120, 130, 3},
      {198, 202, 3},
      {4848, 4848, 2},
  };

  for (auto& [num1, num2, ans] : tests) {
    assert(Solution().totalWaviness(num1, num2) == ans);
  }
  return 0;
}