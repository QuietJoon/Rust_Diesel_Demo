table! {
  core_infos (id) {
      id -> Integer,
      seed -> Integer,
      session_num -> Integer,
      is_running -> Bool,
      body -> Varchar,
  }
}
// General Performance Infos
table! {
  gen_perf_infos (target) {
      target -> Integer,
      target_name -> Text,
      available -> Bool,
      body -> Varchar,
  }
}
