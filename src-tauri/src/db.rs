use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::path::Path;
use std::sync::Mutex;

/// 全テーブル共通の規約(requirements.md 6.0 ホーム画面 を参照):
/// - id: `INTEGER PRIMARY KEY AUTOINCREMENT`
/// - date: `TEXT NOT NULL`(`'YYYY-MM-DD'` 形式の ISO8601 日付文字列)
/// - created_at / updated_at: `TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)`
/// - テーブル名は複数形スネークケース(例: schedules, expenses)
///
/// 各機能のテーブルは、対応する機能タスク(6.1 スケジュール管理 等)で
/// このマイグレーション一覧に追加して作成する。
fn migrations() -> Vec<M<'static>> {
    vec![
        // 例: M::up("CREATE TABLE schedules (...)"),
    ]
}

pub struct DbConnection(pub Mutex<Connection>);

pub fn init(db_path: &Path) -> DbConnection {
    let mut conn = Connection::open(db_path).expect("failed to open database");

    let migrations = migrations();
    if !migrations.is_empty() {
        Migrations::new(migrations)
            .to_latest(&mut conn)
            .expect("failed to run database migrations");
    }

    DbConnection(Mutex::new(conn))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_database_file_and_applies_migrations() {
        let dir = std::env::temp_dir().join(format!("life-dashboard-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let db_path = dir.join("test.db");

        let _conn = init(&db_path);

        assert!(db_path.exists());

        std::fs::remove_dir_all(&dir).ok();
    }
}
