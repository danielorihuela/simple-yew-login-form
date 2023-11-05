use web_sys::HtmlInputElement;
use yew::prelude::*;

use gloo_console::log;

#[function_component]
pub fn LoginForm() -> Html {
    let username = use_state(|| String::new());
    let on_change_username = {
        let username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let new_username = input.value();
            log!(new_username.clone());
            username.set(new_username);
        })
    };

    let password = use_state(|| String::new());
    let on_change_password = {
        let password = password.clone();

        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let new_password = input.value();
            log!(new_password.clone());
            password.set(new_password);
        })
    };

    let onclick = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |_| {
            log!(
                "Trying to login with username",
                (*username).as_str(),
                "and password",
                (*password).as_str()
            );
        })
    };

    html! {
        <div class="login-form">
            <legend text="Log in"></legend>
            <Username {on_change_username}/>
            <Password {on_change_password}/>

            <div class="submit">
                <button class="mui-btn mui-btn--raised" {onclick}>{"Submit"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct UsernameProperties {
    on_change_username: Callback<InputEvent>,
}

#[function_component]
pub fn Username(properties: &UsernameProperties) -> Html {
    html! {
        <div class="mui-textfield">
            <input type="text" placeholder="Username" oninput={properties.on_change_username.clone()}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PasswordProperties {
    on_change_password: Callback<InputEvent>,
}

#[function_component]
pub fn Password(properties: &PasswordProperties) -> Html {
    html! {
        <div class="mui-textfield">
            <input type="password" placeholder="Password" oninput={properties.on_change_password.clone()}/>
        </div>
    }
}
