pub const SEC_ARGON2ID_ENV_MEMORY_SIZE_MB: &str = "ARGON2ID_MEMORY_SIZE_MB";
pub const SEC_ARGON2ID_ENV_NUM_ITERATIONS: &str = "ARGON2ID_NUM_ITERATIONS";
pub const SEC_ARGON2ID_ENV_NUM_THREADS: &str = "ARGON2ID_NUM_THREADS";
pub const SEC_ARGON2ID_ENV_OUTPUT_LEN: &str = "ARGON2ID_OUTPUT_LEN";

pub const SEC_ARGON2ID_MEMORY_SIZE_MB_DEFAULT: u32 = 65536;
pub const SEC_ARGON2ID_NUM_ITERATIONS_DEFAULT: u32 = 3;
pub const SEC_ARGON2ID_NUM_THREADS_DEFAULT: u32 = 4;
pub const SEC_ARGON2ID_OUTPUT_LEN_DEFAULT: usize = 32;

pub const SEC_ERR_HASH_PARSE_FAIL: &str = "password_hashing_error";
pub const SEC_ERR_PASS_VERIFY: &str = "password_verification_error";
pub const SEC_ERR_HASH_FAILED: &str = "password_hashing_failed";
pub const SEC_ERR_PASS_NOT_MATCH: &str = "passwords_do_not_match";
pub const SEC_ERR_AUTHENTICATION: &str = "authentication_error";

pub const ENV_ERR_PARSE_FAIL: &str = "env_parse_error";

pub const ERR_CONTEXT_ARGON2ID_SERV: &str = "argon2id_hash_service";
pub const ERR_CONTEXT_LOGIN: &str = "login";
pub const ERR_CONTEXT_ENV: &str = "environment";

