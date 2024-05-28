/// Converts a slice of u8 bytes values to a a slice of u16 bytes,
/// ordering it to be either little or big endian, depending on the boolean value passed
pub fn order_bytes(byte_stack: Vec<u8>, little_endian: bool) -> Result<Vec<u16>, String> {
    if byte_stack.is_empty() {
        return Ok(vec![]);
    }
    let mut byte_stack = byte_stack.into_iter();
    let mut last_byte: Option<u16> = None;

    if byte_stack.len() % 2 != 0 {
        last_byte = Some(byte_stack.next_back().unwrap() as u16)
    }

    Ok(std::iter::from_fn(|| {
        if let [Some(a), Some(b)] = [byte_stack.next(), byte_stack.next()] {
            return Some([a, b]);
        }
        None
    })
    .map(|[left, right]| {
        let [left, right] = [left as u16, right as u16];
        if little_endian {
            (right << 8) | left
        } else {
            (left << 8) | right
        }
    })
    .chain(last_byte)
    .collect::<Vec<_>>())
}

#[cfg(test)]
mod test {
    use super::order_bytes;

    #[test]
    fn double_byte() {
        let bytes = vec![0x64, 0x65];
        assert_eq!(vec![0x6465], order_bytes(bytes, false).unwrap())
    }

    #[test]
    fn big_endian_hex_alphabet_representation() {
        let alphabet = (0..=25).map(|value| value + 'a' as u8).collect::<Vec<_>>();
        let expected = vec![
            0x6261, 0x6463, 0x6665, 0x6867, 0x6a69, 0x6c6b, 0x6e6d, 0x706f, 0x7271, 0x7473, 0x7675,
            0x7877, 0x7a79,
        ];
        assert_eq!(expected, order_bytes(alphabet, true).unwrap());
    }

    #[test]
    fn odd_byte_ordering() {
        let bytes: Vec<u8> = vec![0x45, 0x60, 0x9D];
        let expected: Vec<u16> = vec![0x4560, 0x9D];
        assert_eq!(expected, order_bytes(bytes, false).unwrap());
    }
}
