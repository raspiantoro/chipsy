

#[derive(Default)]
pub struct CPU {
    v: [u8; 16],
    i: u16,
    st: u8,
    dt: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16]
}

impl CPU{
    pub fn init(&mut self){
        self.pc = 0x200;
    }

    pub fn v_reg_set(&mut self, index: usize, value:u8){
        self.v[index] = value;
    }

    pub fn v_reg_get(&self, index: usize) -> u8{
        self.v[index]
    }

    pub fn get_pc(&self) -> usize{
        self.pc as usize
    }

    pub fn inc_pc(&mut self){
        self.pc += 2;
    }

    

    pub fn run(&mut self, opcode: u16) {
        // notation -> hxyl
        let h: u8 = (opcode >> 12) as u8;
        let l: u8 = (opcode & 0x000F) as u8;
        let x: u8 = ((opcode >> 8) & 0x000F) as u8;
        let y: u8 = ((opcode >> 4) & 0x000F) as u8;

        // println!("h: {}, x: {}, y: {}, l: {}", h,x,y,l);

        //8xy4
        match (h, x, y, l) {
            (8, _, _, 4) => {
                let temp: u16 = (self.v[x as usize] as u16) + (self.v[y as usize] as u16);

                if temp > 255{
                    self.v[x as usize] = (temp & 0x00FF) as u8;
                    self.v[0xF] = 1;
                } else {
                    self.v[x as usize] = temp as u8;
                    self.v[0xF] = 0;
                }
                // let res = self.v[x as usize].checked_add(self.v[y as usize]);
                // if let Some(val) = res{
                //     self.v[x as usize] = val;
                //     self.v[0xF] = 0;
                // } else {
                //     self.v[0xF] = 1;
                // }
            }
            (_,_,_,_) => panic!("unknown instruction")
        }

    }
}