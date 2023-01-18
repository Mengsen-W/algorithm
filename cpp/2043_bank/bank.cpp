/*
 * @Date: 2022-03-18 00:17:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-18 00:24:36
 * @FilePath: /algorithm/2043_bank/bank.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Bank {
 private:
  vector<long long> balance;

 public:
  Bank(vector<long long>& balance) : balance(balance) {}

  bool transfer(int account1, int account2, long long money) {
    if (account1 > balance.size() || account2 > balance.size() ||
        balance[account1 - 1] < money) {
      return false;
    }
    balance[account1 - 1] -= money;
    balance[account2 - 1] += money;
    return true;
  }

  bool deposit(int account, long long money) {
    if (account > balance.size()) {
      return false;
    }
    balance[account - 1] += money;
    return true;
  }

  bool withdraw(int account, long long money) {
    if (account > balance.size() || balance[account - 1] < money) {
      return false;
    }
    balance[account - 1] -= money;
    return true;
  }
};

int main() {
  vector<long long> balance{10, 100, 20, 50, 30};
  Bank b{balance};
  assert(b.withdraw(3, 10) == true);
  assert(b.transfer(5, 1, 20) == true);
  assert(b.deposit(5, 20) == true);
  assert(b.transfer(3, 4, 15) == false);
  assert(b.withdraw(10, 50) == false);
}