/// The current state of the state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// The state machine is in the neutral state. No sequence of scancode has been
    /// generated yet.
    Neutral,
    /// The E0 escape code has been received.
    E0,
}

/// Contains the state required to convert scan-codes into text.
pub struct Qwerty {
    /// The state of key modifiers.
    modifiers: Modifiers,
    /// The current state of the state machine.
    state: State,
}

impl Qwerty {
    /// Returns a new instance of the [`Qwerty`] struct.
    pub const fn new() -> Self {
        Self {
            modifiers: Modifiers::EMPTY,
            state: State::Neutral,
        }
    }

    /// Returns the current state of the modifiers.
    #[inline(always)]
    pub fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    /// Advances the state of the state machine with a new scan-code. If a character can
    /// be produced, it is returned in a [`Some(_)`] variant.
    ///
    /// If no character could be produced, [`None`] is returned instead.
    pub fn advance(&mut self, scancode: u8) -> Option<char> {
        use State::*;

        let st = self.state;

        // Parse the current escape sequence.
        self.state = match (st, scancode) {
            (Neutral, 0xE0) => E0,
            _ => Neutral,
        };

        match (st, scancode) {
            // Update modifiers.
            (Neutral, 0x2A) => {
                self.modifiers.set_left_shift();
                None
            }
            (Neutral, 0xAA) => {
                self.modifiers.clear_left_shift();
                None
            }
            (Neutral, 0x36) => {
                self.modifiers.set_right_shift();
                None
            }
            (Neutral, 0xB6) => {
                self.modifiers.clear_right_shift();
                None
            }
            (Neutral, 0x1D) => {
                self.modifiers.set_left_control();
                None
            }
            (Neutral, 0x9D) => {
                self.modifiers.clear_left_control();
                None
            }
            (Neutral, 0x3A) => {
                if !self.modifiers.caps_lock_pressed() {
                    self.modifiers.set_caps_lock_pressed();
                    self.modifiers.toggle_caps_lock();
                }
                None
            }
            (Neutral, 0xBA) => {
                self.modifiers.clear_caps_lock_pressed();
                None
            }
            (E0, 0x1D) => {
                self.modifiers.set_right_control();
                None
            }
            (E0, 0x9D) => {
                self.modifiers.clear_right_control();
                None
            }
            (Neutral, 0x38) => {
                self.modifiers.set_left_alt();
                None
            }
            (Neutral, 0xB8) => {
                self.modifiers.clear_left_alt();
                None
            }
            (E0, 0x38) => {
                self.modifiers.set_right_alt();
                None
            }
            (E0, 0xB8) => {
                self.modifiers.clear_right_alt();
                None
            }
            (Neutral, 0x45) => {
                if !self.modifiers.num_lock_pressed() {
                    self.modifiers.set_num_lock_pressed();
                    self.modifiers.toggle_num_lock();
                }
                None
            }
            (Neutral, 0xC5) => {
                self.modifiers.clear_num_lock_pressed();
                None
            }
            // Printable characters.
            (Neutral, 0x02) if !self.modifiers.shifted() => Some('1'),
            (Neutral, 0x02) if self.modifiers.shifted() => Some('!'),
            (Neutral, 0x03) if !self.modifiers.shifted() => Some('2'),
            (Neutral, 0x03) if self.modifiers.shifted() => Some('@'),
            (Neutral, 0x04) if !self.modifiers.shifted() => Some('3'),
            (Neutral, 0x04) if self.modifiers.shifted() => Some('#'),
            (Neutral, 0x05) if !self.modifiers.shifted() => Some('4'),
            (Neutral, 0x05) if self.modifiers.shifted() => Some('$'),
            (Neutral, 0x06) if !self.modifiers.shifted() => Some('5'),
            (Neutral, 0x06) if self.modifiers.shifted() => Some('%'),
            (Neutral, 0x07) if !self.modifiers.shifted() => Some('6'),
            (Neutral, 0x07) if self.modifiers.shifted() => Some('^'),
            (Neutral, 0x08) if !self.modifiers.shifted() => Some('7'),
            (Neutral, 0x08) if self.modifiers.shifted() => Some('&'),
            (Neutral, 0x09) if !self.modifiers.shifted() => Some('8'),
            (Neutral, 0x09) if self.modifiers.shifted() => Some('*'),
            (Neutral, 0x0A) if !self.modifiers.shifted() => Some('9'),
            (Neutral, 0x0A) if self.modifiers.shifted() => Some('('),
            (Neutral, 0x0B) if !self.modifiers.shifted() => Some('0'),
            (Neutral, 0x0B) if self.modifiers.shifted() => Some(')'),
            (Neutral, 0x0C) if !self.modifiers.shifted() => Some('-'),
            (Neutral, 0x0C) if self.modifiers.shifted() => Some('_'),
            (Neutral, 0x0D) if !self.modifiers.shifted() => Some('='),
            (Neutral, 0x0D) if self.modifiers.shifted() => Some('+'),
            (Neutral, 0x10) if !self.modifiers.shifted() => Some('q'),
            (Neutral, 0x10) if self.modifiers.shifted() => Some('Q'),
            (Neutral, 0x11) if !self.modifiers.shifted() => Some('w'),
            (Neutral, 0x11) if self.modifiers.shifted() => Some('W'),
            (Neutral, 0x12) if !self.modifiers.shifted() => Some('e'),
            (Neutral, 0x12) if self.modifiers.shifted() => Some('E'),
            (Neutral, 0x13) if !self.modifiers.shifted() => Some('r'),
            (Neutral, 0x13) if self.modifiers.shifted() => Some('R'),
            (Neutral, 0x14) if !self.modifiers.shifted() => Some('t'),
            (Neutral, 0x14) if self.modifiers.shifted() => Some('T'),
            (Neutral, 0x15) if !self.modifiers.shifted() => Some('y'),
            (Neutral, 0x15) if self.modifiers.shifted() => Some('Y'),
            (Neutral, 0x16) if !self.modifiers.shifted() => Some('u'),
            (Neutral, 0x16) if self.modifiers.shifted() => Some('U'),
            (Neutral, 0x17) if !self.modifiers.shifted() => Some('i'),
            (Neutral, 0x17) if self.modifiers.shifted() => Some('I'),
            (Neutral, 0x18) if !self.modifiers.shifted() => Some('o'),
            (Neutral, 0x18) if self.modifiers.shifted() => Some('O'),
            (Neutral, 0x19) if !self.modifiers.shifted() => Some('p'),
            (Neutral, 0x19) if self.modifiers.shifted() => Some('P'),
            (Neutral, 0x1A) if !self.modifiers.shifted() => Some('['),
            (Neutral, 0x1A) if self.modifiers.shifted() => Some('{'),
            (Neutral, 0x1B) if !self.modifiers.shifted() => Some(']'),
            (Neutral, 0x1B) if self.modifiers.shifted() => Some('}'),
            (Neutral, 0x2B) if !self.modifiers.shifted() => Some('\\'),
            (Neutral, 0x2B) if self.modifiers.shifted() => Some('|'),
            (Neutral, 0x1E) if !self.modifiers.shifted() => Some('a'),
            (Neutral, 0x1E) if self.modifiers.shifted() => Some('A'),
            (Neutral, 0x1F) if !self.modifiers.shifted() => Some('s'),
            (Neutral, 0x1F) if self.modifiers.shifted() => Some('S'),
            (Neutral, 0x20) if !self.modifiers.shifted() => Some('d'),
            (Neutral, 0x20) if self.modifiers.shifted() => Some('D'),
            (Neutral, 0x21) if !self.modifiers.shifted() => Some('f'),
            (Neutral, 0x21) if self.modifiers.shifted() => Some('F'),
            (Neutral, 0x22) if !self.modifiers.shifted() => Some('g'),
            (Neutral, 0x22) if self.modifiers.shifted() => Some('G'),
            (Neutral, 0x23) if !self.modifiers.shifted() => Some('h'),
            (Neutral, 0x23) if self.modifiers.shifted() => Some('H'),
            (Neutral, 0x24) if !self.modifiers.shifted() => Some('j'),
            (Neutral, 0x24) if self.modifiers.shifted() => Some('J'),
            (Neutral, 0x25) if !self.modifiers.shifted() => Some('k'),
            (Neutral, 0x25) if self.modifiers.shifted() => Some('K'),
            (Neutral, 0x26) if !self.modifiers.shifted() => Some('l'),
            (Neutral, 0x26) if self.modifiers.shifted() => Some('L'),
            (Neutral, 0x27) if !self.modifiers.shifted() => Some(';'),
            (Neutral, 0x27) if self.modifiers.shifted() => Some(':'),
            (Neutral, 0x28) if !self.modifiers.shifted() => Some('\''),
            (Neutral, 0x28) if self.modifiers.shifted() => Some('"'),
            (Neutral, 0x29) if !self.modifiers.shifted() => Some('`'),
            (Neutral, 0x29) if self.modifiers.shifted() => Some('~'),
            (Neutral, 0x2C) if !self.modifiers.shifted() => Some('z'),
            (Neutral, 0x2C) if self.modifiers.shifted() => Some('Z'),
            (Neutral, 0x2D) if !self.modifiers.shifted() => Some('x'),
            (Neutral, 0x2D) if self.modifiers.shifted() => Some('X'),
            (Neutral, 0x2E) if !self.modifiers.shifted() => Some('c'),
            (Neutral, 0x2E) if self.modifiers.shifted() => Some('C'),
            (Neutral, 0x2F) if !self.modifiers.shifted() => Some('v'),
            (Neutral, 0x2F) if self.modifiers.shifted() => Some('V'),
            (Neutral, 0x30) if !self.modifiers.shifted() => Some('b'),
            (Neutral, 0x30) if self.modifiers.shifted() => Some('B'),
            (Neutral, 0x31) if !self.modifiers.shifted() => Some('n'),
            (Neutral, 0x31) if self.modifiers.shifted() => Some('N'),
            (Neutral, 0x32) if !self.modifiers.shifted() => Some('m'),
            (Neutral, 0x32) if self.modifiers.shifted() => Some('M'),
            (Neutral, 0x33) if !self.modifiers.shifted() => Some(','),
            (Neutral, 0x33) if self.modifiers.shifted() => Some('<'),
            (Neutral, 0x34) if !self.modifiers.shifted() => Some('.'),
            (Neutral, 0x34) if self.modifiers.shifted() => Some('>'),
            (Neutral, 0x35) if !self.modifiers.shifted() => Some('/'),
            (E0, 0x35) => Some('/'),
            (Neutral, 0x35) if self.modifiers.shifted() => Some('?'),
            (Neutral, 0x47) if self.modifiers.num_lock() => Some('7'),
            (Neutral, 0x48) if self.modifiers.num_lock() => Some('8'),
            (Neutral, 0x49) if self.modifiers.num_lock() => Some('9'),
            (Neutral, 0x4B) if self.modifiers.num_lock() => Some('4'),
            (Neutral, 0x4C) if self.modifiers.num_lock() => Some('5'),
            (Neutral, 0x4D) if self.modifiers.num_lock() => Some('6'),
            (Neutral, 0x4F) if self.modifiers.num_lock() => Some('1'),
            (Neutral, 0x50) if self.modifiers.num_lock() => Some('2'),
            (Neutral, 0x51) if self.modifiers.num_lock() => Some('3'),
            (Neutral, 0x52) if self.modifiers.num_lock() => Some('0'),
            (Neutral, 0x53) if self.modifiers.num_lock() => Some('.'),
            // Non-printable keys
            (Neutral, 0x39) => Some(' '),
            (Neutral | E0, 0x1C) => Some('\n'),
            (Neutral, 0x0E) => Some('\x08'),
            (Neutral, 0x0F) => Some('\t'),
            (Neutral, 0x01) => Some('\x1b'),
            _ => None,
        }
    }
}

