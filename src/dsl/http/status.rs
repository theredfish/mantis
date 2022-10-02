//! The `http::status` DSL provides built-in functions to perform declarative
//! assertions against the status of an http response.
use crate::{
    assertion::{
        traits::{IsEq, IsNe, RangeInclusive},
        Assertion,
    },
    dsl::{is_between, Expression, Predicate, Range},
    grillon::LogSettings,
    StatusCode,
};
use std::fmt::Debug;

/// A short-hand function to test if the status code
/// of the response is in the range of 2xx codes.
pub fn is_success() -> Expression<Range<u16>> {
    is_between(200, 299)
}

/// A short-hand function to test if the status code
/// of the response is in the range of 4xx codes.
pub fn is_client_error() -> Expression<Range<u16>> {
    is_between(400, 499)
}

/// A short-hand function to test if the status code
/// of the response is in the range of 5xx codes.
pub fn is_server_error() -> Expression<Range<u16>> {
    is_between(500, 599)
}

/// Http status DSL to assert the status code of a response.
///
/// ```rust
/// use grillon::{Result, Grillon, StatusCode};
/// use grillon::dsl::{is, is_between, is_not, http::is_success};
///
/// #[tokio::test]
/// async fn check_status() -> Result<()> {
///    Grillon::new("http://jsonplaceholder.typicode.com")?
///        .get("users/1")
///        .assert()
///        .await
///        .status(is(200))
///        .status(is(StatusCode::OK))
///        .status(is_not(500))
///        .status(is_not(StatusCode::INTERNAL_SERVER_ERROR))
///        .status(is_success())
///        .status(is_between(200, 204))
///        .status(is_between(StatusCode::OK, StatusCode::NO_CONTENT));
///
///    Ok(())
/// }
pub trait StatusCodeDsl<T> {
    /// The assertion type resulting from the evaluation.
    type Assertion;

    /// Evaluates the status assertion to run depending on the [`Predicate`].
    /// The test results will be produced on the given output configured via the
    /// [`LogSettings`].
    fn eval(self, actual: T, predicate: Predicate, log_settings: &LogSettings) -> Self::Assertion;
}

impl StatusCodeDsl<StatusCode> for StatusCode {
    type Assertion = Assertion<u16>;

    fn eval(
        self,
        actual: StatusCode,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<u16> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

impl StatusCodeDsl<StatusCode> for u16 {
    type Assertion = Assertion<u16>;

    fn eval(
        self,
        actual: StatusCode,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<u16> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

impl StatusCodeDsl<StatusCode> for Range<StatusCode> {
    type Assertion = Assertion<u16>;

    fn eval(
        self,
        actual: StatusCode,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<u16> {
        match predicate {
            Predicate::Between => self.is_between(actual).assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

impl StatusCodeDsl<StatusCode> for Range<u16> {
    type Assertion = Assertion<u16>;

    fn eval(
        self,
        actual: StatusCode,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<u16> {
        match predicate {
            Predicate::Between => self.is_between(actual).assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

/// Http status DSL to assert the status code equality of a response.
pub trait StatusCodeDslEquality<T>: StatusCodeDsl<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    /// Builds an assertion comparing the equality between two status codes.
    fn is(self, actual: T) -> Self::Assertion;
    /// Builds an assertion comparing the non equality between two status codes.
    fn is_not(self, actual: T) -> Self::Assertion;
}

impl StatusCodeDslEquality<StatusCode> for StatusCode {
    fn is(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Self::Assertion {
        actual.is_ne(self)
    }
}

impl StatusCodeDslEquality<StatusCode> for u16 {
    fn is(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Self::Assertion {
        actual.is_ne(self)
    }
}

/// Http status DSL to assert the status code of a response is in
/// the given inclusive range.
pub trait StatusCodeDslBetween<T>: StatusCodeDsl<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    /// Builds an assertion to check if a status code is within an inclusive
    /// range.
    fn is_between(self, actual: T) -> Self::Assertion;
}

impl StatusCodeDslBetween<StatusCode> for Range<StatusCode> {
    fn is_between(self, actual: StatusCode) -> Self::Assertion {
        actual.in_range(self.left, self.right)
    }
}

impl StatusCodeDslBetween<StatusCode> for Range<u16> {
    fn is_between(self, actual: StatusCode) -> Self::Assertion {
        actual.in_range(self.left, self.right)
    }
}