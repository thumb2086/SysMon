use rusqlite::{Connection, params};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

#[derive(Debug, Clone)]
pub struct TrafficRecord {
    pub id: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub interface_name: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct DailyTraffic {
    pub date: NaiveDate,
    pub total_sent: u64,
    pub total_received: u64,
    pub total_bytes: u64,
}

impl Database {
    pub fn new() -> Self {
        let path = PathBuf::from("data");
        std::fs::create_dir_all(&path).ok();
        
        let conn = Connection::open(path.join("sysmon.db")).unwrap();
        let db = Database { conn };
        db.init_tables();
        db
    }

    fn init_tables(&self) {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS traffic (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME NOT NULL,
                interface_name TEXT,
                bytes_sent INTEGER,
                bytes_received INTEGER,
                total_bytes INTEGER
            );
            
            CREATE INDEX IF NOT EXISTS idx_traffic_timestamp 
            ON traffic(timestamp);
            
            CREATE TABLE IF NOT EXISTS daily_stats (
                date DATE PRIMARY KEY,
                total_sent INTEGER,
                total_received INTEGER,
                total_bytes INTEGER
            );
        ").unwrap();
    }

    pub fn insert_traffic(&self, record: &TrafficRecord) {
        self.conn.execute(
            "INSERT INTO traffic (timestamp, interface_name, bytes_sent, bytes_received, total_bytes) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                record.timestamp.to_string(),
                record.interface_name,
                record.bytes_sent,
                record.bytes_received,
                record.total_bytes
            ],
        ).unwrap();
    }

    pub fn get_daily_traffic(&self, date: NaiveDate) -> DailyTraffic {
        let mut stmt = self.conn.prepare(
            "SELECT COALESCE(SUM(bytes_sent), 0), COALESCE(SUM(bytes_received), 0), COALESCE(SUM(total_bytes), 0)
             FROM traffic WHERE DATE(timestamp) = ?1"
        ).unwrap();
        
        let date_str = date.format("%Y-%m-%d").to_string();
        let result = stmt.query_row(params![date_str], |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, u64>(1)?,
                row.get::<_, u64>(2)?
            ))
        }).unwrap_or((0, 0, 0));
        
        DailyTraffic {
            date,
            total_sent: result.0,
            total_received: result.1,
            total_bytes: result.2,
        }
    }

    pub fn get_monthly_traffic(&self, year: i32, month: u32) -> DailyTraffic {
        let mut stmt = self.conn.prepare(
            "SELECT COALESCE(SUM(bytes_sent), 0), COALESCE(SUM(bytes_received), 0), COALESCE(SUM(total_bytes), 0)
             FROM traffic WHERE strftime('%Y', timestamp) = ?1 AND strftime('%m', timestamp) = ?2"
        ).unwrap();
        
        let month_str = format!("{:02}", month);
        let result = stmt.query_row(params![year.to_string(), month_str], |row| {
            Ok((
                row.get::<_, u64>(0)?,
                row.get::<_, u64>(1)?,
                row.get::<_, u64>(2)?
            ))
        }).unwrap_or((0, 0, 0));
        
        let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        DailyTraffic {
            date,
            total_sent: result.0,
            total_received: result.1,
            total_bytes: result.2,
        }
    }

    pub fn get_traffic_history(&self, days: u32) -> Vec<DailyTraffic> {
        let today = Utc::now().date_naive();
        let mut history = Vec::new();
        
        for i in 0..days {
            let date = today - chrono::Duration::days(i as i64);
            history.push(self.get_daily_traffic(date));
        }
        
        history.reverse();
        history
    }

    pub fn cleanup_old_data(&self, retention_days: u32) {
        let cutoff = Utc::now().date_naive() - chrono::Duration::days(retention_days as i64);
        let cutoff_str = cutoff.format("%Y-%m-%d").to_string();
        self.conn.execute(
            "DELETE FROM traffic WHERE DATE(timestamp) < ?1",
            params![cutoff_str],
        ).unwrap();
    }
}
