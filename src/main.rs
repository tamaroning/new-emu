mod cpu;

fn main() {
    println!("Hello NES emulator!");
}

#[cfg(test)]
mod test {
    use super::*;
    use cpu::Cpu;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert!(cpu.stat & 0b0000_0010 == 0b00);
        assert!(cpu.stat & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.stat & 0b0000_0010 == 0b10);
    }

   #[test]
   fn test_0xaa_tax_move_a_to_x() {
       let mut cpu = Cpu::new();
       cpu.a = 10;
       cpu.interpret(vec![0xaa, 0x00]);
 
       assert_eq!(cpu.x, 10)
   }

   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = Cpu::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.x, 1)
    }
}