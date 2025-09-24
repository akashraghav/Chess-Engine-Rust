use chess_core::{Move, PieceType, Square};
use chess_engine::{ChessEngine, Color, GameResult};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long};
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc, Mutex, OnceLock,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

type EngineId = c_long;
static ENGINE_COUNTER: AtomicI64 = AtomicI64::new(0);
static ENGINES: OnceLock<Arc<Mutex<HashMap<EngineId, ChessEngine>>>> = OnceLock::new();

fn get_engines() -> &'static Arc<Mutex<HashMap<EngineId, ChessEngine>>> {
    ENGINES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

fn get_next_id() -> EngineId {
    ENGINE_COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}

// C FFI Functions

#[no_mangle]
pub extern "C" fn chess_engine_create() -> EngineId {
    let engine = ChessEngine::new();
    let id = get_next_id();

    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        engines_map.insert(id, engine);
    }

    id
}

/// # Safety
/// The caller must ensure that `fen` points to a valid, null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn chess_engine_create_from_fen(fen: *const c_char) -> EngineId {
    if fen.is_null() {
        return -1;
    }

    let fen_str = match CStr::from_ptr(fen).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match ChessEngine::from_fen(fen_str) {
        Ok(engine) => {
            let id = get_next_id();
            let engines = get_engines();
            if let Ok(mut engines_map) = engines.lock() {
                engines_map.insert(id, engine);
            }
            id
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_initialize(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get_mut(&engine_id) {
            return match engine.initialize() {
                Ok(_) => 1,
                Err(_) => 0,
            };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_destroy(engine_id: EngineId) {
    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        engines_map.remove(&engine_id);
    }
}

#[no_mangle]
pub extern "C" fn chess_engine_get_fen(engine_id: EngineId) -> *mut c_char {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            let fen = engine.get_fen();
            return match CString::new(fen) {
                Ok(cstring) => cstring.into_raw(),
                Err(_) => std::ptr::null_mut(),
            };
        }
    }
    std::ptr::null_mut()
}

/// # Safety
/// The caller must ensure that `fen` points to a valid, null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn chess_engine_load_fen(engine_id: EngineId, fen: *const c_char) -> c_int {
    if fen.is_null() {
        return 0;
    }

    let fen_str = match CStr::from_ptr(fen).to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get_mut(&engine_id) {
            return match engine.load_fen(fen_str) {
                Ok(_) => 1,
                Err(_) => 0,
            };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_get_side_to_move(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            return match engine.get_side_to_move() {
                Color::White => 0,
                Color::Black => 1,
            };
        }
    }
    -1
}

/// # Safety
/// The caller must ensure that `uci_move` points to a valid, null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn chess_engine_make_move(
    engine_id: EngineId,
    uci_move: *const c_char,
) -> c_int {
    if uci_move.is_null() {
        return 0;
    }

    let uci_str = match CStr::from_ptr(uci_move).to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get_mut(&engine_id) {
            return match engine.make_move_from_uci(uci_str) {
                Ok(result) => {
                    if result.success {
                        1
                    } else {
                        0
                    }
                }
                Err(_) => 0,
            };
        }
    }
    0
}

/// # Safety
/// The caller must ensure that `uci_move` points to a valid, null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn chess_engine_is_legal_move(
    engine_id: EngineId,
    uci_move: *const c_char,
) -> c_int {
    if uci_move.is_null() {
        return 0;
    }

    let uci_str = match CStr::from_ptr(uci_move).to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            // Parse UCI string (e.g., "e2e4" or "e7e8q")
            if uci_str.len() < 4 {
                return 0;
            }

            let from = Square::from_str(&uci_str[0..2]);
            let to = Square::from_str(&uci_str[2..4]);

            match (from, to) {
                (Ok(from), Ok(to)) => {
                    // Handle promotion
                    let mv = if uci_str.len() == 5 {
                        match uci_str.chars().nth(4) {
                            Some('q') => Move::promotion(from, to, PieceType::Queen),
                            Some('r') => Move::promotion(from, to, PieceType::Rook),
                            Some('b') => Move::promotion(from, to, PieceType::Bishop),
                            Some('n') => Move::promotion(from, to, PieceType::Knight),
                            _ => return 0,
                        }
                    } else {
                        Move::normal(from, to)
                    };

                    return if engine.is_legal_move(mv) { 1 } else { 0 };
                }
                _ => return 0,
            }
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_get_legal_moves_count(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            return engine.get_legal_moves().len() as c_int;
        }
    }
    -1
}