/// Keyboard modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers(u16);

impl Modifiers {
    /// The empty set of modifiers.
    pub const EMPTY: Self = Self(0);

    const LEFT_CONTROL_BIT: u32 = 0;
    const RIGHT_CONTROL_BIT: u32 = 1;
    const LEFT_SHIFT_BIT: u32 = 2;
    const RIGHT_SHIFT_BIT: u32 = 3;
    const LEFT_ALT_BIT: u32 = 4;
    const RIGHT_ALT_BIT: u32 = 5;
    const LEFT_SUPER_BIT: u32 = 6;
    const RIGHT_SUPER_BIT: u32 = 7;
    const NUM_LOCK_BIT: u32 = 8;
    const CAPS_LOCK_BIT: u32 = 9;
    const SCROLL_LOCK_BIT: u32 = 10;
    const NUM_LOCK_PRESSED_BIT: u32 = 11;
    const CAPS_LOCK_PRESSED_BIT: u32 = 12;
    const SCROLL_LOCK_PRESSED_BIT: u32 = 13;

    /// Returns whether the specified bit is set.
    #[inline]
    fn is_bit_set(self, bit: u32) -> bool {
        self.0 & (1 << bit) != 0
    }

    /// Toggles the specified bit.
    #[inline]
    fn toggle_bit(&mut self, bit: u32) {
        self.0 ^= 1 << bit;
    }

