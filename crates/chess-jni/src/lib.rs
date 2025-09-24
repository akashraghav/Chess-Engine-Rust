use chess_core::{Move, PieceType, Square};
use chess_engine::ChessEngine;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jlong, jstring};
use jni::JNIEnv;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc, Mutex, OnceLock,
};

type EngineId = i64;

static ENGINE_COUNTER: AtomicI64 = AtomicI64::new(0);
static ENGINES: OnceLock<Arc<Mutex<HashMap<EngineId, ChessEngine>>>> = OnceLock::new();

fn get_engines() -> &'static Arc<Mutex<HashMap<EngineId, ChessEngine>>> {
    ENGINES.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

fn get_next_id() -> EngineId {
    ENGINE_COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}

fn string_to_jstring<'a>(env: JNIEnv<'a>, s: &str) -> jstring {
    env.new_string(s)
        .expect("Failed to create JString")
        .into_raw()
}

fn jstring_to_string<'a>(mut env: JNIEnv<'a>, jstr: JString<'a>) -> String {
    env.get_string(jstr).expect("Failed to get JString").into()
}

#[no_mangle]
pub extern "system" fn Java_com_chess_engine_ChessEngine_createEngine(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let engine = ChessEngine::new();
    let id = get_next_id();

    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        engines_map.insert(id, engine);
        return id;
    }
    -1
}

#[no_mangle]
pub extern "system" fn Java_com_chess_engine_ChessEngine_destroyEngine(
    _env: JNIEnv,
    _class: JClass,
    engine_id: jlong,
) -> jboolean {
    let engines = get_engines();
    if let Ok(mut engines_map) = engines.lock() {
        if engines_map.remove(&engine_id).is_some() {
            return 1;
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_chess_engine_ChessEngine_getFen(
    env: JNIEnv,
    _class: JClass,
    engine_id: jlong,
) -> jstring {
    let engines = get_engines();
    if let Ok(engines_map) = engines.lock() {
        if let Some(engine) = engines_map.get(&engine_id) {
            let fen = engine.get_fen();
            return string_to_jstring(env, &fen);
        }
    }
    string_to_jstring(env, "")
}

#[no_mangle]
pub extern "system" fn Java_com_chess_engine_ChessEngine_isLegalMove(
    env: JNIEnv,
    _class: JClass,
    engine_id: jlong,
    uci_move: JString,
) -> jboolean {
    let uci_str = jstring_to_string(env, uci_move);
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