#[no_mangle]
pub extern "C" fn chess_engine_is_in_check(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            let info = engine.get_game_info();
            return if info.is_check { 1 } else { 0 };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_is_checkmate(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            let info = engine.get_game_info();
            return if info.is_checkmate { 1 } else { 0 };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_is_game_over(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            return if engine.is_game_over() { 1 } else { 0 };
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_get_game_result(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            return match engine.get_game_result() {
                GameResult::Ongoing => 0,
                GameResult::WhiteWins => 1,
                GameResult::BlackWins => 2,
                GameResult::Draw => 3,
            };
        }
    }
    -1
}

#[no_mangle]
pub extern "C" fn chess_engine_evaluate(engine_id: EngineId) -> c_int {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            return engine.evaluate();
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn chess_engine_find_best_move(engine_id: EngineId) -> *mut c_char {
    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get_mut(&engine_id) {
            if let Ok(Some(best_move)) = engine.find_best_move() {
                let move_str = best_move.to_string();
                return match CString::new(move_str) {
                    Ok(cstring) => cstring.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                };
            }
        }
    }
    std::ptr::null_mut()
}

/// # Safety
/// The caller must ensure that `s` was allocated by this library and is not used after this call.
#[no_mangle]
pub unsafe extern "C" fn chess_engine_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

// WASM Bindings
#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;

    #[wasm_bindgen]
    pub struct WasmChessEngine {
        engine_id: EngineId,
    }

    #[wasm_bindgen]
    impl WasmChessEngine {
        #[wasm_bindgen(constructor)]
        pub fn new() -> WasmChessEngine {
            let engine_id = chess_engine_create();
            chess_engine_initialize(engine_id);
            WasmChessEngine { engine_id }
        }

        #[wasm_bindgen]
        pub fn from_fen(fen: &str) -> Option<WasmChessEngine> {
            let fen_cstr = CString::new(fen).ok()?;
            let engine_id = chess_engine_create_from_fen(fen_cstr.as_ptr());
            if engine_id != -1 {
                chess_engine_initialize(engine_id);
                Some(WasmChessEngine { engine_id })
            } else {
                None
            }
        }

        #[wasm_bindgen]
        pub fn get_fen(&self) -> String {
            let fen_ptr = chess_engine_get_fen(self.engine_id);
            if !fen_ptr.is_null() {
                let fen = unsafe { CStr::from_ptr(fen_ptr).to_string_lossy().into_owned() };
                chess_engine_free_string(fen_ptr);
                fen
            } else {
                String::new()
            }
        }

        #[wasm_bindgen]
        pub fn load_fen(&mut self, fen: &str) -> bool {
            if let Ok(fen_cstr) = CString::new(fen) {
                chess_engine_load_fen(self.engine_id, fen_cstr.as_ptr()) == 1
            } else {
                false
            }
        }

        #[wasm_bindgen]
        pub fn get_side_to_move(&self) -> i32 {
            chess_engine_get_side_to_move(self.engine_id)
        }

        #[wasm_bindgen]
        pub fn make_move(&mut self, uci_move: &str) -> bool {
            if let Ok(move_cstr) = CString::new(uci_move) {
                chess_engine_make_move(self.engine_id, move_cstr.as_ptr()) == 1
            } else {
                false
            }
        }

        #[wasm_bindgen]
        pub fn is_legal_move(&self, uci_move: &str) -> bool {
            if let Ok(move_cstr) = CString::new(uci_move) {
                chess_engine_is_legal_move(self.engine_id, move_cstr.as_ptr()) == 1
            } else {
                false
            }
        }

        #[wasm_bindgen]
        pub fn get_legal_moves_count(&self) -> i32 {
            chess_engine_get_legal_moves_count(self.engine_id)
        }

        #[wasm_bindgen]
        pub fn is_in_check(&self) -> bool {
            chess_engine_is_in_check(self.engine_id) == 1
        }

        #[wasm_bindgen]
        pub fn is_checkmate(&self) -> bool {
            chess_engine_is_checkmate(self.engine_id) == 1
        }

        #[wasm_bindgen]
        pub fn is_game_over(&self) -> bool {
            chess_engine_is_game_over(self.engine_id) == 1
        }

        #[wasm_bindgen]
        pub fn get_game_result(&self) -> i32 {
            chess_engine_get_game_result(self.engine_id)
        }

        #[wasm_bindgen]
        pub fn evaluate(&self) -> i32 {
            chess_engine_evaluate(self.engine_id)
        }

        #[wasm_bindgen]
        pub fn find_best_move(&self) -> Option<String> {
            let move_ptr = chess_engine_find_best_move(self.engine_id);
            if !move_ptr.is_null() {
                let best_move = unsafe { CStr::from_ptr(move_ptr).to_string_lossy().into_owned() };
                chess_engine_free_string(move_ptr);
                Some(best_move)
            } else {
                None
            }
        }
    }

    impl Drop for WasmChessEngine {
        fn drop(&mut self) {
            chess_engine_destroy(self.engine_id);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

// Python bindings using PyO3
#[cfg(feature = "python")]
mod python {
    use super::*;
    use pyo3::prelude::*;

    #[pyclass]
    pub struct PyChessEngine {
        engine_id: EngineId,
    }

    #[pymethods]
    impl PyChessEngine {
        #[new]
        fn new() -> Self {
            let engine_id = chess_engine_create();
            chess_engine_initialize(engine_id);
            PyChessEngine { engine_id }
        }

        #[staticmethod]
        fn from_fen(fen: &str) -> PyResult<PyChessEngine> {
            let fen_cstr = CString::new(fen)?;
            let engine_id = chess_engine_create_from_fen(fen_cstr.as_ptr());
            if engine_id != -1 {
                chess_engine_initialize(engine_id);
                Ok(PyChessEngine { engine_id })
            } else {
                Err(pyo3::exceptions::PyValueError::new_err("Invalid FEN"))
            }
        }

        fn get_fen(&self) -> String {
            let fen_ptr = chess_engine_get_fen(self.engine_id);
            if !fen_ptr.is_null() {
                let fen = unsafe { CStr::from_ptr(fen_ptr).to_string_lossy().into_owned() };
                chess_engine_free_string(fen_ptr);
                fen
            } else {
                String::new()
            }
        }

        fn load_fen(&mut self, fen: &str) -> PyResult<bool> {
            let fen_cstr = CString::new(fen)?;
            Ok(chess_engine_load_fen(self.engine_id, fen_cstr.as_ptr()) == 1)
        }

        fn get_side_to_move(&self) -> i32 {
            chess_engine_get_side_to_move(self.engine_id)
        }

        fn make_move(&mut self, uci_move: &str) -> PyResult<bool> {
            let move_cstr = CString::new(uci_move)?;
            Ok(chess_engine_make_move(self.engine_id, move_cstr.as_ptr()) == 1)
        }

        fn is_legal_move(&self, uci_move: &str) -> PyResult<bool> {
            let move_cstr = CString::new(uci_move)?;
            Ok(chess_engine_is_legal_move(self.engine_id, move_cstr.as_ptr()) == 1)
        }

        fn get_legal_moves_count(&self) -> i32 {
            chess_engine_get_legal_moves_count(self.engine_id)
        }

        fn is_in_check(&self) -> bool {
            chess_engine_is_in_check(self.engine_id) == 1
        }

        fn is_checkmate(&self) -> bool {
            chess_engine_is_checkmate(self.engine_id) == 1
        }

        fn is_game_over(&self) -> bool {
            chess_engine_is_game_over(self.engine_id) == 1
        }

        fn get_game_result(&self) -> i32 {
            chess_engine_get_game_result(self.engine_id)
        }

        fn evaluate(&self) -> i32 {
            chess_engine_evaluate(self.engine_id)
        }

        fn find_best_move(&self) -> Option<String> {
            let move_ptr = chess_engine_find_best_move(self.engine_id);
            if !move_ptr.is_null() {
                let best_move = unsafe { CStr::from_ptr(move_ptr).to_string_lossy().into_owned() };
                chess_engine_free_string(move_ptr);
                Some(best_move)
            } else {
                None
            }
        }
    }

    impl Drop for PyChessEngine {
        fn drop(&mut self) {
            chess_engine_destroy(self.engine_id);
        }
    }

    #[pymodule]
    fn chess_engine(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PyChessEngine>()?;
        Ok(())
    }
}

#[cfg(feature = "python")]
pub use python::*;
