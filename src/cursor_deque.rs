// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2026 by Jinui agentic AI, supervised by
// Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use core::cmp;
use core::fmt::{self, Debug, Formatter};
use std::collections::VecDeque;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

use crate::traits::{AsDequeMut, AsDequeRef};

/// A `CursorDeque` wraps an in-memory buffer and provides it with a
/// [`Seek`] implementation.
///
/// `CursorDeque`s are used with in-memory buffers, anything that implements
/// [`AsDequeRef<u8>`], to allow them to implement [`Read`] and/or [`Write`],
/// allowing these buffers to be used anywhere you might use a reader or writer
/// that would otherwise need a standard file or network socket.
///
/// This is similar to [`io::Cursor`], but works with [`VecDeque`].
#[derive(Clone, Default, PartialEq, Eq)]
pub struct CursorDeque<T> {
    inner: T,
    pos: u64,
}

impl<T> CursorDeque<T> {
    /// Creates a new cursor wrapping the provided underlying in-memory buffer.
    ///
    /// Cursor initial position is 0.
    pub fn new(inner: T) -> CursorDeque<T> {
        CursorDeque { inner, pos: 0 }
    }

    /// Consumes this cursor, returning the underlying value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying value in this cursor.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying value in this cursor.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying value as it may corrupt this cursor's position.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Returns the current position of this cursor.
    pub fn position(&self) -> u64 {
        self.pos
    }

    /// Sets the position of this cursor.
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

fn cursor_seek(pos: &mut u64, len: u64, style: SeekFrom) -> io::Result<u64> {
    let (base_pos, offset) = match style {
        SeekFrom::Start(n) => {
            *pos = n;
            return Ok(n);
        }
        SeekFrom::End(n) => (len, n),
        SeekFrom::Current(n) => (*pos, n),
    };
    let new_pos = if offset >= 0 {
        base_pos.checked_add(offset as u64)
    } else {
        base_pos.checked_sub((offset.wrapping_neg()) as u64)
    };
    match new_pos {
        Some(n) => {
            *pos = n;
            Ok(*pos)
        }
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid seek to a negative or overflowing position",
        )),
    }
}

fn cursor_read(pos: &mut u64, inner: &VecDeque<u8>, buf: &mut [u8]) -> io::Result<usize> {
    let len = inner.len() as u64;
    if *pos >= len {
        return Ok(0);
    }
    let pos_idx = *pos as usize;
    let n = cmp::min(buf.len(), (len - *pos) as usize);
    let (s1, s2) = inner.as_slices();

    let mut written = 0;
    if pos_idx < s1.len() {
        let to_read = cmp::min(n, s1.len() - pos_idx);
        buf[..to_read].copy_from_slice(&s1[pos_idx..pos_idx + to_read]);
        written += to_read;
    }

    if written < n {
        let to_read = n - written;
        let s2_pos = if pos_idx < s1.len() {
            0
        } else {
            pos_idx - s1.len()
        };
        buf[written..written + to_read].copy_from_slice(&s2[s2_pos..s2_pos + to_read]);
        written += to_read;
    }

    *pos += written as u64;
    Ok(written)
}

fn cursor_fill_buf(pos: u64, inner: &VecDeque<u8>) -> io::Result<&[u8]> {
    let len = inner.len() as u64;
    if pos >= len {
        return Ok(&[]);
    }
    let pos_idx = pos as usize;
    let (s1, s2) = inner.as_slices();

    if pos_idx < s1.len() {
        Ok(&s1[pos_idx..])
    } else {
        Ok(&s2[pos_idx - s1.len()..])
    }
}

impl<T: Debug> Debug for CursorDeque<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CursorDeque")
            .field("inner", &self.inner)
            .field("pos", &self.pos)
            .finish()
    }
}

impl<T: AsDequeRef<u8>> Seek for CursorDeque<T> {
    fn seek(&mut self, style: SeekFrom) -> io::Result<u64> {
        cursor_seek(&mut self.pos, self.inner.as_deque_ref().len() as u64, style)
    }
}

impl<T: AsDequeRef<u8>> Read for CursorDeque<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        cursor_read(&mut self.pos, self.inner.as_deque_ref(), buf)
    }
}

impl<T: AsDequeRef<u8>> BufRead for CursorDeque<T> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        cursor_fill_buf(self.pos, self.inner.as_deque_ref())
    }

    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}

fn vecdeque_write(pos: &mut u64, inner: &mut VecDeque<u8>, buf: &[u8]) -> io::Result<usize> {
    let len = inner.len() as u64;
    let end = pos
        .checked_add(buf.len() as u64)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "write position overflow"))?;

    if *pos > len {
        inner.resize((*pos) as usize, 0);
    }

    let pos_idx = (*pos) as usize;
    for (i, byte) in buf.iter().enumerate() {
        if pos_idx + i < inner.len() {
            inner[pos_idx + i] = *byte;
        } else {
            inner.push_back(*byte);
        }
    }

    *pos = end;
    Ok(buf.len())
}

