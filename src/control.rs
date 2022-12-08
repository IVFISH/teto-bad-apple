
pub struct Control {

}

impl Control {

    pub fn piece_down(&mut self) -> bool {false}

    pub fn piece_drop(&mut self) {
        while self.piece_down() {}
    }

    pub fn piece_right(&mut self) -> bool {false}

    pub fn piece_left(&mut self) -> bool {false}

    pub fn safe_cw(&mut self) -> bool {false}

    pub fn safe_180(&mut self) -> bool {false}

    pub fn safe_ccw(&mut self) -> bool {false}
}