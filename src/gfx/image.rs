pub trait ImageLoader {}

pub struct ImageManager<T: ImageLoader> {
  loader: T,
}

impl<T: ImageLoader> ImageLoader for ImageManager<T> {}
