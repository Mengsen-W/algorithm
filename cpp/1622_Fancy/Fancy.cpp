#include <cassert>
#include <vector>

using namespace std;

class Fancy {
 private:
  static constexpr int mod = 1000000007;
  vector<int> v;
  int a, b;

 public:
  Fancy() : a(1), b(0) {}

  // 快速幂
  int quickmul(int x, int y) {
    int ret = 1;
    int cur = x;
    while (y) {
      if (y & 1) {
        ret = (long long)ret * cur % mod;
      }
      cur = (long long)cur * cur % mod;
      y >>= 1;
    }
    return ret;
  }

  // 乘法逆元
  int inv(int x) { return quickmul(x, mod - 2); }

  void append(int val) { v.push_back((long long)((val - b + mod) % mod) * inv(a) % mod); }

  void addAll(int inc) { b = (b + inc) % mod; }

  void multAll(int m) {
    a = (long long)a * m % mod;
    b = (long long)b * m % mod;
  }

  int getIndex(int idx) {
    if (idx >= v.size()) {
      return -1;
    }
    int ans = ((long long)a * v[idx] % mod + b) % mod;
    return ans;
  }
};

int main() {
  Fancy fancy = Fancy();
  fancy.append(2);    // 奇妙序列：[2]
  fancy.addAll(3);    // 奇妙序列：[2+3] -> [5]
  fancy.append(7);    // 奇妙序列：[5, 7]
  fancy.multAll(2);   // 奇妙序列：[5*2, 7*2] -> [10, 14]
  assert(fancy.getIndex(0) == 10);  // 返回 10
  fancy.addAll(3);    // 奇妙序列：[10+3, 14+3] -> [13, 17]
  fancy.append(10);   // 奇妙序列：[13, 17, 10]
  fancy.multAll(2);   // 奇妙序列：[13*2, 17*2, 10*2] -> [26, 34, 20]
  assert(fancy.getIndex(0) == 26);  // 返回 26
  assert(fancy.getIndex(1) == 34);  // 返回 34
  assert(fancy.getIndex(2) == 20);  // 返回 20
}