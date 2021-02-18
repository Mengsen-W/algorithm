/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-18 09:50:43
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-18 09:54:12
 */

#include <semaphore.h>

#include <functional>
#include <future>

using namespace std;

class foo_promise {
 public:
  foo_promise() {}

  void first(function<void()> printFirst) {
    // printFirst() outputs "first". Do not change or remove this line.
    printFirst();
    p1.set_value();
  }

  void second(function<void()> printSecond) {
    // printSecond() outputs "second". Do not change or remove this line.
    p1.get_future().wait();
    printSecond();
    p2.set_value();
  }

  void third(function<void()> printThird) {
    // printThird() outputs "third". Do not change or remove this line.
    p2.get_future().wait();
    printThird();
  }

 private:
  std::promise<void> p1;
  std::promise<void> p2;
};

class foo_mutex {
 public:
  foo_mutex() {
    m2.lock();
    m3.lock();
  }

  void first(function<void()> printFirst) {
    printFirst();
    m2.unlock();
  }

  void second(function<void()> printSecond) {
    m2.lock();
    printSecond();
    m3.unlock();
  }

  void third(function<void()> printThird) {
    m3.lock();
    printThird();
    m3.unlock();
  }

 private:
  std::mutex m2, m3;
};

class foo_sem {
 protected:
  sem_t firstJobDone;
  sem_t secondJobDone;

 public:
  foo_sem() {
    sem_init(&firstJobDone, 0, 0);
    sem_init(&secondJobDone, 0, 0);
  }

  void first(function<void()> printFirst) {
    // printFirst() outputs "first".
    printFirst();
    sem_post(&firstJobDone);
  }

  void second(function<void()> printSecond) {
    sem_wait(&firstJobDone);
    // printSecond() outputs "second".
    printSecond();
    sem_post(&secondJobDone);
  }

  void third(function<void()> printThird) {
    sem_wait(&secondJobDone);
    // printThird() outputs "third".
    printThird();
  }
};

class foo_con {
 public:
  foo_con() {
    firstReady = false;
    secondReady = false;
  }

  bool firstReady, secondReady;
  mutex mtx;
  condition_variable cv1, cv2;

  void first(function<void()> printFirst) {
    lock_guard<mutex> l(mtx);
    // printFirst() outputs "first". Do not change or remove this line.
    printFirst();
    firstReady = true;
    cv1.notify_one();
  }

  void second(function<void()> printSecond) {
    unique_lock<mutex> ul(mtx);
    cv1.wait(ul, [&] { return this->firstReady; });

    // printSecond() outputs "second". Do not change or remove this line.
    printSecond();
    secondReady = true;
    cv2.notify_one();
  }

  void third(function<void()> printThird) {
    unique_lock<mutex> ul(mtx);
    cv2.wait(ul, [&] { return this->secondReady; });
    // printThird() outputs "third". Do not change or remove this line.
    printThird();
  }
};

class foo_ato {
 public:
  foo_ato() {
    std::atomic_init(&this->firstReady, false);
    std::atomic_init(&this->secondReady, false);
  }

  std::atomic<bool> firstReady;
  std::atomic<bool> secondReady;

  void first(function<void()> printFirst) {
    // printFirst() outputs "first". Do not change or remove this line.
    printFirst();
    firstReady.store(true);
  }

  void second(function<void()> printSecond) {
    while (!firstReady.load()) std::this_thread::yield();
    // printSecond() outputs "second". Do not change or remove this line.
    printSecond();
    secondReady.store(true);
  }

  void third(function<void()> printThird) {
    while (!secondReady.load()) std::this_thread::yield();
    // printThird() outputs "third". Do not change or remove this line.
    printThird();
  }
};