use anyhow::Result;
use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}, Pool, Sqlite};
use std::path::Path;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(db_path: &Path) -> Result<Self> {
        let db_url = format!("sqlite:{}", db_path.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        let db = Self { pool };
        db.run_migrations().await?;
        Ok(db)
    }

    async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS providers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                provider_type TEXT NOT NULL,
                api_key_ref TEXT,
                enabled BOOLEAN DEFAULT 1,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS metrics (
                id TEXT PRIMARY KEY,
                provider_id TEXT NOT NULL,
                metric_type TEXT NOT NULL,
                value REAL NOT NULL,
                unit TEXT NOT NULL,
                timestamp TIMESTAMP NOT NULL,
                dimensions TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (provider_id) REFERENCES providers(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_metrics_provider_timestamp
            ON metrics(provider_id, timestamp DESC)
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS budgets (
                id TEXT PRIMARY KEY,
                provider_id TEXT NOT NULL,
                period TEXT NOT NULL,
                soft_limit REAL,
                hard_limit REAL,
                notes TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (provider_id) REFERENCES providers(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS alerts (
                id TEXT PRIMARY KEY,
                provider_id TEXT NOT NULL,
                rule_json TEXT NOT NULL,
                last_fired_at TIMESTAMP,
                status TEXT NOT NULL DEFAULT 'active',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (provider_id) REFERENCES providers(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                provider_id TEXT NOT NULL,
                timestamp TIMESTAMP NOT NULL,
                kind TEXT NOT NULL,
                payload TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (provider_id) REFERENCES providers(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_metric(&self, metric: &crate::models::Metric) -> Result<()> {
        let dimensions_json = serde_json::to_string(&metric.dimensions)?;

        sqlx::query(
            r#"
            INSERT INTO metrics (id, provider_id, metric_type, value, unit, timestamp, dimensions)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&metric.id)
        .bind(&metric.provider_id)
        .bind(format!("{:?}", metric.metric_type))
        .bind(metric.value)
        .bind(&metric.unit)
        .bind(metric.timestamp)
        .bind(dimensions_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_metrics(
        &self,
        provider_id: &str,
        hours: i64
    ) -> Result<Vec<crate::models::Metric>> {
        use chrono::{Duration, Utc};

        let since = Utc::now() - Duration::hours(hours);

        let rows = sqlx::query_as::<_, MetricRow>(
            r#"
            SELECT id, provider_id, metric_type, value, unit, timestamp, dimensions
            FROM metrics
            WHERE provider_id = ? AND timestamp > ?
            ORDER BY timestamp DESC
            "#,
        )
        .bind(provider_id)
        .bind(since)
        .fetch_all(&self.pool)
        .await?;

        let metrics = rows.into_iter()
            .map(|row| row.into_metric())
            .collect::<Result<Vec<_>>>()?;

        Ok(metrics)
    }

    pub async fn cleanup_old_metrics(&self, days: i64) -> Result<()> {
        use chrono::{Duration, Utc};

        let cutoff = Utc::now() - Duration::days(days);

        sqlx::query("DELETE FROM metrics WHERE timestamp < ?")
            .bind(cutoff)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct MetricRow {
    id: String,
    provider_id: String,
    metric_type: String,
    value: f64,
    unit: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    dimensions: Option<String>,
}

impl MetricRow {
    fn into_metric(self) -> Result<crate::models::Metric> {
        use crate::models::MetricType;
        use std::collections::HashMap;

        let metric_type = match self.metric_type.as_str() {
            "TokensIn" => MetricType::TokensIn,
            "TokensOut" => MetricType::TokensOut,
            "TokensCached" => MetricType::TokensCached,
            "CostUsd" => MetricType::CostUsd,
            "CreditsRemaining" => MetricType::CreditsRemaining,
            "Balance" => MetricType::Balance,
            _ => MetricType::CostUsd,
        };

        let dimensions = self.dimensions
            .map(|json| serde_json::from_str(&json))
            .transpose()?
            .unwrap_or_else(HashMap::new);

        Ok(crate::models::Metric {
            id: self.id,
            provider_id: self.provider_id,
            metric_type,
            value: self.value,
            unit: self.unit,
            timestamp: self.timestamp,
            dimensions,
        })
    }
}