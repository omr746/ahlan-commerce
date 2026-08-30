// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct TestNull {
    pub description: String,
    pub published_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct TestNullBorrowed<'a> {
    pub description: &'a str,
    pub published_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<TestNullBorrowed<'a>> for TestNull {
    fn from(
        TestNullBorrowed {
            description,
            published_at,
        }: TestNullBorrowed<'a>,
    ) -> Self {
        Self {
            description: description.into(),
            published_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct TestNullQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<TestNullBorrowed, tokio_postgres::Error>,
    mapper: fn(TestNullBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> TestNullQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(TestNullBorrowed) -> R) -> TestNullQuery<'c, 'a, 's, C, R, N> {
        TestNullQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct TestNullStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn test_null() -> TestNullStmt {
    TestNullStmt(
        "SELECT NULL::text AS description, NULL::timestamptz AS published_at",
        None,
    )
}
impl TestNullStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> TestNullQuery<'c, 'a, 's, C, TestNull, 0> {
        TestNullQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<TestNullBorrowed, tokio_postgres::Error> {
                    Ok(TestNullBorrowed {
                        description: row.try_get(0)?,
                        published_at: row.try_get(1)?,
                    })
                },
            mapper: |it| TestNull::from(it),
        }
    }
}
