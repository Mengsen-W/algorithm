/*
 * @Date: 2023-03-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-24
 * @FilePath: /algorithm/rust/1032_StreamChecker/StreamChecker.rs
 */

struct Trie<const N: usize>(Box<([Option<Trie<N>>; N], bool)>);

impl<const N: usize> Trie<N> {
    fn new() -> Self {
        // 这里Default::default()并不能初始化大于32的[None;N]
        // 而直接写[None;N]会因为None不是Copy而出错
        // 可以用泛型定义一个None常量，然后用[常量;N]初始化
        // 也可以像这里一样，使用一个神奇的技巧。
        Self(Box::new(([(); N].map(|_| None), false)))
    }

    fn insert(&mut self, word: String) {
        let mut ptr = self;
        for c in word.bytes().rev() {
            if ptr.0 .0[(c - b'a') as usize].is_none() {
                ptr.0 .0[(c - b'a') as usize] = Some(Trie::new());
            }
            ptr = ptr.0 .0[(c - b'a') as usize].as_mut().unwrap();
        }
        ptr.0 .1 = true;
    }

    fn search(&self, stream: &[u8]) -> bool {
        let mut ptr = self;
        for c in stream.iter().rev() {
            if ptr.0 .0[(c - b'a') as usize].is_some() {
                ptr = ptr.0 .0[(c - b'a') as usize].as_ref().unwrap();
                if ptr.0 .1 {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie<26>,
    stream: Vec<u8>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        Self {
            trie: trie,
            stream: vec![],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);
        self.trie.search(&self.stream)
    }
}

fn main() {
    let words = vec!["cd", "f", "kl"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut s = StreamChecker::new(words);
    assert_eq!(s.query('a'), false);
    assert_eq!(s.query('b'), false);
    assert_eq!(s.query('c'), false);
    assert_eq!(s.query('d'), true);
    assert_eq!(s.query('e'), false);
    assert_eq!(s.query('f'), true);
    assert_eq!(s.query('h'), false);
    assert_eq!(s.query('i'), false);
    assert_eq!(s.query('j'), false);
    assert_eq!(s.query('k'), false);
    assert_eq!(s.query('l'), true);
}
