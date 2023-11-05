use yew::prelude::*;

use crate::components::login_form::LoginForm;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div class="logo-container">
                <img class="logo" src="images/cyberpunk_programmer_reading_a_book.jpg"/>
            </div>

            <LoginForm />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
