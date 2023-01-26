pub struct SuperVillain {
    pub first_name: String,
    pub last_name: String,
}

pub trait MegaWeapon {
    fn shoot(&self) -> String;
}

impl SuperVillain {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn set_full_name(&mut self, full_name: String) {
        let full_name = full_name.split(" ").collect::<Vec<_>>();
        self.first_name = full_name[0].to_string();
        self.last_name = full_name[1].to_string();
    }

    pub fn attack(&self, weapon: impl MegaWeapon) {
        weapon.shoot();
    }

    pub async fn come_up_with_plan(&self) -> String {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        format!("{} {} is coming up with a plan {}", self.first_name, self.last_name, "To take over the world!")
    }
}


impl From<&str> for SuperVillain {
    // This is the right way to do this, instead of using the `set_full_name` method.
    fn from(name: &str) -> Self {
        let name = name.split(" ").collect::<Vec<_>>();

        SuperVillain {
            first_name: name[0].to_string(),
            last_name: name[1].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_context::{AsyncTestContext, test_context, TestContext};
    use super::*;

    // Constants.
    const FIRST_NAME: &str = "Ahmed";
    const LAST_NAME: &str = "Ahmed";
    const FULL_NAME: &str = "Ahmed Ahmed";

    struct Context {
        super_villain: SuperVillain,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            Context {
                super_villain: SuperVillain {
                    first_name: FIRST_NAME.to_string(),
                    last_name: LAST_NAME.to_string(),
                },
            }
        }
        /// Clean up after tests.
        fn teardown(self) {
            // Nothing to do here.
        }
    }

    // Async context, for testing async functions.
    struct AsyncContext {
        super_villain: SuperVillain,
    }

    #[async_trait::async_trait] // We need to apply the `async_trait` macro here, because we're using `async` functions.
    impl AsyncTestContext for AsyncContext {
        async fn setup() -> Self {
            AsyncContext {
                super_villain: SuperVillain {
                    first_name: FIRST_NAME.to_string(),
                    last_name: LAST_NAME.to_string(),
                },
            }
        }

        async fn teardown(self) {
            // Nothing to do here.
        }
    }

    fn run_test<F>(test: F)
        where
            F: FnOnce(&mut Context) -> () + std::panic::UnwindSafe,
    {
        // Set up the context.
        let mut context = Context::setup();
        // Wrap the context in an `AssertUnwindSafe` to ensure that the context
        let mut wrapper = std::panic::AssertUnwindSafe(&mut context);
        // Invoke the test, and catch any panics.
        let result = std::panic::catch_unwind(move || test(*wrapper));
        // Clean up after the test.
        context.teardown();

        // Check if the test panicked, and if so, resume unwinding.
        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }

    #[test_context(Context)] // This attribute will generate a function that will set up the context.
    #[test]
    fn test_full_name_with_spaces(ctx: &mut Context) {
        assert_eq!(ctx.super_villain.full_name(), FULL_NAME, "Unsuspected full name");
    }

    #[test_context(Context)]
    #[test]
    fn test_set_full_name(ctx: &mut Context) {
        ctx.super_villain.set_full_name(FULL_NAME.to_string());
        assert_eq!(ctx.super_villain.first_name, FIRST_NAME, "Unsuspected first name");
        assert_eq!(ctx.super_villain.last_name, LAST_NAME, "Unsuspected last name");
    }

    #[test_context(Context)]
    #[test]
    // We can use the `should_panic` attribute to test that a function panics.
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn test_set_full_name_with_empty_string(ctx: &mut Context) {
        ctx.super_villain.set_full_name("".to_string());
    }

    #[test]
    fn test_from_str() {
        run_test(|ctx| {
            let super_villain = SuperVillain::from(FULL_NAME);
            assert_eq!(super_villain.first_name, FIRST_NAME, "Unsuspected first name");
            assert_eq!(super_villain.last_name, LAST_NAME, "Unsuspected last name");
        })
    }

    // Async test.
    #[test_context(AsyncContext)]
    #[tokio::test] // We need to use the `tokio::test` attribute here, because we're using `tokio::time::sleep`.
    async fn plan_is_sadly_expected(ctx: &mut AsyncContext) {
        assert_eq!(ctx.super_villain.come_up_with_plan().await,
                   format!("{} {} is coming up with a plan {}", ctx.super_villain.first_name,
                           ctx.super_villain.last_name, "To take over the world!"))
    }
}
