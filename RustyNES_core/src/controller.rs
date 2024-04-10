#[derive(Default)]
pub struct Controller {
    state: u8,
    index: u8,
    reset: bool,
}

impl Controller {
    pub fn new_controller() -> Controller {
        Controller {
            state: 0,
            index: 0,
            reset: false,
        }
    }

    pub fn read(&mut self) -> u8 {
        // Read the bit at the current index
        let val = (self.state >> self.index) & 1;
        // After reading, increment the index
        self.index += 1;
        if self.index == 8 {
            self.index = 0;
        }
        val
    }

    pub fn write(&mut self, val: u8) {
        self.reset = (val & 1) == 1;
        if self.reset {
            self.index = 0;
        }
    }

    /*
    button 0: A
    button 1: B
    button 2: Select
    button 3: Start
    button 4: Up
    button 5: Down
    button 6: Left
    button 7: Right
     */
    pub fn update_button(&mut self, index: u8, pressed: bool) {
        if pressed {
            self.state |= 1 << index;
        } else {
            self.state &= !(1 << index);
        }
    }
}
