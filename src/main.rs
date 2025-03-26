use leptos::prelude::*;

#[component]
fn ProgressBar(#[prop(default = 50)] max: u16, progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

// Create a generic component that accepts something that
// implements Fn() -> i32
#[component]
fn ProgressBarGen(
    #[prop(default = 50)] max: u16,
    progress: impl Fn() -> i32 + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn ProgressBarInto(
    #[prop(default = 10)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

//Optional
#[component]
fn ProgressBarOptional(
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32 + Send + Sync>>,
) -> impl IntoView {
    progress.map(|progress| {
        view! {
            <progress
                max=10
                value=progress
            />
        }
    })
}

/// Same component with docstring
/// Shows progress toward a goal
#[component]
fn ProgressBarDoc(
    /// the maximum value of the progress bar, default 100
    #[prop(default = 100)]
    max: u16,
    /// Current progress state
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

/// Components and Props Examples
#[component]
fn ComponentsAndProps() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    // Spreading Attributes into Components
    // Leptos allows for creating reusable spreading attributes
    let button_classes = view! {< {..} class="rounded-xl bg-red-500" />};
    let on_click_attribute = view! {< {..} on:click=move |_| *set_count.write() += 1 />};

    view! {
         <button
             on:click=move |_| *set_count.write() += 1
             class:red=move || count.get() % 2 == 1
             class="rounded-xl bg-blue-500"
         >
             <strong>"Click Me: "</strong>
             {count}
         </button>
         <button {..button_classes} {..on_click_attribute} >
             "No! Click Me: "
             {count}
         </button>
         <div class="container">
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
             // derived signal
             <progress
                 max="50"
                 value=double_count
             />
             <p>
                 "Double Count: "
                 {double_count}
             </p>
         </div>
         <div class="container">
             <p><strong>"Using a component"</strong></p>
             <ProgressBar progress=count/>
             <ProgressBar max=25 progress=count/>
             <ProgressBar max=10 progress=count/>
             <ProgressBarGen max=10 progress=double_count/>
             <ProgressBarInto progress=Signal::derive(double_count)/>
             <p>"Optional (no value)"</p>
             <ProgressBarOptional/>
             <ProgressBarDoc max=100 progress=count/>
         </div>
    }
}

/// Iterations
#[component]
fn Iterations() -> impl IntoView {
    // We can use vectors generate items that don't change
    let values = vec![0, 1, 2];
    view! {
        <p> {values.clone()} </p>
        <ul>
            {values.into_iter().map(|v| view!{ <li> {v} </li>}).collect_view()}
        </ul>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Components and Props"</h2>
        <ComponentsAndProps/>
        <h2>"Iterations"</h2>
        <Iterations/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
