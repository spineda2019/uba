use std::{cell::RefCell, rc::Rc};

use slint::ComponentHandle;
use uba_core::persistence::Config;

use crate::AppWindow;

pub struct MainController {
    app_window: AppWindow,

    balance_model: Rc<RefCell<crate::model::BalanceModel>>,
}

impl MainController {
    pub fn new() -> std::io::Result<Self> {
        let app_window = match AppWindow::new() {
            Ok(window) => window,
            Err(err) => return Err(std::io::Error::other(err)),
        };

        let balance_model = Rc::new(RefCell::new(crate::model::BalanceModel::new()));

        Ok(Self {
            app_window,
            balance_model,
        })
    }

    pub fn bind(&self) {
        let handle: slint::Weak<AppWindow> = self.app_window.as_weak();

        self.app_window.on_click({
            let model_clone = Rc::clone(&self.balance_model);
            move || {
                let new_bal = model_clone.borrow_mut().increment_and_get_balance();
                if let Some(strong) = handle.upgrade() {
                    strong.set_balance(format!("Balance: {}", new_bal).into());
                }
            }
        });
    }

    pub fn load_config(&mut self, config: &Config) {
        self.balance_model.borrow_mut().load_config(config);
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.app_window.run()
    }
}
