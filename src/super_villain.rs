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
    use test_context::{test_context, TestContext};
    use super::*;

    // Constants.
    const FIRST_NAME: &str = "Anas";
    const LAST_NAME: &str = "Elgarhy";
    const FULL_NAME: &str = "Anas Elgarhy";

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

    #[test_context(Context)]
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


    #[test]
    fn test_from_str() {
        run_test(|ctx| {
            let super_villain = SuperVillain::from(FULL_NAME);
            assert_eq!(super_villain.first_name, FIRST_NAME, "Unsuspected first name");
            assert_eq!(super_villain.last_name, LAST_NAME, "Unsuspected last name");
        })
    }
}
