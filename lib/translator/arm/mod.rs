use translator::{Arch, BlockTranslationResult, Endian};
use capstone_rust::capstone;
use il::*;
use error::*;

pub struct ARM;

impl ARM {
    pub fn new() -> ARM { ARM }
}

impl Arch for ARM {
    fn endian(&self) -> Endian { Endian::Little }

    fn translate_block(&self, bytes: &[u8], address: u64) -> Result<BlockTranslationResult> {
        let cs = match capstone::Capstone::new(capstone::cs_arch::CS_ARCH_ARM, capstone::cs_mode::CS_MODE_THUMB) {
            Ok(cs) => cs,
            Err(_) => return Err("Capstone Error".into())
        };

        cs.option(capstone::cs_opt_type::CS_OPT_DETAIL, capstone::cs_opt_value::CS_OPT_ON).unwrap();

        // our graph for the block which we will build iteratively with each instruction
        let mut block_graph = ControlFlowGraph::new();

        // the length of this block in bytes
        let mut length: usize = 0;

        let mut successors = Vec::new();

        let mut offset: usize = 0;


        Ok(BlockTranslationResult::new(block_graph, address, length, successors))
    }
}
