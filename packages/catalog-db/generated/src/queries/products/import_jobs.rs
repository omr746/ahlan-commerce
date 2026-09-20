// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct InsertImportJobParams<T1: crate::StringSql> {
    pub id: uuid::Uuid,
    pub input_path: T1,
    pub now: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Clone, Copy, Debug)]
pub struct MarkImportJobSucceededParams {
    pub now: chrono::DateTime<chrono::FixedOffset>,
    pub id: uuid::Uuid,
}
#[derive(Debug)]
pub struct MarkImportJobFailedParams<T1: crate::StringSql> {
    pub last_error: Option<T1>,
    pub now: chrono::DateTime<chrono::FixedOffset>,
    pub id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct RetryFailedImportJobParams {
    pub now: chrono::DateTime<chrono::FixedOffset>,
    pub id: uuid::Uuid,
}
#[derive(Clone, Copy, Debug)]
pub struct ReapStaleRunningJobsParams {
    pub now: chrono::DateTime<chrono::FixedOffset>,
    pub stale_before: chrono::DateTime<chrono::FixedOffset>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct InsertImportJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct InsertImportJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<InsertImportJobBorrowed<'a>> for InsertImportJob {
    fn from(
        InsertImportJobBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: InsertImportJobBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            input_path: input_path.into(),
            attempts,
            last_error: last_error.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetImportJobById {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetImportJobByIdBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetImportJobByIdBorrowed<'a>> for GetImportJobById {
    fn from(
        GetImportJobByIdBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: GetImportJobByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            input_path: input_path.into(),
            attempts,
            last_error: last_error.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListImportJobsByStatus {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ListImportJobsByStatusBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ListImportJobsByStatusBorrowed<'a>> for ListImportJobsByStatus {
    fn from(
        ListImportJobsByStatusBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: ListImportJobsByStatusBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            input_path: input_path.into(),
            attempts,
            last_error: last_error.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimNextQueuedJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct ClaimNextQueuedJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<ClaimNextQueuedJobBorrowed<'a>> for ClaimNextQueuedJob {
    fn from(
        ClaimNextQueuedJobBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: ClaimNextQueuedJobBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            input_path: input_path.into(),
            attempts,
            last_error: last_error.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct MarkImportJobSucceeded {
    pub id: uuid::Uuid,
    pub status: String,
}
pub struct MarkImportJobSucceededBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
}
impl<'a> From<MarkImportJobSucceededBorrowed<'a>> for MarkImportJobSucceeded {
    fn from(
        MarkImportJobSucceededBorrowed { id, status }: MarkImportJobSucceededBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct MarkImportJobFailed {
    pub id: uuid::Uuid,
    pub status: String,
}
pub struct MarkImportJobFailedBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
}
impl<'a> From<MarkImportJobFailedBorrowed<'a>> for MarkImportJobFailed {
    fn from(MarkImportJobFailedBorrowed { id, status }: MarkImportJobFailedBorrowed<'a>) -> Self {
        Self {
            id,
            status: status.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct RetryFailedImportJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub attempts: i32,
}
pub struct RetryFailedImportJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub attempts: i32,
}
impl<'a> From<RetryFailedImportJobBorrowed<'a>> for RetryFailedImportJob {
    fn from(
        RetryFailedImportJobBorrowed {
            id,
            status,
            attempts,
        }: RetryFailedImportJobBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            attempts,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReapStaleRunningJobs {
    pub id: uuid::Uuid,
    pub status: String,
    pub attempts: i32,
}
pub struct ReapStaleRunningJobsBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub attempts: i32,
}
impl<'a> From<ReapStaleRunningJobsBorrowed<'a>> for ReapStaleRunningJobs {
    fn from(
        ReapStaleRunningJobsBorrowed {
            id,
            status,
            attempts,
        }: ReapStaleRunningJobsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            attempts,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct InsertImportJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<InsertImportJobBorrowed, tokio_postgres::Error>,
    mapper: fn(InsertImportJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> InsertImportJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(InsertImportJobBorrowed) -> R,
    ) -> InsertImportJobQuery<'c, 'a, 's, C, R, N> {
        InsertImportJobQuery {
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
pub struct GetImportJobByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetImportJobByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(GetImportJobByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetImportJobByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetImportJobByIdBorrowed) -> R,
    ) -> GetImportJobByIdQuery<'c, 'a, 's, C, R, N> {
        GetImportJobByIdQuery {
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
pub struct ListImportJobsByStatusQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ListImportJobsByStatusBorrowed, tokio_postgres::Error>,
    mapper: fn(ListImportJobsByStatusBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListImportJobsByStatusQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ListImportJobsByStatusBorrowed) -> R,
    ) -> ListImportJobsByStatusQuery<'c, 'a, 's, C, R, N> {
        ListImportJobsByStatusQuery {
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
pub struct ClaimNextQueuedJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ClaimNextQueuedJobBorrowed, tokio_postgres::Error>,
    mapper: fn(ClaimNextQueuedJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ClaimNextQueuedJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ClaimNextQueuedJobBorrowed) -> R,
    ) -> ClaimNextQueuedJobQuery<'c, 'a, 's, C, R, N> {
        ClaimNextQueuedJobQuery {
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
pub struct MarkImportJobSucceededQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<MarkImportJobSucceededBorrowed, tokio_postgres::Error>,
    mapper: fn(MarkImportJobSucceededBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MarkImportJobSucceededQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(MarkImportJobSucceededBorrowed) -> R,
    ) -> MarkImportJobSucceededQuery<'c, 'a, 's, C, R, N> {
        MarkImportJobSucceededQuery {
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
pub struct MarkImportJobFailedQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<MarkImportJobFailedBorrowed, tokio_postgres::Error>,
    mapper: fn(MarkImportJobFailedBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MarkImportJobFailedQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(MarkImportJobFailedBorrowed) -> R,
    ) -> MarkImportJobFailedQuery<'c, 'a, 's, C, R, N> {
        MarkImportJobFailedQuery {
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
pub struct RetryFailedImportJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<RetryFailedImportJobBorrowed, tokio_postgres::Error>,
    mapper: fn(RetryFailedImportJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> RetryFailedImportJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(RetryFailedImportJobBorrowed) -> R,
    ) -> RetryFailedImportJobQuery<'c, 'a, 's, C, R, N> {
        RetryFailedImportJobQuery {
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
pub struct ReapStaleRunningJobsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ReapStaleRunningJobsBorrowed, tokio_postgres::Error>,
    mapper: fn(ReapStaleRunningJobsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ReapStaleRunningJobsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ReapStaleRunningJobsBorrowed) -> R,
    ) -> ReapStaleRunningJobsQuery<'c, 'a, 's, C, R, N> {
        ReapStaleRunningJobsQuery {
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
pub struct InsertImportJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_import_job() -> InsertImportJobStmt {
    InsertImportJobStmt(
        "INSERT INTO import_jobs ( id, status, input_path, attempts, created_at, updated_at ) VALUES ( $1, 'queued', $2, 0, $3, $3 ) RETURNING id, status, input_path, attempts, last_error, created_at, updated_at",
        None,
    )
}
impl InsertImportJobStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
        input_path: &'a T1,
        now: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> InsertImportJobQuery<'c, 'a, 's, C, InsertImportJob, 3> {
        InsertImportJobQuery {
            client,
            params: [id, input_path, now],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<InsertImportJobBorrowed, tokio_postgres::Error> {
                Ok(InsertImportJobBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| InsertImportJob::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        InsertImportJobParams<T1>,
        InsertImportJobQuery<'c, 'a, 's, C, InsertImportJob, 3>,
        C,
    > for InsertImportJobStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a InsertImportJobParams<T1>,
    ) -> InsertImportJobQuery<'c, 'a, 's, C, InsertImportJob, 3> {
        self.bind(client, &params.id, &params.input_path, &params.now)
    }
}
pub struct GetImportJobByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_import_job_by_id() -> GetImportJobByIdStmt {
    GetImportJobByIdStmt(
        "SELECT id, status, input_path, attempts, last_error, created_at, updated_at FROM import_jobs WHERE id = $1",
        None,
    )
}
impl GetImportJobByIdStmt {
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
        id: &'a uuid::Uuid,
    ) -> GetImportJobByIdQuery<'c, 'a, 's, C, GetImportJobById, 1> {
        GetImportJobByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetImportJobByIdBorrowed, tokio_postgres::Error> {
                Ok(GetImportJobByIdBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| GetImportJobById::from(it),
        }
    }
}
pub struct ListImportJobsByStatusStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_import_jobs_by_status() -> ListImportJobsByStatusStmt {
    ListImportJobsByStatusStmt(
        "SELECT id, status, input_path, attempts, last_error, created_at, updated_at FROM import_jobs WHERE status = $1 ORDER BY updated_at DESC",
        None,
    )
}
impl ListImportJobsByStatusStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        status: &'a T1,
    ) -> ListImportJobsByStatusQuery<'c, 'a, 's, C, ListImportJobsByStatus, 1> {
        ListImportJobsByStatusQuery {
            client,
            params: [status],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ListImportJobsByStatusBorrowed, tokio_postgres::Error> {
                Ok(ListImportJobsByStatusBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| ListImportJobsByStatus::from(it),
        }
    }
}
pub struct ClaimNextQueuedJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn claim_next_queued_job() -> ClaimNextQueuedJobStmt {
    ClaimNextQueuedJobStmt(
        "UPDATE import_jobs SET status = 'running', attempts = attempts + 1, updated_at = $1 WHERE id = ( SELECT id FROM import_jobs WHERE status = 'queued' AND attempts < 3 ORDER BY created_at LIMIT 1 FOR UPDATE SKIP LOCKED ) RETURNING id, status, input_path, attempts, last_error, created_at, updated_at",
        None,
    )
}
impl ClaimNextQueuedJobStmt {
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
        now: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> ClaimNextQueuedJobQuery<'c, 'a, 's, C, ClaimNextQueuedJob, 1> {
        ClaimNextQueuedJobQuery {
            client,
            params: [now],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ClaimNextQueuedJobBorrowed, tokio_postgres::Error> {
                Ok(ClaimNextQueuedJobBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| ClaimNextQueuedJob::from(it),
        }
    }
}
pub struct MarkImportJobSucceededStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn mark_import_job_succeeded() -> MarkImportJobSucceededStmt {
    MarkImportJobSucceededStmt(
        "UPDATE import_jobs SET status = 'succeeded', last_error = NULL, updated_at = $1 WHERE id = $2 AND status = 'running' RETURNING id, status",
        None,
    )
}
impl MarkImportJobSucceededStmt {
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
        now: &'a chrono::DateTime<chrono::FixedOffset>,
        id: &'a uuid::Uuid,
    ) -> MarkImportJobSucceededQuery<'c, 'a, 's, C, MarkImportJobSucceeded, 2> {
        MarkImportJobSucceededQuery {
            client,
            params: [now, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<MarkImportJobSucceededBorrowed, tokio_postgres::Error> {
                Ok(MarkImportJobSucceededBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                })
            },
            mapper: |it| MarkImportJobSucceeded::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        MarkImportJobSucceededParams,
        MarkImportJobSucceededQuery<'c, 'a, 's, C, MarkImportJobSucceeded, 2>,
        C,
    > for MarkImportJobSucceededStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a MarkImportJobSucceededParams,
    ) -> MarkImportJobSucceededQuery<'c, 'a, 's, C, MarkImportJobSucceeded, 2> {
        self.bind(client, &params.now, &params.id)
    }
}
pub struct MarkImportJobFailedStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn mark_import_job_failed() -> MarkImportJobFailedStmt {
    MarkImportJobFailedStmt(
        "UPDATE import_jobs SET status = 'failed', last_error = $1, updated_at = $2 WHERE id = $3 AND status = 'running' RETURNING id, status",
        None,
    )
}
impl MarkImportJobFailedStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        last_error: &'a Option<T1>,
        now: &'a chrono::DateTime<chrono::FixedOffset>,
        id: &'a uuid::Uuid,
    ) -> MarkImportJobFailedQuery<'c, 'a, 's, C, MarkImportJobFailed, 3> {
        MarkImportJobFailedQuery {
            client,
            params: [last_error, now, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<MarkImportJobFailedBorrowed, tokio_postgres::Error> {
                Ok(MarkImportJobFailedBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                })
            },
            mapper: |it| MarkImportJobFailed::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        MarkImportJobFailedParams<T1>,
        MarkImportJobFailedQuery<'c, 'a, 's, C, MarkImportJobFailed, 3>,
        C,
    > for MarkImportJobFailedStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a MarkImportJobFailedParams<T1>,
    ) -> MarkImportJobFailedQuery<'c, 'a, 's, C, MarkImportJobFailed, 3> {
        self.bind(client, &params.last_error, &params.now, &params.id)
    }
}
pub struct RetryFailedImportJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn retry_failed_import_job() -> RetryFailedImportJobStmt {
    RetryFailedImportJobStmt(
        "UPDATE import_jobs SET status = 'queued', updated_at = $1 WHERE id = $2 AND status = 'failed' AND attempts < 3 RETURNING id, status, attempts",
        None,
    )
}
impl RetryFailedImportJobStmt {
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
        now: &'a chrono::DateTime<chrono::FixedOffset>,
        id: &'a uuid::Uuid,
    ) -> RetryFailedImportJobQuery<'c, 'a, 's, C, RetryFailedImportJob, 2> {
        RetryFailedImportJobQuery {
            client,
            params: [now, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<RetryFailedImportJobBorrowed, tokio_postgres::Error> {
                Ok(RetryFailedImportJobBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    attempts: row.try_get(2)?,
                })
            },
            mapper: |it| RetryFailedImportJob::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        RetryFailedImportJobParams,
        RetryFailedImportJobQuery<'c, 'a, 's, C, RetryFailedImportJob, 2>,
        C,
    > for RetryFailedImportJobStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a RetryFailedImportJobParams,
    ) -> RetryFailedImportJobQuery<'c, 'a, 's, C, RetryFailedImportJob, 2> {
        self.bind(client, &params.now, &params.id)
    }
}
pub struct ReapStaleRunningJobsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn reap_stale_running_jobs() -> ReapStaleRunningJobsStmt {
    ReapStaleRunningJobsStmt(
        "UPDATE import_jobs SET status = 'failed', last_error = 'worker crashed or timed out while running', updated_at = $1 WHERE status = 'running' AND updated_at < $2 RETURNING id, status, attempts",
        None,
    )
}
impl ReapStaleRunningJobsStmt {
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
        now: &'a chrono::DateTime<chrono::FixedOffset>,
        stale_before: &'a chrono::DateTime<chrono::FixedOffset>,
    ) -> ReapStaleRunningJobsQuery<'c, 'a, 's, C, ReapStaleRunningJobs, 2> {
        ReapStaleRunningJobsQuery {
            client,
            params: [now, stale_before],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ReapStaleRunningJobsBorrowed, tokio_postgres::Error> {
                Ok(ReapStaleRunningJobsBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    attempts: row.try_get(2)?,
                })
            },
            mapper: |it| ReapStaleRunningJobs::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ReapStaleRunningJobsParams,
        ReapStaleRunningJobsQuery<'c, 'a, 's, C, ReapStaleRunningJobs, 2>,
        C,
    > for ReapStaleRunningJobsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ReapStaleRunningJobsParams,
    ) -> ReapStaleRunningJobsQuery<'c, 'a, 's, C, ReapStaleRunningJobs, 2> {
        self.bind(client, &params.now, &params.stale_before)
    }
}