    /// Sets the specified bit to the provided value.
    #[inline]
    fn set_bit(&mut self, bit: u32) {
        self.0 |= 1 << bit;
    }

    /// Clears the specified bit.
    #[inline]
    fn clear_bit(&mut self, bit: u32) {
        self.0 &= !(1 << bit);
    }

    /// Returns whether the left **CONTROL** key is currently pressed.
    #[inline]
    pub fn left_control(self) -> bool {
        self.is_bit_set(Self::LEFT_CONTROL_BIT)
    }

    /// Returns whether the right **CONTROL** key is currently pressed.
    #[inline]
    pub fn right_control(self) -> bool {
        self.is_bit_set(Self::RIGHT_CONTROL_BIT)
    }

    /// Returns whether the left **SHIFT** key is currently pressed.
    #[inline]
    pub fn left_shift(self) -> bool {
        self.is_bit_set(Self::LEFT_SHIFT_BIT)
    }

    /// Returns whether the right **SHIFT** key is currently pressed.
    #[inline]
    pub fn right_shift(self) -> bool {
        self.is_bit_set(Self::RIGHT_SHIFT_BIT)
    }

    /// Returns whether the left **ALT** key is currently pressed.
    #[inline]
    pub fn left_alt(self) -> bool {
        self.is_bit_set(Self::LEFT_ALT_BIT)
    }

