// declarations.rs
// Benjamin Kim, name, name
// Lab1 types, constants, and static variables

struct Play(Vec<(usize, String, String)>);

const MAX_ARGS: usize = 3;
const MIN_ARGS: usize = 2;

const PROGRAM_NAME: usize = 0;
const CONFIG_FILE: usize = 1;
const OPT: usize = 2;

const CMD_LINE_ERR: usize = 1;
const GEN_SCRIPT_ERR: usize = 2;

static DEBUG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);