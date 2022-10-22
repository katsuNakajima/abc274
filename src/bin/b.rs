use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

trait Transpose<'a, Elem, Iter, T>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter>;
}

impl<'a, Elem, Iter, T> Transpose<'a, Elem, Iter, T> for T
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
    T: IntoIterator<Item = Iter>,
{
    fn transpose(self) -> Transposed<'a, Elem, Iter> {
        Transposed {
            iters: self.into_iter().map(IntoIterator::into_iter).collect(),
        }
    }
}

struct Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    iters: Vec<Iter::IntoIter>,
}

impl<'a, Elem, Iter> Iterator for Transposed<'a, Elem, Iter>
where
    Elem: 'a,
    Iter: IntoIterator<Item = &'a Elem>,
{
    type Item = Vec<&'a Elem>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(Iterator::next).collect()
    }
}

fn main() {
    let (h, _w) = parse_line!(usize, usize);
    let mut c = Vec::new();
    for _i in 0..h {
        let ci: Vec<char> = parse_line!(String).chars().collect();
        c.push(ci);
    }
    let ct: Vec<Vec<&char>> = c.iter().transpose().collect();
    for x in ct {
        let mut ans = HashMap::new();
        for y in x {
            let cnt = ans.entry(y).or_insert(0);
            *cnt += 1;
        }
        let ansp = ans.get(&'#').unwrap_or(&0);
        print!("{:?} ", ansp);
    }
}
