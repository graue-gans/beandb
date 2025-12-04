use crate::models::beans::{Beans, BrewingMethod};
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    pub fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS beans (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                flavor_profile TEXT NOT NULL,
                country TEXT NOT NULL,
                region TEXT NOT NULL,
                variety TEXT NOT NULL,
                process TEXT NOT NULL,
                roaster TEXT NOT NULL,
                roast_level TEXT NOT NULL,
                purchase_date TEXT NOT NULL,
                price REAL NOT NULL,
                weight INTEGER NOT NULL,
                bought_at TEXT NOT NULL,
                rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 10),
                notes TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS brewing_methods (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                bean_id INTEGER NOT NULL,
                method TEXT NOT NULL,
                grind_size TEXT NOT NULL,
                brewing_ratio TEXT NOT NULL,
                rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 10),
                notes TEXT NOT NULL,
                FOREIGN KEY (bean_id) REFERENCES beans(id) ON DELETE CASCADE
            );
            "#,
        )?;
        Ok(())
    }

    pub fn insert_bean(&self, bean: &Beans, brewing_method: &BrewingMethod) -> Result<i32> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            r#"
            INSERT INTO beans (
                name, flavor_profile, country, region, variety, process,
                roaster, roast_level, purchase_date, price, weight, bought_at,
                rating, notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            "#,
            params![
                bean.name,
                bean.flavor_profile,
                bean.country,
                bean.region,
                bean.variety,
                bean.process,
                bean.roaster,
                bean.roast_level,
                bean.purchase_date,
                bean.price,
                bean.weight,
                bean.bought_at,
                bean.rating,
                bean.notes,
            ],
        )?;

        let bean_id = conn.last_insert_rowid() as i32;

        conn.execute(
            r#"
            INSERT INTO brewing_methods (
                bean_id, method, grind_size, brewing_ratio, rating, notes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                bean_id,
                brewing_method.method,
                brewing_method.grind_size,
                brewing_method.brewing_ratio,
                brewing_method.rating,
                brewing_method.notes,
            ],
        )?;

        Ok(bean_id)
    }

    pub fn get_all_beans(&self) -> Result<Vec<Beans>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, name, flavor_profile, country, region, variety, process,
                    roaster, roast_level, purchase_date, price, weight, bought_at,
                    rating, notes FROM beans ORDER BY id DESC",
        )?;

        let beans = stmt.query_map([], |row| {
            Ok(Beans {
                id: row.get(0)?,
                name: row.get(1)?,
                flavor_profile: row.get(2)?,
                country: row.get(3)?,
                region: row.get(4)?,
                variety: row.get(5)?,
                process: row.get(6)?,
                roaster: row.get(7)?,
                roast_level: row.get(8)?,
                purchase_date: row.get(9)?,
                price: row.get(10)?,
                weight: row.get(11)?,
                bought_at: row.get(12)?,
                rating: row.get(13)?,
                notes: row.get(14)?,
            })
        })?;

        beans.collect()
    }

    pub fn get_bean_by_id(&self, id: i32) -> Result<Option<Beans>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, name, flavor_profile, country, region, variety, process,
                    roaster, roast_level, purchase_date, price, weight, bought_at,
                    rating, notes FROM beans WHERE id = ?",
        )?;

        let mut rows = stmt.query_map([id], |row| {
            Ok(Beans {
                id: row.get(0)?,
                name: row.get(1)?,
                flavor_profile: row.get(2)?,
                country: row.get(3)?,
                region: row.get(4)?,
                variety: row.get(5)?,
                process: row.get(6)?,
                roaster: row.get(7)?,
                roast_level: row.get(8)?,
                purchase_date: row.get(9)?,
                price: row.get(10)?,
                weight: row.get(11)?,
                bought_at: row.get(12)?,
                rating: row.get(13)?,
                notes: row.get(14)?,
            })
        })?;

        rows.next().transpose()
    }
}
