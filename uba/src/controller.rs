use std::{cell::RefCell, rc::Rc};

use slint::ComponentHandle;
use uba_core::persistence::Config;

use crate::AppWindow;

pub struct MainController {
    app_window: AppWindow,

    balance_model: Rc<RefCell<crate::model::BalanceModel>>,

    /// Contains amalgamated data for _all_ models. Used to save all info to disk
    config: Rc<RefCell<Config>>,
}

impl MainController {
    pub fn new(config: Config) -> std::io::Result<Self> {
        let app_window = match AppWindow::new() {
            Ok(window) => window,
            Err(err) => return Err(std::io::Error::other(err)),
        };

        let mut balance_model = crate::model::BalanceModel::new();
        balance_model.load_config(&config);
        app_window.set_balance(format!("Balance: {}", balance_model.get_balance()).into());

        let balance_model = Rc::new(RefCell::new(balance_model));
        let config = Rc::new(RefCell::new(config));

        Ok(Self {
            app_window,
            balance_model,
            config,
        })
    }

    pub fn bind(&self) {
        let handle: slint::Weak<AppWindow> = self.app_window.as_weak();

        self.app_window.on_click({
            let model_clone = Rc::clone(&self.balance_model);
            let config_clone = Rc::clone(&self.config);
            move || {
                let new_bal = model_clone.borrow_mut().increment_and_get_balance();
                config_clone.borrow_mut().set_balance(new_bal);
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
