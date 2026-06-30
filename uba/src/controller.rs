pub mod transaction_controller;

use slint::ComponentHandle;
use std::{cell::RefCell, rc::Rc};

use crate::AppWindow;
use transaction_controller::TransactionController;

pub struct MainController {
    app_window: AppWindow,

    balance_controller: Rc<RefCell<TransactionController>>,
}

impl MainController {
    pub fn new(logger: &mut uba_core::log::Logger<impl std::io::Write>) -> std::io::Result<Self> {
        let app_window = match AppWindow::new() {
            Ok(window) => window,
            Err(err) => return Err(std::io::Error::other(err)),
        };

        logger.log_msg("Window handle constructed")?;

        let balance_controller = TransactionController::new(logger)?;
        app_window.set_balance(format!("Balance: {}", balance_controller.get_balance()).into());

        let balance_controller = Rc::new(RefCell::new(balance_controller));

        Ok(Self {
            app_window,
            balance_controller,
        })
    }

    pub fn bind(&self) {
        let handle: slint::Weak<AppWindow> = self.app_window.as_weak();

        self.app_window.on_click({
            let controller_clone = Rc::clone(&self.balance_controller);
            move || {
                let new_bal = controller_clone.borrow_mut().increment_and_get_balance();
                if let Some(strong) = handle.upgrade() {
                    strong.set_balance(format!("Balance: {}", new_bal).into());
                }
            }
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.app_window.run()
    }
}
