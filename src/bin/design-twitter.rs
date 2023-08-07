#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

#[derive(Default)]
struct User {
    /// 关注者
    stars: std::collections::HashSet<i32>,
    /// 时间与postid
    posts: std::collections::LinkedList<(std::time::Instant, i32)>,
}

struct Twitter {
    users: std::collections::HashMap<i32, User>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.

* Your Twitter object will be instantiated and called as such:
* let obj = Twitter::new();
* obj.post_tweet(userId, tweetId);
* let ret_2: Vec<i32> = obj.get_news_feed(userId);
* obj.follow(followerId, followeeId);
* obj.unfollow(followerId, followeeId);
*/
impl Twitter {
    fn new() -> Self {
        Self {
            users: std::collections::HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let user = self.users.entry(user_id).or_insert_with(|| User::default());

        if user.posts.len() == 10 {
            user.posts.pop_back();
        }

        let now = std::time::Instant::now();

        user.posts.push_front((now, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let user = if let Some(x) = self.users.get(&user_id) {
            x
        } else {
            return vec![];
        };

        let mut x = std::collections::BinaryHeap::new();

        for i in user
            .stars
            .iter()
            .map_while(|x| self.users.get(x))
            .flat_map(|x| x.posts.iter())
            .chain(user.posts.iter())
        {
            x.push(std::cmp::Reverse(i.clone()));

            if x.len() == 11 {
                x.pop();
            }
        }

        let mut r = vec![0; x.len()];

        while let Some(p) = x.pop() {
            r[x.len()] = p.0 .1;
        }

        r
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_insert_with(|| User::default())
            .stars
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .and_modify(|user| {
                user.stars.remove(&followee_id);
            })
            .or_insert_with(|| User::default());
    }
}
