use std::{
	cell::UnsafeCell, ptr::null_mut, slice::from_raw_parts,
	sync::atomic::AtomicPtr
};

/// Dynamically run-time sized [`Chunk`].
pub type UnsizedChunk = Chunk;

/// Statically compile-time sized [`Chunk`].
pub type SizedChunk<const N: usize> = Chunk<[u8; N]>;

/// Representation of a Minecraft chunk, a 16x16 region of blocks.
///
/// `buffer` Layout
/// ---------------
/// The [`buffer`](Self::buffer) contains the chunk block palette and the data
/// of the blocks inside of the chunk. All aspects of `buffer` described here
/// are invariants, and can be relied upon by `unsafe` code.
///
/// The chunk block palette is the first data encountered in the buffer.
/// Where *`P`* is [`palette_size`](Self::palette_size), the first
/// [`u64::BITS`]` / 8 * `*`P`* octets in the buffer is the chunk block palette.
/// The chunk block palette may be retrieved using the
/// [`block_palette`](Self::block_palette) method.
///
/// Immediately following the chunk block palette is the chunk data, a list of
/// indexes into the chunk block palette. Each item in the list is
/// `⌈log₂`[`palette_size`](Self::palette_size)`⌉` bits long, calculatable with
/// the [`bits_per_block`](Self::bits_per_block) method. There are exactly
/// 4096 items in the chunk data list[^1].
///
/// [^1]: This is very likely subject to change.
// TODO: Chunk height?
#[derive(Debug)]
pub struct Chunk<B: ?Sized = [u8]>
where
	B: AsRef<[u8]> + AsMut<[u8]>
{
	/// The size of the chunk block palette.
	palette_size: usize,

	mutability_guard: AtomicPtr<()>,

	/// The dynamically sized data buffer. This buffer contains the palette, and
	/// the chunk data. Read the [struct level documentation](Self) for a more
	/// indepth explanation.
	buffer: UnsafeCell<B>
}

/// Generic implementation of [Chunk], where B can be sized, or unsized.
impl<B: ?Sized> Chunk<B>
where
	B: AsRef<[u8]> + AsMut<[u8]>
{
	pub unsafe fn from_raw_parts(palette_size: usize, buffer: B) -> Self
	where
		B: Sized
	{
		Self {
			palette_size,
			mutability_guard: AtomicPtr::new(null_mut()),
			buffer: UnsafeCell::new(buffer)
		}
	}

	/// The number of bits per block in the chunk data.
	pub fn bits_per_block(&self) -> usize {
		(self.palette_size as f32).log2().ceil() as usize
	}

	/// The block palette.
	pub fn block_palette(&mut self) -> &[u64] {
		let buffer = self.buffer.get_mut().as_mut();
		debug_assert!(
			buffer.len() >= self.palette_size * (u64::BITS as usize / 8),
			"invariant: chunk buffer was smaller than calculated palette buffer size"
		);

		// SAFETY: This invariant is upheld by the buffer layout.
		unsafe {
			from_raw_parts(buffer as *const _ as *const u64, self.palette_size)
		}
	}
}

/// Constructor implementations for [Chunk].
impl UnsizedChunk {
	pub fn new() -> Box<Self> {
		Box::new(SizedChunk::<0> {
			palette_size: 0,
			mutability_guard: AtomicPtr::new(null_mut()),
			buffer: UnsafeCell::new([])
		}) as _
	}
}

// SAFETY: UnsafeCell is already unsafe to use. This trait is just another
// addition to the safety contract that we must uphold when using it.
unsafe impl<B: ?Sized> Sync for Chunk<B> where B: AsRef<[u8]> + AsMut<[u8]> {}

pub(crate) struct MutabilityGuard<const N: usize>([*const [u8]; N]);

#[cfg(test)]
mod tests {
	use super::Chunk;

	#[test]
	fn chunk_bits_per_block() {
		let mut chunk = unsafe { Chunk::from_raw_parts(0, []) };
		assert_eq!(chunk.bits_per_block(), 0);

		chunk.palette_size = 2;
		assert_eq!(chunk.bits_per_block(), 1);
		chunk.palette_size = 3;
		assert_eq!(chunk.bits_per_block(), 2);

		chunk.palette_size = 10;
		assert_eq!(chunk.bits_per_block(), 4);

		chunk.palette_size = 16;
		assert_eq!(chunk.bits_per_block(), 4);
		chunk.palette_size = 17;
		assert_eq!(chunk.bits_per_block(), 5);

		chunk.palette_size = 256;
		assert_eq!(chunk.bits_per_block(), 8);
		chunk.palette_size = 257;
		assert_eq!(chunk.bits_per_block(), 9);

		chunk.palette_size = u16::MAX as usize;
		assert_eq!(chunk.bits_per_block(), u16::BITS as usize);
		chunk.palette_size = u32::MAX as usize;
		assert_eq!(chunk.bits_per_block(), u32::BITS as usize);
		chunk.palette_size = u8::MAX as usize;
		assert_eq!(chunk.bits_per_block(), u8::BITS as usize);
	}

	#[test]
	fn chunk_block_palette() {
		let mut chunk = unsafe { Chunk::from_raw_parts(0, []) };
		assert_eq!(chunk.block_palette(), &[]);

		let mut chunk = unsafe { Chunk::from_raw_parts(1, 0u64.to_ne_bytes()) };
		assert_eq!(chunk.block_palette(), &[0]);

		let mut buffer = Vec::new();
		buffer.extend(0u64.to_ne_bytes());
		buffer.extend(u64::MAX.to_ne_bytes());
		let mut chunk = unsafe { Chunk::from_raw_parts(2, buffer) };
		assert_eq!(chunk.block_palette(), &[0, u64::MAX]);

		let mut buffer = Vec::new();
		buffer.extend(1_234_567u64.to_ne_bytes());
		buffer.extend(694_201_337u64.to_ne_bytes());
		buffer.extend(99_999u64.to_ne_bytes());
		let mut chunk = unsafe { Chunk::from_raw_parts(3, buffer) };
		assert_eq!(chunk.block_palette(), &[1_234_567, 694_201_337, 99_999]);
	}
}
