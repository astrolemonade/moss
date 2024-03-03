// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::future::Future;

use sqlx::Sqlite;

use crate::runtime;

pub mod layout;
pub mod meta;
pub mod state;

#[derive(Debug, Clone)]
struct Pool(sqlx::Pool<Sqlite>);

impl Pool {
    fn new(pool: sqlx::Pool<Sqlite>) -> Self {
        Self(pool)
    }

    fn exec<F, T>(&self, f: impl FnOnce(sqlx::Pool<Sqlite>) -> F) -> T
    where
        F: Future<Output = T>,
    {
        runtime::block_on(async {
            let pool = self.0.clone();
            f(pool).await
        })
    }
}
