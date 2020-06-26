//! This module contains the implementation of a service
//! to show alerts and confirm dialogs in a browser.

use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use crate::utils;
    }
}

/// A dialog service.
#[derive(Default, Debug)]
pub struct DialogService {}

impl DialogService {
    /// Calls [alert](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    /// function.
    pub fn alert(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) alert(@{message}); },
            feature = "web_sys" => utils::window().alert_with_message(message).unwrap(),
        };
    }

    /// Calls [confirm](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    /// function.
    pub fn confirm(message: &str) -> bool {
        cfg_match! {
            feature = "std_web" => ({
                let value: Value = js! { return confirm(@{message}); };
                match value {
                    Value::Bool(result) => result,
                    _ => false,
                }
            }),
            feature = "web_sys" => utils::window().confirm_with_message(message).unwrap(),
        }
    }

    /// Prompts the user to input a message. In most browsers this will open an alert box with
    /// an input field where the user can input a message.
    ///
    /// It is possible to supply a default value which will be used if the user does not provide
    /// a value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    ///
    /// This method will `panic!` if there is an error in the process of trying to carry out this
    /// operation.
    ///
    /// Bear in mind that this function is blocking; no other code can be run on the thread while
    /// the user inputs their message.
    pub fn prompt(message: &str, default: Option<&str>) -> Option<String> {
        cfg_match! {
            feature = "std_web" => compile_error!("The `prompt` method is not supported for `stdweb`."),
            feature = "web_sys" => ({
                    if let Some(default) = default {
                        utils::window()
                               .prompt_with_message_and_default(message, default)
                               .expect("Couldn't read input.")
                    }
                    else {
                        utils::window()
                               .prompt_with_message(message)
                               .expect("Couldn't read input.")
                    }
            })
        }
    }
}
