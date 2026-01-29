use smallvec::SmallVec;

pub fn pop_n_iter<'a, T, const N: usize>(v: &'a mut SmallVec<[T; N]>, n_poped: usize) -> smallvec::Drain<'a, [T; N]>
where
    [T; N]: smallvec::Array
{
    let new_len = v.len().saturating_sub(n_poped);
    v.drain(new_len..)
}
