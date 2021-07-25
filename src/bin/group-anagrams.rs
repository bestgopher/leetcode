fn main() {}

struct Solution;

impl Solution {
    /// 排序
    pub fn group_anagrams1(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = std::collections::HashMap::<Vec<u8>, Vec<String>>::new();

        for i in strs.into_iter() {
            let mut s = i.clone().into_bytes();
            s.sort();

            hash.entry(s)
                .and_modify(|x| x.push(i.clone()))
                .or_insert(vec![i]);
        }

        hash.into_iter().map(|(_x, y)| y).collect()
    }

    // 计算字母出现的个数
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash = std::collections::HashMap::<[u8; 26], Vec<String>>::new();

        for i in strs.into_iter() {
            let mut v = [0; 26];

            for &j in i.as_bytes() {
                v[(j - b'a') as usize] += 1;
            }

            if let Some(x) = hash.get_mut(&v) {
                x.push(i);
            } else {
                hash.insert(v, vec![i]);
            }
        }

        hash.into_iter().map(|(_x, y)| y).collect()
    }
}