    /// Returns whether the right **ALT** key is currently pressed.
    #[inline]
    pub fn right_alt(self) -> bool {
        self.is_bit_set(Self::RIGHT_ALT_BIT)
    }

    /// Returns whether the left **SUPER** key is currently pressed.
    #[inline]
    pub fn left_super(self) -> bool {
        self.is_bit_set(Self::LEFT_SUPER_BIT)
    }

    /// Returns whether the right **SUPER** key is currently pressed.
    #[inline]
    pub fn right_super(self) -> bool {
        self.is_bit_set(Self::RIGHT_SUPER_BIT)
    }

    /// Returns whether the **CAPS LOCK** key is currently active.
    #[inline]
    pub fn caps_lock(self) -> bool {
        self.is_bit_set(Self::CAPS_LOCK_BIT)
    }

    /// Returns whether the **NUM LOCK** key is currently active.
    #[inline]
    pub fn num_lock(self) -> bool {
        self.is_bit_set(Self::NUM_LOCK_BIT)
    }

    /// Returns whether the **SCROLL LOCK** key is currently active.
    #[inline]
    pub fn scroll_lock(self) -> bool {
        self.is_bit_set(Self::SCROLL_LOCK_BIT)
    }

    /// Returns whether the **CAPS LOCK** key is currently pressed.
    #[inline]
    pub fn caps_lock_pressed(self) -> bool {
        self.is_bit_set(Self::CAPS_LOCK_PRESSED_BIT)
    }

    /// Returns whether the **NUM LOCK** key is currently pressed.
    #[inline]
    pub fn num_lock_pressed(self) -> bool {
        self.is_bit_set(Self::NUM_LOCK_PRESSED_BIT)
    }

    /// Returns whether the **SCROLL LOCK** key is currently pressed.
    #[inline]
    pub fn scroll_lock_pressed(self) -> bool {
        self.is_bit_set(Self::SCROLL_LOCK_PRESSED_BIT)
    }

    /// Returns whether any shift key is used.
    #[inline]
    pub fn shift(self) -> bool {
        self.left_shift() || self.right_shift()
    }

