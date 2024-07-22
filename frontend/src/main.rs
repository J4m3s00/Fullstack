use common::User;
use gloo_net::http::Request;
use yew::prelude::*;

fn fetch_users(out: UseStateHandle<Vec<User>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get("/api/v1/users").send().await.unwrap();
        let response = response.json::<Vec<User>>().await.unwrap();

        out.set(response);
    });
}

#[function_component]
fn App() -> Html {
    let users = use_state(|| vec![]);

    let add_user = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("/api/v1/user").send().await.unwrap();
            });
        })
    };

    let fetch_all_users = {
        let users = users.clone();
        Callback::from(move |_| {
            fetch_users(users.clone());
        })
    };

    {
        let users = users.clone();
        use_effect_with((), move |_| {
            fetch_users(users.clone());
            || ()
        });
    }

    html! {
        <div class={classes!("flex", "flex-col", "justify-center")}>
            <h1 class={classes!("text-center", "text-2xl", "font-bold")}>{ "Sign in" }</h1>
            <div class={classes!("mt-10")}>
                <div>
                    <label class={classes!("block", "text-sm", "font-medium", "text-gray-700")}>{ "Username" }</label>
                    <input type="email" autocomplete="email" class={classes!("block", "w-full", "rounded-md", "border-0", "py-1.5", "text-gray-900", "shadow-sm", "ring-1", "ring-inset", "ring-gray-300", "placeholder:text-gray-400", "focus:ring-2", "focus:ring-inset", "focus:ring-indigo-600", "sm:text-sm", "sm:leading-6")} />
                </div>
                <div>
                    <label class={classes!("block", "text-sm", "font-medium", "text-gray-700")}>{ "Password" }</label>
                    <input type="password" autocomplete="current-password" class={classes!("block", "w-full", "rounded-md", "border-0", "py-1.5", "text-gray-900", "shadow-sm", "ring-1", "ring-inset", "ring-gray-300", "placeholder:text-gray-400", "focus:ring-2", "focus:ring-inset", "focus:ring-indigo-600", "sm:text-sm", "sm:leading-6")} />
                </div>
            </div>
            <button onclick={add_user}>{ "Sign in" }</button>
            <button onclick={fetch_all_users}>{ "Fetch all users" }</button>
            <UserList users={(*users).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserListProps {
    users: Vec<User>,
}

#[function_component]
fn UserList(UserListProps { users }: &UserListProps) -> Html {
    users
        .iter()
        .map(|user| {
            html! {
                <div class={classes!("flex", "flex-col", "justify-center")}>
                    {&user.name}
                </div>
            }
        })
        .collect()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
