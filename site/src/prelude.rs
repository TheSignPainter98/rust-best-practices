# type Result<T> = std::result::Result<T, Error>;
#
# #[derive(Debug)]
# enum Error {
#     Invalid,
#     Unsupported,
#     MalformedEnvUrl {
#         env_var: &'static str,
#         source: Box<Error>,
#     },
#     NetworkUnavailable,
#     Unavailable,
#     Unknown(std::sync::Arc<dyn std::error::Error>),
# }
# impl<E: std::error::Error + 'static> From<E> for Error {
#     fn from(err: E) -> Self {
#         Self::Unknown(std::sync::Arc::new(err))
#     }
# }
#
# struct ArbitraryValue;
#
# impl ArbitraryValue {
#     fn is_valid(&self) -> bool {
#         true
#     }
#
#    fn something_else(self) -> Self {
#        self
#    }
#
#    fn another_thing(self) -> Self {
#        self
#    }
#
#    fn chained_with_something_else(self) -> Result<Self> {
#        Ok(self)
#    }
# }
#
# impl std::fmt::Display for ArbitraryValue {
#     fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
#         Ok(())
#     }
# }
#
# impl std::ops::Add for ArbitraryValue {
#     type Output = Self;
#     fn add(self, _rhs: Self) -> Self {
#         self
#     }
# }
#
# impl std::process::Termination for ArbitraryValue {
#     fn report(self) -> std::process::ExitCode {
#         std::process::ExitCode::SUCCESS
#     }
# }
#
# fn foo() -> ArbitraryValue {
#     ArbitraryValue
# }
#
# fn bar() -> ArbitraryValue {
#     ArbitraryValue
# }
#
# fn baz() -> ArbitraryValue {
#     ArbitraryValue
# }
#
# type SomeType = ArbitraryValue;
#
# struct Frobnicator;
#
# impl Frobnicator {
#     fn builder() -> Self { Self }
#     fn foo(self, _: &str) -> Self { self }
#     fn bar(self, _: &str) -> Self { self }
#     fn build(self) -> Result<Self> { Ok(self) }
# }
