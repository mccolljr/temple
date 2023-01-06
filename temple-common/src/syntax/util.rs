use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct Lookahead<I, const N: usize>
where
    I: Iterator,
{
    iter: std::iter::Fuse<I>,
    peek: [Option<I::Item>; N],
}

impl<I, const N: usize> Lookahead<I, N>
where
    I: Iterator,
{
    const __NONE: Option<I::Item> = None;

    pub fn new(iter: I) -> Self {
        let mut iter = iter.fuse();
        let mut peek = [Self::__NONE; N];
        for i in 0..N {
            peek[i] = iter.next();
        }
        Self { iter, peek }
    }

    pub fn peek_nth(&self, n: usize) -> Option<&I::Item> {
        self.peek.get(n)?.as_ref()
    }
}

impl<I, const N: usize> Lookahead<I, N>
where
    I: Iterator,
    I::Item: Copy + Eq,
{
    pub fn has_next(&self, want: &[I::Item]) -> bool {
        want.len() <= N
            && want
                .iter()
                .zip(self.peek.iter().take(want.len()))
                .all(|(a, b)| Some(a) == b.as_ref())
    }

    pub fn with_next(&mut self, want: &[I::Item]) -> bool {
        if self.has_next(want) {
            self.skip_next(want.len());
            return true;
        }

        false
    }

    pub fn skip_next(&mut self, n: usize) {
        self.take(n).for_each(std::mem::drop)
    }
}

impl<I, const N: usize> Iterator for Lookahead<I, N>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek.rotate_left(1);
        return std::mem::replace(&mut self.peek[N - 1], self.iter.next());
    }
}

impl<I, const N: usize> FusedIterator for Lookahead<I, N> where I: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookahead_iter() {
        let mut lookahead = Lookahead::<_, 5>::new([1, 2, 3, 4, 5, 6].into_iter());

        assert_eq!(lookahead.peek_nth(0), Some(&1));
        assert_eq!(lookahead.peek_nth(1), Some(&2));
        assert_eq!(lookahead.peek_nth(2), Some(&3));
        assert_eq!(lookahead.peek_nth(3), Some(&4));
        assert_eq!(lookahead.peek_nth(4), Some(&5));

        assert_eq!(lookahead.next(), Some(1));
        assert_eq!(lookahead.peek_nth(0), Some(&2));
        assert_eq!(lookahead.peek_nth(1), Some(&3));
        assert_eq!(lookahead.peek_nth(2), Some(&4));
        assert_eq!(lookahead.peek_nth(3), Some(&5));
        assert_eq!(lookahead.peek_nth(4), Some(&6));

        assert_eq!(lookahead.next(), Some(2));
        assert_eq!(lookahead.peek_nth(0), Some(&3));
        assert_eq!(lookahead.peek_nth(1), Some(&4));
        assert_eq!(lookahead.peek_nth(2), Some(&5));
        assert_eq!(lookahead.peek_nth(3), Some(&6));
        assert_eq!(lookahead.peek_nth(4), None);

        assert_eq!(lookahead.next(), Some(3));
        assert_eq!(lookahead.peek_nth(0), Some(&4));
        assert_eq!(lookahead.peek_nth(1), Some(&5));
        assert_eq!(lookahead.peek_nth(2), Some(&6));
        assert_eq!(lookahead.peek_nth(3), None);
        assert_eq!(lookahead.peek_nth(4), None);

        assert_eq!(lookahead.next(), Some(4));
        assert_eq!(lookahead.peek_nth(0), Some(&5));
        assert_eq!(lookahead.peek_nth(1), Some(&6));
        assert_eq!(lookahead.peek_nth(2), None);
        assert_eq!(lookahead.peek_nth(3), None);
        assert_eq!(lookahead.peek_nth(4), None);

        assert_eq!(lookahead.next(), Some(5));
        assert_eq!(lookahead.peek_nth(0), Some(&6));
        assert_eq!(lookahead.peek_nth(1), None);
        assert_eq!(lookahead.peek_nth(2), None);
        assert_eq!(lookahead.peek_nth(3), None);
        assert_eq!(lookahead.peek_nth(4), None);

        assert_eq!(lookahead.next(), Some(6));
        assert_eq!(lookahead.peek_nth(0), None);
        assert_eq!(lookahead.peek_nth(1), None);
        assert_eq!(lookahead.peek_nth(2), None);
        assert_eq!(lookahead.peek_nth(3), None);
        assert_eq!(lookahead.peek_nth(4), None);

        assert_eq!(lookahead.next(), None);
        assert_eq!(lookahead.peek_nth(0), None);
        assert_eq!(lookahead.peek_nth(1), None);
        assert_eq!(lookahead.peek_nth(2), None);
        assert_eq!(lookahead.peek_nth(3), None);
        assert_eq!(lookahead.peek_nth(4), None);
    }
}
