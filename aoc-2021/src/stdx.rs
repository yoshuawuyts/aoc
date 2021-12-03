pub trait IterExt<T>: Iterator<Item = T>
where
    Self: Sized,
{
    fn array_windows<const N: usize>(&mut self) -> ArrayWindows<'_, T, Self, N> {
        ArrayWindows::new(self)
    }
}

impl<I, T> IterExt<T> for I where I: Iterator<Item = T> {}

pub struct ArrayWindows<'a, T, I, const N: usize>
where
    I: Iterator<Item = T>,
{
    iter: &'a mut I,
    cache: Option<[T; N]>,
}

impl<'a, T, I, const N: usize> ArrayWindows<'a, T, I, N>
where
    I: Iterator<Item = T>,
{
    fn new(iter: &'a mut I) -> Self {
        Self { iter, cache: None }
    }
}

impl<'a, 'b, T, I, const N: usize> Iterator for ArrayWindows<'a, T, I, N>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.cache.is_none() {
            // Handle the empty array case.
            if N == 0 {
                return None;
            }
            let arr: [Option<T>; N] = core::array::from_fn(|_| self.iter.next());
            self.cache = Some(maybe_array(arr)?);
        } else {
            // Access our numbers cache, but drop the first value.
            let mut cache = self.cache.take().unwrap().into_iter().skip(1);

            // Create a new array with the old values, and push the new value to
            // the end.
            let arr: [Option<T>; N] = core::array::from_fn(|i| {
                if i == (N - 1) {
                    self.iter.next()
                } else {
                    cache.next()
                }
            });
            self.cache = Some(maybe_array(arr)?);
        }

        self.cache.clone()
    }
}

// We need this because `array::try_from_fn` is not generic over `Try`.
//
// See: https://github.com/rust-lang/rust/issues/89379#issuecomment-983714215
fn maybe_array<T, const N: usize>(arr: [Option<T>; N]) -> Option<[T; N]> {
    for item in &arr {
        if item.is_none() {
            return None;
        }
    }
    Some(arr.map(|item| item.unwrap()))
}
