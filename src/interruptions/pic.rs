//I looked at pic8259_simple crate source code to know how to do this
extern crate cpuio;
use cpuio::UnsafePort; 

//Const values to send over to the pics
const INIT: u8 = 0x11;
const END_INTERRUPT: u8 = 0x20;
const MODE: u8 = 0x01; //Select mode 8086


struct Pic {
    //Need to map this so it doesn't collide with exceptions
    offset: usize,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}

impl Pic {
    fn handles(&self, interrupt: u8) -> bool {
        crate::util::math::is_in_range(self.offset, self.offset + 8, interrupt as usize)
    }

    fn end_interrupt(&mut self) {
        unsafe {
            self.command.write(END_INTERRUPT);
        }
    }
}

pub struct Pics {
    primary: Pic,
    secondary: Pic,
}

impl Pics {
    //Make this unsafe for users to keep an eye out and not overwrite existing
    //interrupts
    pub unsafe fn get_init(primary_offset: usize, secondary_offset: usize) -> Pics {
        let mut created = Self::new(primary_offset, secondary_offset);
        created.init();
        created
    }

    pub const unsafe fn new(primary_offset: usize, secondary_offset: usize) -> Pics {
        Pics {
            primary: Pic {
                offset: primary_offset,
                command: UnsafePort::new(0x20),
                data: UnsafePort::new(0x21),
            }, 
            secondary: Pic {
                offset: secondary_offset,
                command: UnsafePort::new(0xa0),
                data: UnsafePort::new(0xa1),
            }
        }
    }

    pub unsafe fn init(&mut self) {
        //This is used as a delay on pic8259_simple, so I use it here also,
        //port 0x80 is a garbage port, but takes long enough to write there
        //to let us wait properly for the other ports to write
        let mut wait_port = UnsafePort::<u8>::new(0x80);
        
        let primary_mask = self.primary.data.read();
        wait_port.write(0);
        let secondary_mask = self.secondary.data.read();
        wait_port.write(0);

        //Init both pics
        self.primary.command.write(INIT);
        wait_port.write(0);
        self.secondary.command.write(INIT);
        wait_port.write(0);

        //Set base offsets
        self.primary.data.write(self.primary.offset as u8);
        wait_port.write(0);
        self.secondary.data.write(self.secondary.offset as u8);
        wait_port.write(0);

        //Configure chaining, from secondary to primary
        self.primary.data.write(4);
        wait_port.write(0);
        self.secondary.data.write(4);
        wait_port.write(0);

        //Set mode
        self.primary.data.write(MODE);
        wait_port.write(0);
        self.secondary.data.write(MODE);
        wait_port.write(0);

        //Reset mask
        self.primary.data.write(primary_mask);
        self.secondary.data.write(secondary_mask);
    }

    //Im not going to bother create an enum, -1 is neither,
    //0 is primary and 1 secondary
    fn who_handles(&self, interrupt: u8) -> i8 {
        if self.primary.handles(interrupt) { 0 }
        else if self.secondary.handles(interrupt) { 1 }
        else { -1 }
    }

    pub fn handles(&self, interrupt: u8) -> bool {
        self.who_handles(interrupt) != -1
    }

    pub unsafe fn end_interrupt(&mut self, interrupt: u8) {
        let who = self.who_handles(interrupt);
        if who == 0 { self.primary.end_interrupt(); }
        else if who == 1 { self.secondary.end_interrupt(); }
    }
}
