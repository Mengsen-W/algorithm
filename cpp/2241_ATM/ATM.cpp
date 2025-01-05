#include <cassert>
#include <vector>

using namespace std;

class ATM {
 private:
  vector<long long> cnt;    // 每张钞票剩余数量
  vector<long long> value;  // 每张钞票面额

 public:
  ATM() {
    cnt = {0, 0, 0, 0, 0};
    value = {20, 50, 100, 200, 500};
  }

  void deposit(vector<int> banknotesCount) {
    for (int i = 0; i < 5; ++i) {
      cnt[i] += banknotesCount[i];
    }
  }

  vector<int> withdraw(int amount) {
    vector<int> res(5);
    // 模拟尝试取出钞票的过程
    for (int i = 4; i >= 0; --i) {
      res[i] = min(cnt[i], amount / value[i]);
      amount -= res[i] * value[i];
    }
    if (amount) {
      // 无法完成该操作
      return {-1};
    } else {
      // 可以完成该操作
      for (int i = 0; i < 5; ++i) {
        cnt[i] -= res[i];
      }
      return res;
    }
  }
};

int main() {
  ATM *atm = new ATM();
  atm->deposit({0, 0, 1, 2, 1});  // 存入 1 张 $100 ，2 张 $200 和 1 张 $500 的钞票。
  assert((atm->withdraw(600) == vector<int>{0, 0, 1, 0, 1}));
  atm->deposit({0, 1, 0, 1, 1});
  assert((atm->withdraw(600) == vector<int>{-1}));
  assert((atm->withdraw(550) == vector<int>{0, 1, 0, 0, 1}));
}
