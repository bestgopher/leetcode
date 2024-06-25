#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut s = [['C'; 3]; 3];
        let mut flag = 'A';
        for i in moves.iter() {
            s[i[0] as usize][i[1] as usize] = flag;

            if i[0] == 0 {
                if i[1] == 0 {
                    if (s[0][1] == flag && s[0][2] == flag)
                        || (s[1][0] == flag && s[2][0] == flag)
                        || (s[1][1] == flag && s[2][2] == flag)
                    {
                        return flag.to_string();
                    }
                } else if i[1] == 1 {
                    if (s[0][0] == flag && s[0][2] == flag) || (s[1][1] == flag && s[2][1] == flag)
                    {
                        return flag.to_string();
                    }
                } else {
                    if (s[0][1] == flag && s[0][0] == flag)
                        || (s[1][2] == flag && s[2][2] == flag)
                        || (s[1][1] == flag && s[2][0] == flag)
                    {
                        return flag.to_string();
                    }
                }
            } else if i[0] == 1 {
                if i[1] == 0 {
                    if (s[1][1] == flag && s[1][2] == flag) || (s[0][0] == flag && s[2][0] == flag)
                    {
                        return flag.to_string();
                    }
                } else if i[1] == 1 {
                    if (s[1][0] == flag && s[1][2] == flag)
                        || (s[0][1] == flag && s[2][1] == flag)
                        || (s[0][0] == flag && s[2][2] == flag)
                        || (s[0][2] == flag && s[2][0] == flag)
                    {
                        return flag.to_string();
                    }
                } else {
                    if (s[0][2] == flag && s[2][2] == flag) || (s[1][0] == flag && s[1][1] == flag)
                    {
                        return flag.to_string();
                    }
                }
            } else {
                if i[1] == 0 {
                    if (s[2][1] == flag && s[2][2] == flag)
                        || (s[0][0] == flag && s[1][0] == flag)
                        || s[1][1] == flag && s[0][2] == flag
                    {
                        return flag.to_string();
                    }
                } else if i[1] == 1 {
                    if (s[2][0] == flag && s[2][2] == flag) || (s[1][1] == flag && s[0][1] == flag)
                    {
                        return flag.to_string();
                    }
                } else {
                    if (s[2][0] == flag && s[2][1] == flag)
                        || (s[1][2] == flag && s[0][2] == flag)
                        || (s[0][0] == flag && s[1][1] == flag)
                    {
                        return flag.to_string();
                    }
                }
            }

            if flag == 'A' {
                flag = 'B';
            } else {
                flag = 'A';
            }
        }

        if moves.len() < 9 {
            "Pending".into()
        } else {
            "Draw".into()
        }
    }
}
