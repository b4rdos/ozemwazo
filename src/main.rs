use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
            class:red=move || count.get() % 2 == 1
        >
            <strong>"Click Me: "</strong>
            {count}
        </button>
        <div class:container>
            <p>
                <strong>"Reactive: "</strong>
                {move || count.get()}
            </p>
            <p>
                <strong>"Reactive shorthand: "</strong>
                {count}
            </p>
            <p>
                <strong>"Not reactive: "</strong>
                {count.get()}
            </p>
        </div>
   }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