impl<T: AsDequeMut<u8>> Write for CursorDeque<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        vecdeque_write(&mut self.pos, self.inner.as_deque_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn test_cursor_deque_smart_pointers() {
        use std::borrow::Cow;
        use std::cell::RefCell;
        use std::sync::{Mutex, RwLock};

        let vd = VecDeque::from(vec![1, 2, 3]);
        let rc = Rc::new(vd.clone());
        let arc = Arc::new(vd.clone());
        let refcell = RefCell::new(vd.clone());
        let mutex = Mutex::new(vd.clone());
        let rwlock = RwLock::new(vd.clone());
        let cow = Cow::Borrowed(&vd);

        let mut rc_cursor = CursorDeque::new(rc);
        let mut buf = [0u8; 3];
        rc_cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3]);

        let mut arc_cursor = CursorDeque::new(arc);
        arc_cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3]);

        let mut cow_cursor = CursorDeque::new(cow);
        cow_cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3]);

        {
            let mut ref_cursor = CursorDeque::new(refcell.borrow());
            ref_cursor.read_exact(&mut buf).unwrap();
            assert_eq!(buf, [1, 2, 3]);
        }

        {
            let mut ref_mut_cursor = CursorDeque::new(refcell.borrow_mut());
            ref_mut_cursor.write_all(&[4, 5, 6]).unwrap();
        }
        assert_eq!(Vec::from(refcell.borrow().clone()), vec![4, 5, 6]);

        {
            let mut mutex_cursor = CursorDeque::new(mutex.lock().unwrap());
            mutex_cursor.write_all(&[4, 5, 6]).unwrap();
            mutex_cursor.set_position(0);
            mutex_cursor.read_exact(&mut buf).unwrap();
            assert_eq!(buf, [4, 5, 6]);
        }

        {
            let mut rw_read_cursor = CursorDeque::new(rwlock.read().unwrap());
            rw_read_cursor.read_exact(&mut buf).unwrap();
            assert_eq!(buf, [1, 2, 3]);
        }

        {
            let mut rw_write_cursor = CursorDeque::new(rwlock.write().unwrap());
            rw_write_cursor.write_all(&[7, 8, 9]).unwrap();
        }
        assert_eq!(Vec::from(rwlock.read().unwrap().clone()), vec![7, 8, 9]);
    }

    #[test]
    fn test_cursor_deque_comprehensive() {
        use std::cell::RefCell;
        use std::sync::Mutex;

        // Test Rc/Arc with writing
        let rc = Rc::new(VecDeque::from(vec![1u8, 2, 3]));
        let mut cursor = CursorDeque::new(rc.clone());
        cursor.write_all(&[4, 5, 6]).unwrap();
        // Since we had another clone (rc), make_mut should have cloned the data
        assert_eq!(Vec::from((*rc).clone()), vec![1, 2, 3]);
        assert_eq!(Vec::from((*cursor.into_inner()).clone()), vec![4, 5, 6]);

        let arc = Arc::new(VecDeque::from(vec![1u8, 2, 3]));
        let mut cursor = CursorDeque::new(arc);
        cursor.write_all(&[7, 8, 9]).unwrap();
        assert_eq!(Vec::from((*cursor.into_inner()).clone()), vec![7, 8, 9]);

        // Test nested references
        let vd = VecDeque::from(vec![1u8, 2, 3]);
        let rc = Rc::new(vd);
        {
            let mut cursor = CursorDeque::new(&rc);
            let mut buf = [0u8; 3];
            cursor.read_exact(&mut buf).unwrap();
            assert_eq!(buf, [1, 2, 3]);
        }

        // Test Arc<Mutex<VecDeque>>
        let mutex = Arc::new(Mutex::new(VecDeque::from(vec![1u8, 2, 3])));
        {
            let mut cursor = CursorDeque::new(mutex.lock().unwrap());
            cursor.write_all(&[10, 11, 12]).unwrap();
        }
        assert_eq!(Vec::from(mutex.lock().unwrap().clone()), vec![10, 11, 12]);

        // Test RefCell
        let refcell = RefCell::new(VecDeque::from(vec![1u8, 2, 3]));
        {
            let mut cursor = CursorDeque::new(refcell.borrow_mut());
            cursor.write_all(&[13, 14, 15]).unwrap();
        }
        assert_eq!(Vec::from(refcell.borrow().clone()), vec![13, 14, 15]);
    }

    #[test]
    fn test_cursor_deque_read() {
        let mut vd = VecDeque::<u8>::new();
        vd.push_back(1);
        vd.push_back(2);
        vd.push_back(3);
        let mut cursor = CursorDeque::new(vd);

        let mut buf = [0u8; 2];
        assert_eq!(cursor.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [1, 2]);
        assert_eq!(cursor.position(), 2);

        assert_eq!(cursor.read(&mut buf).unwrap(), 1);
        assert_eq!(buf[0], 3);
        assert_eq!(cursor.position(), 3);

        assert_eq!(cursor.read(&mut buf).unwrap(), 0);
    }

    #[test]
    fn test_cursor_deque_write() {
        let mut vd = VecDeque::<u8>::new();
        let mut cursor = CursorDeque::new(&mut vd);

        assert_eq!(cursor.write(&[1, 2]).unwrap(), 2);
        assert_eq!(cursor.position(), 2);
        assert_eq!(
            *cursor.get_ref(),
            &vec![1, 2].into_iter().collect::<VecDeque<u8>>()
        );

        cursor.set_position(1);
        assert_eq!(cursor.write(&[3, 4]).unwrap(), 2);
        assert_eq!(cursor.position(), 3);
        assert_eq!(
            *cursor.get_ref(),
            &vec![1, 3, 4].into_iter().collect::<VecDeque<u8>>()
        );

        cursor.set_position(5);
        assert_eq!(cursor.write(&[5]).unwrap(), 1);
        assert_eq!(
            *cursor.get_ref(),
            &vec![1, 3, 4, 0, 0, 5].into_iter().collect::<VecDeque<u8>>()
        );
    }

    #[test]
    fn test_cursor_deque_seek() {
        let mut vd = VecDeque::<u8>::new();
        vd.extend(vec![1, 2, 3, 4, 5]);
        let mut cursor = CursorDeque::new(vd);

        assert_eq!(cursor.seek(SeekFrom::Start(2)).unwrap(), 2);
        let mut buf = [0u8; 1];
        cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 3);

        assert_eq!(cursor.seek(SeekFrom::Current(1)).unwrap(), 4);
        cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 5);

        assert_eq!(cursor.seek(SeekFrom::End(-2)).unwrap(), 3);
        cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf[0], 4);
    }

    #[test]
    fn test_cursor_deque_fragmented() {
        let mut vd = VecDeque::with_capacity(8);
        // Fill and pop to move start index
        for i in 0..6 {
            vd.push_back(i);
        }
        for _ in 0..4 {
            vd.pop_front();
        }
        // Now vd has [4, 5] at the end of the internal buffer.
        // Pushing more will cause wrap-around.
        for i in 6..12 {
            vd.push_back(i);
        }
        // vd content: [4, 5, 6, 7, 8, 9, 10, 11]
        // Internal structure should be two slices.
        let (s1_len, s2_len) = {
            let (s1, s2) = vd.as_slices();
            assert!(!s1.is_empty());
            assert!(!s2.is_empty());
            (s1.len(), s2.len())
        };

        let mut cursor = CursorDeque::new(vd);

        // Test reading across fragmentation boundary
        let mut buf = [0u8; 8];
        assert_eq!(cursor.read(&mut buf).unwrap(), 8);
        assert_eq!(buf, [4, 5, 6, 7, 8, 9, 10, 11]);

        // Test BufRead (fill_buf)
        cursor.set_position(0);
        let filled = cursor.fill_buf().unwrap();
        // fill_buf should return only the first slice if we are in it
        assert_eq!(filled.len(), s1_len);
        assert_eq!(filled, &[4, 5, 6, 7, 8, 9, 10, 11][..s1_len]);
        cursor.consume(s1_len);
        assert_eq!(cursor.position(), s1_len as u64);

        let filled2 = cursor.fill_buf().unwrap();
        assert_eq!(filled2.len(), s2_len);
        assert_eq!(filled2, &[4, 5, 6, 7, 8, 9, 10, 11][s1_len..]);
        cursor.consume(s2_len);
        assert_eq!(cursor.position(), 8);

        // Test writing across fragmentation boundary
        let mut vd = VecDeque::with_capacity(8);
        for i in 0..6 {
            vd.push_back(i);
        }
        for _ in 0..4 {
            vd.pop_front();
        }
        // vd: [4, 5]
        let mut cursor = CursorDeque::new(vd);
        // Write 4 bytes, should wrap around
        cursor.write_all(&[20, 21, 22, 23]).unwrap();
        // Result should be [20, 21, 22, 23] replacing [4, 5] and extending
        assert_eq!(cursor.position(), 4);
        let inner = cursor.into_inner();
        assert_eq!(
            inner,
            vec![20, 21, 22, 23].into_iter().collect::<VecDeque<u8>>()
        );

        // Verify it's still fragmented if it didn't reallocate
        let (_s1, _s2) = inner.as_slices();
        // Depending on VecDeque implementation, it might have reallocated or not.
        // But our code should handle it either way.
        // Let's check reading again with this inner
        let mut cursor = CursorDeque::new(inner);
        let mut buf = [0u8; 4];
        cursor.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [20, 21, 22, 23]);
    }
}
