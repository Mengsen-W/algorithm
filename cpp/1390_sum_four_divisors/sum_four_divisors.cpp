#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int sumFourDivisors(vector<int>& nums) {
    // C 是数组 nums 元素的上限，C3 是 C 的立方根
    int C = 100000, C3 = 46;

    vector<int> isprime(C + 1, 1);
    vector<int> primes;

    // 埃拉托斯特尼筛法
    for (int i = 2; i <= C; ++i) {
      if (isprime[i]) {
        primes.push_back(i);
      }
      for (int j = i + i; j <= C; j += i) {
        isprime[j] = 0;
      }
    }

    // 欧拉筛法
    /*
    for (int i = 2; i <= C; ++i) {
        if (isprime[i]) {
            primes.push_back(i);
        }
        for (int prime: primes) {
            if (i * prime > C) {
                break;
            }
            isprime[i * prime] = 0;
            if (i % prime == 0) {
                break;
            }
        }
    }
    */

    // 通过质数表构造出所有的四因数
    unordered_map<int, int> factor4;
    for (int prime : primes) {
      if (prime <= C3) {
        factor4[prime * prime * prime] = 1 + prime + prime * prime + prime * prime * prime;
      }
    }
    for (size_t i = 0; i < primes.size(); ++i) {
      for (size_t j = i + 1; j < primes.size(); ++j) {
        if (primes[i] <= C / primes[j]) {
          factor4[primes[i] * primes[j]] = 1 + primes[i] + primes[j] + primes[i] * primes[j];
        } else {
          break;
        }
      }
    }

    int ans = 0;
    for (int num : nums) {
      if (factor4.count(num)) {
        ans += factor4[num];
      }
    }
    return ans;
  }
};

int main() {
  vector<tuple<vector<int>, int>> tests{
      {{21, 4, 7}, 32},
      {{21, 21}, 64},
      {{1, 2, 3, 4, 5}, 0},
  };

  for (auto [nums, expected] : tests) {
    assert(Solution().sumFourDivisors(nums) == expected);
  }
}