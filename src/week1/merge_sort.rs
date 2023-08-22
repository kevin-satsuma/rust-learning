pub fn main(v: &mut Vec<i32>) {
    let end = v.len() - 1;
    let mut scratch = vec![0; v.len()];
    sort(v, 0, end, &mut scratch);
}

pub fn sort(v: &mut Vec<i32>, s: usize, e: usize, scratch: &mut Vec<i32>) {
    let m = (s + e) / 2;
    if (e - s) < 1 {
        return
    }

    sort(v, s, m, scratch);
    sort(v, m+1, e, scratch);

    for k in s..=m {
        scratch[k] = v[k]
    }
    for k in m+1..=e {
        scratch[k] = v[k]
    }

    let mut i: usize = s;
    let mut j: usize = m+1;

    for k in s..=e {
        if (i <= m) && (j <= e) {
            if scratch[i] < scratch[j] {
                v[k] = scratch[i];
                i += 1
            } else {
                v[k] = scratch[j];
                j += 1
            }
        } else if i <= m {
            v[k] = scratch[i];
            i += 1
        } else {
            v[k] = scratch[j];
            j += 1
        }
    }
}