    /// Returns whether any control key is used.
    #[inline]
    pub fn control(self) -> bool {
        self.left_control() || self.right_control()
    }

    /// Returns whether any alt key is used.
    #[inline]
    pub fn alt(self) -> bool {
        self.left_alt() || self.right_alt()
    }

    /// Returns whether any super key is used.
    #[inline]
    pub fn super_key(self) -> bool {
        self.left_super() || self.right_super()
    }

    /// Returns whether the keyboard is shifted.
    #[inline]
    pub fn shifted(self) -> bool {
        self.shift() ^ self.caps_lock()
    }

    /// Toggles the **NUM LOCK** key.
    #[inline]
    pub fn toggle_num_lock(&mut self) {
        self.toggle_bit(Self::NUM_LOCK_BIT);
    }

    /// Toggles the **SCROLL LOCK** key.
    #[inline]
    pub fn toggle_scroll_lock(&mut self) {
        self.toggle_bit(Self::SCROLL_LOCK_BIT);
    }

    /// Toggles the **CAPS LOCK** key.
    #[inline]
    pub fn toggle_caps_lock(&mut self) {
        self.toggle_bit(Self::CAPS_LOCK_BIT);
    }

    /// Set the left shift key.
    pub fn set_left_shift(&mut self) {
        self.set_bit(Self::LEFT_SHIFT_BIT);
    }

    /// Clears the left shift key.
    pub fn clear_left_shift(&mut self) {
        self.clear_bit(Self::LEFT_SHIFT_BIT);
    }

    /// Set the right shift key.
    pub fn set_right_shift(&mut self) {
        self.set_bit(Self::RIGHT_SHIFT_BIT);
    }

    /// Clears the right shift key.
    pub fn clear_right_shift(&mut self) {
        self.clear_bit(Self::RIGHT_SHIFT_BIT);
    }

    /// Set the left control key.
    pub fn set_left_control(&mut self) {
        self.set_bit(Self::LEFT_CONTROL_BIT);
    }

    /// Clears the left control key.
    pub fn clear_left_control(&mut self) {
        self.clear_bit(Self::LEFT_CONTROL_BIT);
    }

    /// Set the right control key.
    pub fn set_right_control(&mut self) {
        self.set_bit(Self::RIGHT_CONTROL_BIT);
    }

    /// Clears the right control key.
    pub fn clear_right_control(&mut self) {
        self.clear_bit(Self::RIGHT_CONTROL_BIT);
    }

    /// Set the left alt key.
    pub fn set_left_alt(&mut self) {
        self.set_bit(Self::LEFT_ALT_BIT);
    }

    /// Clears the left alt key.
    pub fn clear_left_alt(&mut self) {
        self.clear_bit(Self::LEFT_ALT_BIT);
    }

    /// Set the right alt key.
    pub fn set_right_alt(&mut self) {
        self.set_bit(Self::RIGHT_ALT_BIT);
    }

    /// Clears the right alt key.
    pub fn clear_right_alt(&mut self) {
        self.clear_bit(Self::RIGHT_ALT_BIT);
    }

    /// Sets the state of the **CAPS LOCK** key.
    pub fn set_caps_lock_pressed(&mut self) {
        self.set_bit(Self::CAPS_LOCK_BIT);
    }

    /// Clears the state of the **CAPS LOCK** key.
    pub fn clear_caps_lock_pressed(&mut self) {
        self.clear_bit(Self::CAPS_LOCK_BIT);
    }

    /// Sets the state of the **NUM LOCK** key.
    pub fn set_num_lock_pressed(&mut self) {
        self.set_bit(Self::NUM_LOCK_BIT);
    }

    /// Clears the state of the **NUM LOCK** key.
    pub fn clear_num_lock_pressed(&mut self) {
        self.clear_bit(Self::NUM_LOCK_BIT);
    }

    /// Sets the state of the **SCROLL LOCK** key.
    pub fn set_scroll_lock_pressed(&mut self) {
        self.set_bit(Self::SCROLL_LOCK_BIT);
    }

    /// Clears the state of the **SCROLL LOCK** key.
    pub fn clear_scroll_lock_pressed(&mut self) {
        self.clear_bit(Self::SCROLL_LOCK_BIT);
    }
}
