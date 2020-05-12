use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_common::entity::PrimaryMap;
use wasm_common::{
    Features, LocalFunctionIndex, MemoryIndex, OwnedDataInitializer, SignatureIndex, TableIndex,
};
use wasmer_compiler::{FunctionBody, JumpTableOffsets, Relocation, SectionBody, SectionIndex};
use wasmer_engine::SerializableFunctionFrameInfo;
use wasmer_runtime::Module;
use wasmer_runtime::{MemoryPlan, TablePlan};

/// Serializable struct that represents the compiled metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleMetadata {
    pub features: Features,
    pub module: Arc<Module>,
    pub prefix: String,
    pub data_initializers: Box<[OwnedDataInitializer]>,
    // The function body lengths (used for reverse-locate traps in the function)
    pub function_body_lengths: PrimaryMap<LocalFunctionIndex, u64>,
    // Plans for that module
    pub memory_plans: PrimaryMap<MemoryIndex, MemoryPlan>,
    pub table_plans: PrimaryMap<TableIndex, TablePlan>,
}
