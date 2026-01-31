//! Модуль UI компонентов

use yew::prelude::*;

/// Простой компонент для теста
#[function_component]
pub fn TestComponent() -> Html {
    html! {
        <div>
            <h2>{ "Тестовый компонент" }</h2>
            <p>{ "UI компоненты в разработке..." }</p>
        </div>
    }
}
