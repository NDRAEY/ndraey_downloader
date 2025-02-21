/// Function that receives downloaded byte count, total byte count (0 if no data), speed in bytes/second.
pub type Callback = fn(usize, usize, usize);
