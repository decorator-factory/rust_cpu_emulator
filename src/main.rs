use cpuem::CPU;


/*
Instruction set:

0x00 NUM <reg> <const>
0x01 MOV <dst> <src>
0x02 ADD <dst> <src>
0x03 SUB <dst> <src>
0x30 SET <adr> <reg>

0xff HLT

*/


fn main() {
    let mut cpu = CPU::new(
        Box::new(|_| None),
        Box::new(|adr, value| {
            if adr.0 == 0xef {
                println!("from 0x{:x}: 0x{:x}", adr, value);
                Some(())
            } else{
                None
            }
        }),
    );
    cpu.fill_ram([
        0x00, 0x01, 0xff,
        0x30, 0xef, 0x01,
        0xff,
    ].iter());
    if let Err(e) = cpu.run_with_debug(
        |_| {},
        |cpu| {
            println!("{}", cpu);
        }
    ) {
        println!("Error: {:?}", e);
    }
}