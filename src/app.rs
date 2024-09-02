use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  let counter = use_state(|| 0);

  let handle_increment = {
    let counter = counter.clone();
    move |_| {
      let value = *counter + 1;
      counter.set(value);
    }
  };

  html! {
      <main class="container">
         <section class="mx-auto">
            <img src="https://yew.rs/img/logo.png" alt="Yew logo" width={200} height={200} class="mx-auto" />

            <h1>{ "Hello World!" }</h1>
            <button onclick={handle_increment}>
              {"Increment +1"}
            </button>

            <p>{ "You clicked me " }{ *counter }{ " times" }</p>
            <span>{ "from Yew with 💗" } </span>
          </section>
      </main>
  }
}
