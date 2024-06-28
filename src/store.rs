use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use handle_errors::Error;
use tracing::Level;

use crate::types::account::{Account, AccountId};
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        tracing::warn!("{}", db_url);
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        Ok(Store {
            connection: db_pool,
        })
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn is_owner_question(
        &self,
        question_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "SELECT * FROM questions WHERE id = $1 AND account_id = $2",
        )
        .bind(question_id)
        .bind(account_id.0)
        .fetch_optional(&self.connection)
        .await
        {
            Ok(question) => Ok(question.is_some()),
            Err(e) => {
                tracing::event!(Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags, account_id) VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .bind(account_id.0)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions 
            SET title = $1, content = $2, tags = $3 
            WHERE id = $4 and account_id = $5 RETURNING id, title, content, tags")
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .bind(account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags")
        }).fetch_one(&self.connection).await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_question(
        &self,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "DELETE FROM quesitons WHERE id = $1 and account_id = $2",
        )
        .bind(question_id)
        .bind(account_id.0)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_answer(
        &self,
        new_answer: NewAnswer,
        account_id: AccountId,
    ) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INTO answers (content, corresponding_question, account_id) VALUES ($1, $2, $3)",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .bind(account_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = e.as_database_error().unwrap().code().unwrap().parse::<i32>().unwrap(),
                    db_message = e.as_database_error().unwrap().message(),
                    constraint = e.as_database_error().unwrap().constraint().unwrap());
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_account(
        &self,
        account: Account,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO accounts (email, password) VALUES ($1, $2)",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = e
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = e.as_database_error().unwrap().message(),
                    constraint = e
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn get_account(
        &self,
        email: String,
    ) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM accounts WHERE email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
