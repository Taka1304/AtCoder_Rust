// https://atcoder.jp/contests/abc348/tasks/abc348_d
// D - Medicines on Grid

use proconio::{
  fastout, input,
  marker::{Chars, Usize1},
};

#[fastout]
pub fn main() {
  input! {
    h: usize, w: usize,
    a: [Chars; h],
    n: usize,
    rce: [(Usize1, Usize1, usize); n],
  }
  // 薬位置
  let mut rc = vec![vec![0; w]; h];
  for (r, c, e) in rce {
    rc[r][c] = e;
  }
  // スタートとゴール地点
  let (mut si, mut sj) = (0, 0);
  let (mut gi, mut gj) = (0, 0);
  for i in 0..h {
    for j in 0..w {
      if a[i][j] == 'S' {
        si = i;
        sj = j;
      }
      if a[i][j] == 'T' {
        gi = i;
        gj = j;
      }
    }
  }

  // スタートに薬が無かったら詰み
  if rc[si][sj] == 0 {
    println!("No");
    return;
  }

  let mut d = vec![vec![0; w]; h];
  d[si][sj] = rc[si][sj];
  let mut q = std::collections::BinaryHeap::new();
  q.push((rc[si][sj], si, sj));
  while let Some((e, i, j)) = q.pop() {
    println!("{}, {}, {}", e, i, j);
    if d[i][j] > e {
      continue;
    }
    if e == 0 {
      continue;
    }
    for &(di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let (ni, nj) = (i as i32 + di, j as i32 + dj);
      if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
        continue;
      }
      let (ni, nj) = (ni as usize, nj as usize);
      if a[ni][nj] == '#' {
        continue;
      }
      if (ni, nj) == (gi, gj) {
        println!("Yes");
        return;
      }
      let ne = rc[ni][nj].max(e - 1);
      if d[ni][nj] < ne {
        d[ni][nj] = ne;
        q.push((ne, ni, nj));
      }
    }
  }
  println!("{:?}", q);
  println!("No");
}
