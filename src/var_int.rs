pub fn get_var_int<'a, T, I>(
    iter: &'a mut I,
) -> Result<(T, usize), VarIntError>
    where
        T: VarIntOutput,
        I: Iterator<Item = &'a u8>,
{
    let mut value: T = T::zero();
    let mut bit_offset = 0usize;
    while {
        if bit_offset == T::MAX_LENGTH.max_length() {
            return Err(VarIntError::TooManyBytes {
                length: T::MAX_LENGTH
            });
        }
        let current_byte = if let Some(current_byte) = iter.next() {
            current_byte
        } else {
            return Err(VarIntError::MissingExpectedByte);
        };
        value |= (T::from(current_byte & 0b01111111)).unwrap() << bit_offset;

        bit_offset += 7;

        (current_byte & 0b1000000) != 0
    } {}
    Ok((value, bit_offset / 7))
}

#[derive(Debug)]
pub enum VarIntLength {
    VarInt,
    VarLong,
}

pub trait VarIntOutput: num_traits::Signed + num_traits::PrimInt + num_traits::NumCast + std::ops::BitOrAssign {
    const MAX_LENGTH: VarIntLength;
}

impl VarIntOutput for i32 {
    const MAX_LENGTH: VarIntLength = VarIntLength::VarInt;
}

impl VarIntOutput for i64 {
    const MAX_LENGTH: VarIntLength = VarIntLength::VarLong;
}

impl VarIntLength {
    pub const fn max_length(&self) -> usize {
        use VarIntLength::*;
        match *self {
            VarInt => 35,
            VarLong => 70,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VarIntError {
    #[error("Too many bytes are available to read, the max is: {}", .length.max_length())]
    TooManyBytes { length: VarIntLength },
    #[error("The previous byte implied another byte was available, but .next() returned `None`.")]
    MissingExpectedByte,
}

#[cfg(test)]
mod test {
    use bytes::{BufMut, BytesMut};

    use super::*;

    #[test]
    fn var_int_25565() {
        let sample_var_int = {
            let mut bytes = BytesMut::with_capacity(3);
            bytes.put_u8(0xdd);
            bytes.put_u8(0xc7);
            bytes.put_u8(0x01);
            bytes
        };

        assert_eq!(
            get_var_int::<i32, _>(&mut sample_var_int.iter())
                .unwrap().0,
            25565
        );
    }
}